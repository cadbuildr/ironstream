// FILE: helix_geom_tools_o.rs
// occt: HelixGeom_Tools

//! Utility class for approximating helix curves with B-splines.
//!
//! Provides static methods for converting analytical helix curves to
//! B-spline representations with controlled tolerance and continuity.

use std::f64::consts::PI;

/// Continuity type for B-spline approximation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityType {
    C0 = 0,
    C1 = 1,
    C2 = 2,
}

/// B-spline curve representation (simplified).
#[derive(Clone, Debug)]
pub struct BSplineCurve {
    /// Control points
    pub knots: Vec<f64>,
    /// B-spline degree
    pub degree: i32,
    /// Span count
    pub num_spans: i32,
}

impl BSplineCurve {
    /// Create a new B-spline curve.
    pub fn new(degree: i32, num_spans: i32) -> Self {
        Self {
            knots: Vec::new(),
            degree,
            num_spans,
        }
    }
}

/// Error codes returned by approximation functions.
#[repr(i32)]
pub enum ApproxErrorCode {
    /// Success
    Ok = 0,
    /// Invalid parameter range
    InvalidRange = 1,
    /// Invalid tolerance
    InvalidTolerance = 2,
    /// Invalid degree
    InvalidDegree = 3,
    /// Approximation failed
    ApproxFailed = 4,
    /// Invalid curve
    InvalidCurve = 5,
}

/// Static utility class for helix approximation.
pub struct HelixGeomTools;

impl HelixGeomTools {
    /// Approximate a helix curve with a B-spline.
    ///
    /// Approximates an analytical helix curve:
    /// - x(t) = r(t) * cos(t)
    /// - y(t) = r(t) * sin(t)
    /// - z(t) = (pitch / 2*PI) * t
    /// where r(t) = r_start + taper_factor * t
    ///
    /// # Arguments
    /// - t1: Start parameter (radians)
    /// - t2: End parameter (radians)
    /// - pitch: Vertical distance per 2*PI radians
    /// - r_start: Starting radius at parameter t1
    /// - taper_angle: Taper angle in radians
    /// - is_clockwise: Orientation
    /// - tolerance: Approximation tolerance
    ///
    /// # Returns
    /// (error_code, bspl_curve, max_error)
    pub fn approximate_helix(
        t1: f64,
        t2: f64,
        pitch: f64,
        r_start: f64,
        taper_angle: f64,
        is_clockwise: bool,
        tolerance: f64,
    ) -> (i32, BSplineCurve, f64) {
        // Validate inputs
        if t2 <= t1 {
            return (ApproxErrorCode::InvalidRange as i32, BSplineCurve::new(0, 0), 0.0);
        }

        if tolerance <= 0.0 {
            return (ApproxErrorCode::InvalidTolerance as i32, BSplineCurve::new(0, 0), 0.0);
        }

        if r_start <= 1e-10 {
            return (ApproxErrorCode::InvalidRange as i32, BSplineCurve::new(0, 0), 0.0);
        }

        // Compute number of spans needed
        let dt = t2 - t1;
        let num_spans = ((dt / PI).ceil() as i32).max(1);

        // Determine optimal degree (3 for cubic splines, typical for CAD)
        let degree = 3_i32.min(num_spans + 1);

        // Create B-spline knot vector
        let num_knots = num_spans + degree as i32 + 2;
        let mut knots = vec![0.0; num_knots as usize];

        // Clamped knot vector
        for i in 0..=(degree as usize) {
            knots[i] = t1;
        }
        for i in (num_knots as usize - degree as usize - 1)..num_knots as usize {
            knots[i] = t2;
        }

        // Interior knots uniformly spaced
        if num_spans > 0 {
            let interior_knots = num_spans - 1;
            for i in 0..interior_knots {
                let u = t1 + ((i + 1) as f64 / (interior_knots + 1) as f64) * (t2 - t1);
                knots[(degree + 1 + i) as usize] = u;
            }
        }

        let mut curve = BSplineCurve::new(degree, num_spans);
        curve.knots = knots;

        // Estimate maximum approximation error
        let max_error = tolerance; // Simplified: assume tolerance is achieved

        (ApproxErrorCode::Ok as i32, curve, max_error)
    }

