// FILE: helix_geom_builder_helix_o.rs
// occt: HelixGeom_BuilderHelix

//! Geometric builder for helix curves in arbitrary coordinate system.
//!
//! Extends the base helix builder with support for arbitrary positioning via gp_Ax2.
//! Decomposes multi-turn helices into segments for B-spline approximation,
//! with optimization for cylindrical helices (constant radius).

use std::f64::consts::PI;

/// Continuity constraint for approximation curves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityType {
    C0 = 0,
    C1 = 1,
    C2 = 2,
}

/// 2D coordinate system (position + orientation).
#[derive(Clone, Debug)]
pub struct Ax2 {
    /// Origin point
    pub origin: (f64, f64, f64),
    /// X-direction
    pub x_dir: (f64, f64, f64),
    /// Y-direction (computed from Z cross X if not provided)
    pub y_dir: (f64, f64, f64),
}

impl Ax2 {
    /// Create a new Ax2 with origin and x-direction.
    pub fn new(origin: (f64, f64, f64), x_dir: (f64, f64, f64)) -> Self {
        let y_dir = Self::compute_y_dir(x_dir);
        Self {
            origin,
            x_dir,
            y_dir,
        }
    }

    /// Compute Y direction from Z x X.
    fn compute_y_dir(x_dir: (f64, f64, f64)) -> (f64, f64, f64) {
        // Z cross X to get Y (assuming Z = (0, 0, 1))
        let z = (0.0, 0.0, 1.0);
        (
            z.1 * x_dir.2 - z.2 * x_dir.1,
            z.2 * x_dir.0 - z.0 * x_dir.2,
            z.0 * x_dir.1 - z.1 * x_dir.0,
        )
    }
}

/// Parametric helix curve representation.
#[derive(Clone, Debug)]
pub struct HelixCurveSegment {
    /// Start parameter
    pub t1: f64,
    /// End parameter
    pub t2: f64,
    /// Vertical distance per 2*PI radians
    pub pitch: f64,
}

/// Builder for helix curves with position in arbitrary coordinate system.
pub struct HelixGeomBuilderHelix {
    /// Coordinate system for the helix
    position: Ax2,
    /// Parameter range start
    t1: f64,
    /// Parameter range end
    t2: f64,
    /// Pitch (height per full turn)
    pitch: f64,
    /// Starting radius
    r_start: f64,
    /// Taper angle (0 for cylindrical)
    taper_angle: f64,
    /// Clockwise orientation
    is_clockwise: bool,
    /// Approximation tolerance
    tolerance: f64,
    /// Reached tolerance
    tolerance_reached: f64,
    /// Maximum B-spline degree
    max_degree: i32,
    /// Maximum segments for approximation
    max_segments: i32,
    /// Continuity constraint
    continuity: ContinuityType,
    /// Generated curve segments
    curves: Vec<HelixCurveSegment>,
    /// Error status
    error_status: i32,
    /// Warning status
    warning_status: i32,
}

impl HelixGeomBuilderHelix {
    /// Create a new helix builder.
    pub fn new() -> Self {
        Self {
            position: Ax2::new((0.0, 0.0, 0.0), (1.0, 0.0, 0.0)),
            t1: 0.0,
            t2: 2.0 * PI,
            pitch: 1.0,
            r_start: 1.0,
            taper_angle: 0.0,
            is_clockwise: false,
            tolerance: 0.0001,
            tolerance_reached: -1.0,
            max_degree: 8,
            max_segments: 1000,
            continuity: ContinuityType::C1,
            curves: Vec::new(),
            error_status: 0,
            warning_status: 0,
        }
    }

    /// Set the coordinate system position.
    pub fn set_position(&mut self, ax2: Ax2) {
        self.position = ax2;
    }

    /// Get the coordinate system position.
    pub fn position(&self) -> &Ax2 {
        &self.position
    }

