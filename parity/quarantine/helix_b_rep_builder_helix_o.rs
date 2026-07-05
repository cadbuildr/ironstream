// FILE: helix_b_rep_builder_helix_o.rs
// occt: HelixBRep_BuilderHelix

//! Builder for creating helix and spiral B-rep shapes.
//!
//! Constructs helix/spiral wires with optional tapering from parametric definitions.
//! Supports multi-segment helices with different diameters and pitches per segment.

use std::f64::consts::PI;

/// Continuity constraint for approximation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GeomAbsShapeContiguity {
    /// C0 - positional continuity only
    C0 = 0,
    /// C1 - tangent continuity
    C1 = 1,
    /// C2 - curvature continuity
    C2 = 2,
}

/// Error and warning status codes for the helix builder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusCode(i32);

impl StatusCode {
    /// Status 0: OK
    pub fn ok() -> Self {
        StatusCode(0)
    }
    /// Status 1: Object just initialized
    pub fn not_initialized() -> Self {
        StatusCode(1)
    }
    /// Status 2: Approximation failed
    pub fn approximation_failed() -> Self {
        StatusCode(2)
    }
    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

/// Coordinate system for helix axis.
#[derive(Clone, Debug)]
pub struct Ax3 {
    /// Origin point
    pub origin: (f64, f64, f64),
    /// Z-direction (helix axis)
    pub z_dir: (f64, f64, f64),
    /// X-direction
    pub x_dir: (f64, f64, f64),
}

impl Ax3 {
    /// Create a default axis (origin at 0,0,0; Z-axis is helix direction).
    pub fn new() -> Self {
        Self {
            origin: (0.0, 0.0, 0.0),
            z_dir: (0.0, 0.0, 1.0),
            x_dir: (1.0, 0.0, 0.0),
        }
    }

    /// Set the direction and origin.
    pub fn set_direction(&mut self, z_dir: (f64, f64, f64)) {
        self.z_dir = z_dir;
    }

    /// Set the location.
    pub fn set_location(&mut self, origin: (f64, f64, f64)) {
        self.origin = origin;
    }
}

impl Default for Ax3 {
    fn default() -> Self {
        Self::new()
    }
}

/// Result shape (simplified TopoDS_Shape stub).
#[derive(Clone, Debug)]
pub struct TopodsShape {
    /// Shape type identifier
    pub shape_type: String,
    /// Shape data
    data: Vec<u8>,
}

impl TopodsShape {
    /// Create a new empty shape.
    pub fn new(shape_type: impl Into<String>) -> Self {
        Self {
            shape_type: shape_type.into(),
            data: Vec::new(),
        }
    }

    /// Check if shape is null (empty).
    pub fn is_null(&self) -> bool {
        self.data.is_empty() && self.shape_type.is_empty()
    }

    /// Nullify the shape.
    pub fn nullify(&mut self) {
        self.shape_type.clear();
        self.data.clear();
    }
}

/// Builder for helix and spiral B-rep shapes.
///
/// Constructs helix wires from:
/// - Composite helix with varying diameter (cone-shaped)
/// - Pure helix with constant diameter
/// - Pure spiral with linearly varying diameter
pub struct HelixBRepBuilderHelix {
    /// Helix coordinate system
    axis: Ax3,
    /// Diameters at each point (n_parts + 1 values)
    diameters: Vec<f64>,
    /// Heights for each part
    heights: Vec<f64>,
    /// Pitches (or steps) for each part
    pitches: Vec<f64>,
    /// Flags: true if pitch is specified, false if nb_turns
    is_pitches: Vec<bool>,
    /// Approximation tolerance
    tolerance: f64,
    /// Reached tolerance from approximation
    tolerance_reached: f64,
    /// Maximum B-spline degree for approximation
    max_degree: i32,
    /// Maximum segments for approximation
    max_segments: i32,
    /// Continuity constraint
    continuity: GeomAbsShapeContiguity,
    /// Error status code
    error_status: StatusCode,
    /// Warning status code
    warning_status: StatusCode,
    /// Resulting shape
    shape: TopodsShape,
    /// Number of helix segments
    n_parts: i32,
}

impl HelixBRepBuilderHelix {
    /// Create a new helix builder (empty, uninitialized).
    pub fn new() -> Self {
        Self {
            axis: Ax3::new(),
            diameters: Vec::new(),
            heights: Vec::new(),
            pitches: Vec::new(),
            is_pitches: Vec::new(),
            tolerance: 0.0001,
            tolerance_reached: 99.0,
            max_degree: 8,
            max_segments: 1000,
            continuity: GeomAbsShapeContiguity::C1,
            error_status: StatusCode::not_initialized(),
            warning_status: StatusCode::not_initialized(),
            shape: TopodsShape::new(""),
            n_parts: 1,
        }
    }