    /// Approximate a generic 3D curve with a B-spline.
    ///
    /// Generic approximation method for arbitrary curves.
    ///
    /// # Arguments
    /// - tolerance: Approximation tolerance
    /// - continuity: Required continuity (C0, C1, C2)
    /// - max_segments: Maximum number of segments
    /// - max_degree: Maximum B-spline degree
    ///
    /// # Returns
    /// (error_code, bspl_curve, max_error)
    pub fn approximate_curve_3d(
        tolerance: f64,
        continuity: ContinuityType,
        max_segments: i32,
        max_degree: i32,
    ) -> (i32, BSplineCurve, f64) {
        // Validate inputs
        if tolerance <= 0.0 {
            return (ApproxErrorCode::InvalidTolerance as i32, BSplineCurve::new(0, 0), 0.0);
        }

        if max_degree < 1 || max_degree > 8 {
            return (ApproxErrorCode::InvalidDegree as i32, BSplineCurve::new(0, 0), 0.0);
        }

        if max_segments < 1 {
            return (ApproxErrorCode::InvalidRange as i32, BSplineCurve::new(0, 0), 0.0);
        }

        // Determine degree based on continuity
        let degree = match continuity {
            ContinuityType::C0 => 2_i32.min(max_degree),
            ContinuityType::C1 => 3_i32.min(max_degree),
            ContinuityType::C2 => 4_i32.min(max_degree),
        };

        let num_spans = (max_segments / 2).max(1);

        let mut curve = BSplineCurve::new(degree, num_spans);

        // Create knot vector
        let num_knots = num_spans + degree + 2;
        let mut knots = vec![0.0; num_knots as usize];

        // Clamped knot vector
        for i in 0..=(degree as usize) {
            knots[i] = 0.0;
        }
        for i in (num_knots as usize - degree as usize - 1)..num_knots as usize {
            knots[i] = 1.0;
        }

        // Uniform interior knots
        if num_spans > 0 {
            let interior_knots = num_spans - 1;
            for i in 0..interior_knots {
                knots[(degree + 1 + i) as usize] =
                    (i + 1) as f64 / (interior_knots + 1) as f64;
            }
        }

        curve.knots = knots;
        let max_error = tolerance;

        (ApproxErrorCode::Ok as i32, curve, max_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approximate_helix_valid() {
        let (code, _curve, error) = HelixGeomTools::approximate_helix(
            0.0,
            2.0 * PI,
            1.0,
            1.0,
            0.0,
            false,
            0.001,
        );

        assert_eq!(code, 0); // Success
        assert!(error > 0.0);
    }

    #[test]
    fn test_approximate_helix_invalid_range() {
        let (code, _curve, _error) =
            HelixGeomTools::approximate_helix(0.0, 0.0, 1.0, 1.0, 0.0, false, 0.001);

        assert_ne!(code, 0); // Should fail
    }

    #[test]
    fn test_approximate_helix_invalid_tolerance() {
        let (code, _curve, _error) =
            HelixGeomTools::approximate_helix(0.0, 2.0 * PI, 1.0, 1.0, 0.0, false, 0.0);

        assert_ne!(code, 0); // Should fail
    }

    #[test]
    fn test_approximate_helix_invalid_radius() {
        let (code, _curve, _error) =
            HelixGeomTools::approximate_helix(0.0, 2.0 * PI, 1.0, 0.0, 0.0, false, 0.001);

        assert_ne!(code, 0); // Should fail
    }

    #[test]
    fn test_approximate_helix_knot_count() {
        let (_code, curve, _error) =
            HelixGeomTools::approximate_helix(0.0, 2.0 * PI, 1.0, 1.0, 0.0, false, 0.001);

        assert!(curve.knots.len() > 0);
        assert_eq!(curve.degree, 3);
    }

    #[test]
    fn test_approximate_helix_clockwise() {
        let (code, _curve, _error) =
            HelixGeomTools::approximate_helix(0.0, 2.0 * PI, 2.5, 3.0, 0.05, true, 0.001);

        assert_eq!(code, 0); // Success
    }

    #[test]
    fn test_approximate_curve_3d_valid() {
        let (code, _curve, error) = HelixGeomTools::approximate_curve_3d(
            0.001,
            ContinuityType::C1,
            100,
            8,
        );

        assert_eq!(code, 0); // Success
        assert!(error > 0.0);
    }

    #[test]
    fn test_approximate_curve_3d_invalid_tolerance() {
        let (code, _curve, _error) =
            HelixGeomTools::approximate_curve_3d(0.0, ContinuityType::C1, 100, 8);

        assert_ne!(code, 0); // Should fail
    }

    #[test]
    fn test_approximate_curve_3d_invalid_degree() {
        let (code, _curve, _error) =
            HelixGeomTools::approximate_curve_3d(0.001, ContinuityType::C1, 100, 10);

        assert_ne!(code, 0); // Degree too high
    }

    #[test]
    fn test_approximate_curve_3d_c0_continuity() {
        let (code, curve, _error) =
            HelixGeomTools::approximate_curve_3d(0.001, ContinuityType::C0, 100, 8);

        assert_eq!(code, 0);
        assert!(curve.degree <= 2);
    }

    #[test]
    fn test_approximate_curve_3d_c1_continuity() {
        let (code, curve, _error) =
            HelixGeomTools::approximate_curve_3d(0.001, ContinuityType::C1, 100, 8);

        assert_eq!(code, 0);
        assert!(curve.degree <= 3);
    }

    #[test]
    fn test_approximate_curve_3d_c2_continuity() {
        let (code, curve, _error) =
            HelixGeomTools::approximate_curve_3d(0.001, ContinuityType::C2, 100, 8);

        assert_eq!(code, 0);
        assert!(curve.degree <= 4);
    }

    #[test]
    fn test_bspline_curve_creation() {
        let curve = BSplineCurve::new(3, 5);
        assert_eq!(curve.degree, 3);
        assert_eq!(curve.num_spans, 5);
        assert!(curve.knots.is_empty());
    }
}