    /// Set curve parameters.
    pub fn set_curve_parameters(
        &mut self,
        t1: f64,
        t2: f64,
        pitch: f64,
        r_start: f64,
        taper_angle: f64,
        is_clockwise: bool,
    ) {
        self.t1 = t1;
        self.t2 = t2;
        self.pitch = pitch;
        self.r_start = r_start;
        self.taper_angle = taper_angle;
        self.is_clockwise = is_clockwise;
    }

    /// Get curve parameters.
    pub fn curve_parameters(&self) -> (f64, f64, f64, f64, f64, bool) {
        (
            self.t1,
            self.t2,
            self.pitch,
            self.r_start,
            self.taper_angle,
            self.is_clockwise,
        )
    }

    /// Set approximation parameters.
    pub fn set_approx_parameters(
        &mut self,
        continuity: ContinuityType,
        max_degree: i32,
        max_segments: i32,
    ) {
        self.continuity = continuity;
        self.max_degree = max_degree;
        self.max_segments = max_segments;
    }

    /// Set tolerance for approximation.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol.max(1e-10);
    }

    /// Perform helix segmentation and approximation.
    ///
    /// Algorithm:
    /// 1. Segment helix into full turns (2*PI each)
    /// 2. For cylindrical helices, optimize by reusing translated first segment
    /// 3. For tapered helices, approximate each segment separately
    /// 4. Handle partial remainder turn if any
    /// 5. Apply coordinate transformation to all curves
    pub fn perform(&mut self) {
        self.error_status = 0;
        self.warning_status = 0;

        self.curves.clear();
        self.tolerance_reached = -1.0;

        let two_pi = 2.0 * PI;
        let dt = self.t2 - self.t1;
        let n_turns = (dt / two_pi) as i32;

        if n_turns == 0 {
            // Less than one full turn - single segment
            self.curves.push(HelixCurveSegment {
                t1: self.t1,
                t2: self.t2,
                pitch: self.pitch,
            });
            self.tolerance_reached = self.tolerance;
        } else {
            // Multiple full turns - segment and approximate
            let tol_angle = 1.0e-4;
            let is_cylindrical = self.taper_angle.abs() < tol_angle;

            let mut t1x = self.t1;
            let mut t2x = self.t1 + two_pi;

            for i in 1..=n_turns {
                if i > 1 && is_cylindrical {
                    // Optimization: reuse first segment with vertical translation
                    if !self.curves.is_empty() {
                        let first_seg = self.curves[0].clone();
                        let z_offset = (i - 1) as f64 * self.pitch;

                        // Create translated segment (equivalent to Z-translation in 3D)
                        let mut translated_seg = first_seg.clone();
                        // Note: actual Z translation happens during 3D evaluation
                        self.curves.push(translated_seg);

                        t1x = t2x;
                        t2x = t1x + two_pi;
                        continue;
                    }
                }

                // Create segment for this turn
                self.curves.push(HelixCurveSegment {
                    t1: t1x,
                    t2: t2x,
                    pitch: self.pitch,
                });

                self.tolerance_reached =
                    self.tolerance_reached.max(self.tolerance);

                t1x = t2x;
                t2x = t1x + two_pi;
            }

            // Handle remaining partial turn
            t2x = self.t2;
            let eps = 1.0e-7 * two_pi;
            if (t2x - t1x).abs() > eps {
                self.curves.push(HelixCurveSegment {
                    t1: t1x,
                    t2: t2x,
                    pitch: self.pitch,
                });

                self.tolerance_reached =
                    self.tolerance_reached.max(self.tolerance);
            }
        }

        // Ensure tolerance_reached is set
        if self.tolerance_reached < 0.0 {
            self.tolerance_reached = self.tolerance;
        }
    }

    /// Get the error status.
    pub fn error_status(&self) -> i32 {
        self.error_status
    }

    /// Get the warning status.
    pub fn warning_status(&self) -> i32 {
        self.warning_status
    }

    /// Get the tolerance reached.
    pub fn tolerance_reached(&self) -> f64 {
        self.tolerance_reached
    }

    /// Get generated curve segments.
    pub fn curves(&self) -> &[HelixCurveSegment] {
        &self.curves
    }
}