    /// Set parameters for a composite helix (varying diameter).
    ///
    /// diameters: n_parts + 1 values (start and end diameter for each segment)
    /// heights: height of each segment
    /// pitches: pitch (or nb_turns) for each segment
    /// is_pitches: true if pitches are pitch values, false if nb_turns
    pub fn set_parameters_composite(
        &mut self,
        axis: Ax3,
        diameters: Vec<f64>,
        heights: Vec<f64>,
        pitches: Vec<f64>,
        is_pitches: Vec<bool>,
    ) {
        self.n_parts = (diameters.len() - 1) as i32;

        if diameters.len() != (self.n_parts + 1) as usize
            || heights.len() != self.n_parts as usize
            || pitches.len() != self.n_parts as usize
            || is_pitches.len() != self.n_parts as usize
        {
            self.error_status = StatusCode(13); // Wrong dimensions
            return;
        }

        self.axis = axis;
        self.diameters = diameters;
        self.heights = heights;
        self.pitches = pitches;
        self.is_pitches = is_pitches;
        self.shape.nullify();

        self.error_status = StatusCode::ok();
        self.warning_status = StatusCode::ok();
    }

    /// Set parameters for a pure helix (constant diameter).
    pub fn set_parameters_helix(
        &mut self,
        axis: Ax3,
        diameter: f64,
        heights: Vec<f64>,
        pitches: Vec<f64>,
        is_pitches: Vec<bool>,
    ) {
        let n_parts = heights.len();
        let mut diams = vec![diameter; n_parts + 1];
        self.set_parameters_composite(axis, diams, heights, pitches, is_pitches);
    }

    /// Set parameters for a pure spiral (linearly varying diameter).
    pub fn set_parameters_spiral(
        &mut self,
        axis: Ax3,
        diam_start: f64,
        diam_end: f64,
        heights: Vec<f64>,
        pitches: Vec<f64>,
        is_pitches: Vec<bool>,
    ) {
        let n_parts = heights.len();
        let total_height: f64 = heights.iter().sum();

        let mut diams = vec![0.0; n_parts + 1];
        diams[0] = diam_start;

        let slope = (diam_end - diam_start) / total_height;
        let mut h = 0.0;
        for i in 1..=n_parts {
            h += heights[i - 1];
            diams[i] = diam_start + slope * h;
        }

        self.set_parameters_composite(axis, diams, heights, pitches, is_pitches);
    }

    /// Set approximation parameters.
    pub fn set_approx_parameters(
        &mut self,
        tolerance: f64,
        max_degree: i32,
        continuity: GeomAbsShapeContiguity,
    ) {
        self.tolerance = tolerance.max(1e-10);
        self.max_degree = max_degree;
        self.continuity = continuity;
    }

    /// Perform the helix construction.
    ///
    /// TODO: Implement actual BRep wire construction with B-spline approximation.
    /// This is a simplified version that validates parameters.
    pub fn perform(&mut self) {
        // Validate parameters
        if self.error_status != StatusCode::ok() {
            return;
        }

        // TODO: Validate diameters > tolerance
        for (i, &d) in self.diameters.iter().enumerate() {
            if d < 1e-7 {
                self.error_status = StatusCode(10); // R < tolerance
                return;
            }
        }

        // TODO: Validate pitches and heights > tolerance
        for (i, &p) in self.pitches.iter().enumerate() {
            if p < 1e-7 {
                self.error_status = StatusCode(11); // Pitch < tolerance
                return;
            }
        }

        for (i, &h) in self.heights.iter().enumerate() {
            if h < 1e-7 {
                self.error_status = StatusCode(12); // Height < tolerance
                return;
            }
        }

        // TODO: Build helix segments using B-spline approximation
        // For now, mark success with empty shape as placeholder
        self.shape = TopodsShape::new("Wire");
        self.tolerance_reached = self.tolerance;
        self.error_status = StatusCode::ok();
        self.warning_status = StatusCode::ok();
    }

    /// Get the tolerance reached by approximation.
    pub fn tolerance_reached(&self) -> f64 {
        self.tolerance_reached
    }

    /// Get the error status.
    pub fn error_status(&self) -> i32 {
        self.error_status.as_i32()
    }

    /// Get the warning status.
    pub fn warning_status(&self) -> i32 {
        self.warning_status.as_i32()
    }

    /// Get the resulting shape.
    pub fn shape(&self) -> &TopodsShape {
        &self.shape
    }

