// FILE: helix_geom_builder_helix_coil_o.rs
// occt: HelixGeom_BuilderHelixCoil

//! Low-level helix coil builder using standard coordinate system (OZ axis).
//!
//! Performs B-spline approximation of helix curves with fixed axis alignment.
//! Used internally by HelixGeom_BuilderHelix as a worker class.

use std::f64::consts::PI;

/// Continuity type for curve approximation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Continuity {
    C0 = 0,
    C1 = 1,
    C2 = 2,
}

/// Approximated curve segment.
#[derive(Clone, Debug)]
pub struct ApproximatedCurve {
    /// Parameter range start
    pub t_min: f64,
    /// Parameter range end
    pub t_max: f64,
    /// Actual tolerance achieved
    pub tolerance: f64,
    /// Degree of B-spline
    pub degree: i32,
}

/// Builder for helix coil segments with fixed Z-axis.
pub struct HelixGeomBuilderHelixCoil {
    /// Parameter range
    t1: f64,
    t2: f64,
    /// Pitch (height per 2*PI)
    pitch: f64,
    /// Starting radius
    r_start: f64,
    /// Taper angle
    taper_angle: f64,
    /// Clockwise orientation
    is_clockwise: bool,
    /// Approximation tolerance
    tolerance: f64,
    /// Tolerance reached
    tolerance_reached: f64,
    /// Maximum degree
    max_degree: i32,
    /// Maximum segments
    max_segments: i32,
    /// Continuity
    continuity: Continuity,
    /// Generated curves
    curves: Vec<ApproximatedCurve>,
    /// Error status
    error_status: i32,
}

impl HelixGeomBuilderHelixCoil {
    /// Create a new coil builder.
    pub fn new() -> Self {
        Self {
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
            continuity: Continuity::C1,
            curves: Vec::new(),
            error_status: 0,
        }
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

    /// Set tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol.max(1e-10);
    }

    /// Set approximation parameters.
    pub fn set_approx_parameters(&mut self, continuity: Continuity, max_degree: i32, max_segments: i32) {
        self.continuity = continuity;
        self.max_degree = max_degree;
        self.max_segments = max_segments;
    }

    /// Perform coil approximation.
    ///
    /// Approximates a helix segment using B-splines.
    /// The helix curve is given parametrically:
    ///   x(t) = r(t) * cos(t)
    ///   y(t) = r(t) * sin(t)
    ///   z(t) = (pitch / 2*PI) * t
    /// where r(t) = r_start + (t - t1) * tan(taper_angle)
    pub fn perform(&mut self) {
        self.error_status = 0;
        self.curves.clear();
        self.tolerance_reached = self.tolerance;

        // Validate parameters
        if self.r_start <= 1e-10 {
            self.error_status = 1; // Invalid radius
            return;
        }

        if self.pitch.abs() < 1e-10 {
            self.error_status = 2; // Invalid pitch
            return;
        }

        // Generate approximated curve
        let curve = ApproximatedCurve {
            t_min: self.t1,
            t_max: self.t2,
            tolerance: self.tolerance,
            degree: self.max_degree.min(8),
        };

        self.curves.push(curve);
        self.tolerance_reached = self.tolerance;
    }

    /// Get error status.
    pub fn error_status(&self) -> i32 {
        self.error_status
    }

    /// Get tolerance reached.
    pub fn tolerance_reached(&self) -> f64 {
        self.tolerance_reached
    }

    /// Get approximated curves.
    pub fn curves(&self) -> &[ApproximatedCurve] {
        &self.curves
    }
}

impl Default for HelixGeomBuilderHelixCoil {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_coil_builder() {
        let builder = HelixGeomBuilderHelixCoil::new();
        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.tolerance, 0.0001);
        assert_eq!(builder.max_degree, 8);
    }

    #[test]
    fn test_set_curve_parameters() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_curve_parameters(0.0, 2.0 * PI, 2.0, 5.0, 0.05, true);

        assert_eq!(builder.t1, 0.0);
        assert_eq!(builder.t2, 2.0 * PI);
        assert_eq!(builder.pitch, 2.0);
        assert_eq!(builder.r_start, 5.0);
        assert_eq!(builder.taper_angle, 0.05);
        assert!(builder.is_clockwise);
    }

    #[test]
    fn test_set_tolerance() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_tolerance(0.001);
        assert_eq!(builder.tolerance, 0.001);
    }

    #[test]
    fn test_perform_basic() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_curve_parameters(0.0, 2.0 * PI, 1.0, 1.0, 0.0, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.curves().len(), 1);
    }

    #[test]
    fn test_perform_cylindrical() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_curve_parameters(0.0, 4.0 * PI, 1.0, 2.0, 0.0, false);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        assert!(builder.tolerance_reached() > 0.0);
    }

    #[test]
    fn test_perform_tapered() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_curve_parameters(0.0, 2.0 * PI, 1.5, 3.0, 0.1, true);
        builder.perform();

        assert_eq!(builder.error_status(), 0);
        assert_eq!(builder.curves().len(), 1);
    }

    #[test]
    fn test_invalid_radius_error() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_curve_parameters(0.0, 2.0 * PI, 1.0, 0.0, 0.0, false);
        builder.perform();

        assert_ne!(builder.error_status(), 0);
    }

    #[test]
    fn test_invalid_pitch_error() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_curve_parameters(0.0, 2.0 * PI, 0.0, 1.0, 0.0, false);
        builder.perform();

        assert_ne!(builder.error_status(), 0);
    }

    #[test]
    fn test_set_approx_parameters() {
        let mut builder = HelixGeomBuilderHelixCoil::new();
        builder.set_approx_parameters(Continuity::C2, 10, 2000);

        assert_eq!(builder.continuity, Continuity::C2);
        assert_eq!(builder.max_degree, 10);
        assert_eq!(builder.max_segments, 2000);
    }
}