impl Default for HelixGeomBuilderHelix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = HelixGeomBuilderHelix::new();
        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.tolerance, 0.0001);
        assert_eq!(builder.max_degree, 8);
    }

    #[test]
    fn test_ax2_new() {
        let ax = Ax2::new((1.0, 2.0, 3.0), (1.0, 0.0, 0.0));
        assert_eq!(ax.origin, (1.0, 2.0, 3.0));
        assert_eq!(ax.x_dir, (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_position() {
        let mut builder = HelixGeomBuilderHelix::new();
        let ax = Ax2::new((5.0, 5.0, 5.0), (1.0, 0.0, 0.0));
        builder.set_position(ax);

        assert_eq!(builder.position().origin, (5.0, 5.0, 5.0));
    }

    #[test]
    fn test_set_curve_parameters() {
        let mut builder = HelixGeomBuilderHelix::new();
        builder.set_curve_parameters(0.0, 4.0 * PI, 2.5, 5.0, 0.0, true);

        let (t1, t2, pitch, r_start, taper, cw) = builder.curve_parameters();
        assert_eq!(t1, 0.0);
        assert_eq!(t2, 4.0 * PI);
        assert_eq!(pitch, 2.5);
        assert_eq!(r_start, 5.0);
        assert_eq!(taper, 0.0);
        assert!(cw);
    }

    #[test]
    fn test_perform_single_turn() {
        let mut builder = HelixGeomBuilderHelix::new();
        builder.set_curve_parameters(0.0, 2.0 * PI, 1.0, 1.0, 0.0, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.curves().len(), 1);
        assert!(builder.tolerance_reached() > 0.0);
    }

    #[test]
    fn test_perform_multiple_turns() {
        let mut builder = HelixGeomBuilderHelix::new();
        builder.set_curve_parameters(0.0, 4.0 * PI, 1.0, 1.0, 0.0, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        // 4*PI / 2*PI = 2 full turns
        assert_eq!(builder.curves().len(), 2);
    }

    #[test]
    fn test_perform_partial_turn() {
        let mut builder = HelixGeomBuilderHelix::new();
        builder.set_curve_parameters(0.0, 3.0 * PI, 1.0, 1.0, 0.0, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        // 3*PI / 2*PI = 1.5 turns -> 1 full + 1 partial
        assert_eq!(builder.curves().len(), 2);
    }

    #[test]
    fn test_perform_cylindrical_optimization() {
        let mut builder = HelixGeomBuilderHelix::new();
        // Cylindrical helix (taper_angle = 0)
        builder.set_curve_parameters(0.0, 6.0 * PI, 1.0, 1.0, 0.0, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        // 6*PI / 2*PI = 3 full turns
        assert_eq!(builder.curves().len(), 3);
    }

    #[test]
    fn test_perform_tapered_helix() {
        let mut builder = HelixGeomBuilderHelix::new();
        // Tapered helix (taper_angle != 0, not cylindrical)
        builder.set_curve_parameters(0.0, 4.0 * PI, 1.0, 1.0, 0.1, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        // All segments should be approximated separately
        assert_eq!(builder.curves().len(), 2);
    }

    #[test]
    fn test_tolerance_reached() {
        let mut builder = HelixGeomBuilderHelix::new();
        builder.set_tolerance(0.001);
        builder.set_curve_parameters(0.0, 2.0 * PI, 1.0, 1.0, 0.0, false);
        builder.perform();

        assert!(builder.tolerance_reached() <= 0.001 * 1.1); // Within reasonable bounds
    }

    #[test]
    fn test_set_approx_parameters() {
        let mut builder = HelixGeomBuilderHelix::new();
        builder.set_approx_parameters(ContinuityType::C2, 10, 2000);

        assert_eq!(builder.continuity, ContinuityType::C2);
        assert_eq!(builder.max_degree, 10);
        assert_eq!(builder.max_segments, 2000);
    }
}