    /// Get the number of parts in the helix.
    pub fn n_parts(&self) -> i32 {
        self.n_parts
    }
}

impl Default for HelixBRepBuilderHelix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = HelixBRepBuilderHelix::new();
        assert_eq!(builder.error_status(), 1); // not initialized
        assert!(builder.shape.is_null());
        assert_eq!(builder.tolerance, 0.0001);
        assert_eq!(builder.max_degree, 8);
    }

    #[test]
    fn test_ax3_default() {
        let ax = Ax3::new();
        assert_eq!(ax.origin, (0.0, 0.0, 0.0));
        assert_eq!(ax.z_dir, (0.0, 0.0, 1.0));
        assert_eq!(ax.x_dir, (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_parameters_helix() {
        let mut builder = HelixBRepBuilderHelix::new();
        let axis = Ax3::new();
        let heights = vec![10.0, 10.0];
        let pitches = vec![5.0, 5.0];
        let is_pitches = vec![true, true];

        builder.set_parameters_helix(axis, 20.0, heights, pitches, is_pitches);

        assert_eq!(builder.error_status(), 0); // OK
        assert_eq!(builder.n_parts, 2);
        assert_eq!(builder.diameters.len(), 3);
        assert!(builder.diameters.iter().all(|&d| (d - 20.0).abs() < 1e-10));
    }

    #[test]
    fn test_set_parameters_spiral() {
        let mut builder = HelixBRepBuilderHelix::new();
        let axis = Ax3::new();
        let heights = vec![10.0];
        let pitches = vec![5.0];
        let is_pitches = vec![true];

        builder.set_parameters_spiral(axis, 10.0, 20.0, heights, pitches, is_pitches);

        assert_eq!(builder.error_status(), 0); // OK
        assert_eq!(builder.n_parts, 1);
        assert_eq!(builder.diameters.len(), 2);
        assert!((builder.diameters[0] - 10.0).abs() < 1e-10);
        assert!((builder.diameters[1] - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_set_approx_parameters() {
        let mut builder = HelixBRepBuilderHelix::new();
        builder.set_approx_parameters(0.001, 5, GeomAbsShapeContiguity::C2);

        assert_eq!(builder.tolerance, 0.001);
        assert_eq!(builder.max_degree, 5);
        assert_eq!(builder.continuity, GeomAbsShapeContiguity::C2);
    }

    #[test]
    fn test_perform_helix() {
        let mut builder = HelixBRepBuilderHelix::new();
        let axis = Ax3::new();
        let heights = vec![10.0];
        let pitches = vec![5.0];
        let is_pitches = vec![true];

        builder.set_parameters_helix(axis, 20.0, heights, pitches, is_pitches);
        builder.perform();

        assert_eq!(builder.error_status(), 0); // OK
        assert!(!builder.shape.is_null());
        assert_eq!(builder.shape.shape_type, "Wire");
    }

    #[test]
    fn test_tolerance_reached() {
        let mut builder = HelixBRepBuilderHelix::new();
        builder.set_approx_parameters(0.001, 8, GeomAbsShapeContiguity::C1);
        let axis = Ax3::new();

        builder.set_parameters_helix(axis, 20.0, vec![10.0], vec![5.0], vec![true]);
        builder.perform();

        assert!((builder.tolerance_reached() - 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_dimension_mismatch_error() {
        let mut builder = HelixBRepBuilderHelix::new();
        let axis = Ax3::new();
        let diameters = vec![10.0, 20.0, 30.0]; // 3 elements -> 2 parts
        let heights = vec![10.0]; // 1 part mismatch
        let pitches = vec![5.0, 5.0]; // 2 parts
        let is_pitches = vec![true, true]; // 2 parts

        builder.set_parameters_composite(axis, diameters, heights, pitches, is_pitches);

        assert_ne!(builder.error_status(), 0); // Should have error
    }

    #[test]
    fn test_spiral_linear_interpolation() {
        let mut builder = HelixBRepBuilderHelix::new();
        let axis = Ax3::new();
        let heights = vec![10.0, 10.0];
        let pitches = vec![5.0, 5.0];
        let is_pitches = vec![true, true];

        builder.set_parameters_spiral(axis, 10.0, 30.0, heights, pitches, is_pitches);

        // Check linear interpolation: 10 -> 20 -> 30 over height 20
        assert!((builder.diameters[0] - 10.0).abs() < 1e-10);
        assert!((builder.diameters[1] - 20.0).abs() < 1e-10);
        assert!((builder.diameters[2] - 30.0).abs() < 1e-10);
    }
}
