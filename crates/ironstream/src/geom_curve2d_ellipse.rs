//! `Ellipse2dCurve` — a zero-dependency stub for a parameterised 2D ellipse curve.
//!
//! Models the analytic ellipse from OpenCascade's `Geom2d_Ellipse`.  An ellipse
//! is parameterised by angle `U` in the local frame of the ellipse, which may be
//! rotated by `angle` radians with respect to the global X axis:
//!
//! ```text
//! local_x = [cos(angle), sin(angle)]
//! local_y = [-sin(angle), cos(angle)]
//!
//! P(U) = center
//!      + major_radius * cos(U) * local_x
//!      + minor_radius * sin(U) * local_y
//! ```
//!
//! The parameter domain is `[0, 2*PI]`.  The curve is closed and periodic with
//! period `2*PI`.  `reverse()` negates the sense of traversal by flipping the
//! sign of the local Y axis, which maps `U → -U` (equivalently `2*PI - U`).

use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

/// A parameterised analytic ellipse curve in the plane.
///
/// Stores the centre, the major and minor semi-axes, and a rotation angle
/// (in radians) that tilts the major axis away from the global X axis.  The
/// parametrisation follows OCCT's convention for `Geom2d_Ellipse`:
///
/// ```text
/// P(U) = center
///      + major_radius * cos(U) * [cos(angle), sin(angle)]
///      + minor_radius * sin(U) * [-sin(angle), cos(angle)]
/// ```
///
/// `major_radius` must be greater than or equal to `minor_radius`, and both
/// must be non-negative.
// occt-ref: Geom2d_Ellipse
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse2dCurve {
    /// Centre of the ellipse `[cx, cy]`.
    pub center: [f64; 2],
    /// Semi-major axis length (along the local X direction).
    pub major_radius: f64,
    /// Semi-minor axis length (along the local Y direction).
    pub minor_radius: f64,
    /// Rotation angle of the major axis with respect to the global X axis
    /// (radians, counter-clockwise positive).
    pub angle: f64,
}

impl Ellipse2dCurve {
    /// Constructs a new axis-aligned ellipse with the given centre and radii.
    ///
    /// The rotation angle is initialised to `0.0` (major axis along global X).
    ///
    /// # Panics
    /// Panics if `major < 0`, `minor < 0`, or `major < minor`.
    #[inline]
    pub fn new(cx: f64, cy: f64, major: f64, minor: f64) -> Self {
        assert!(
            major >= 0.0,
            "Ellipse2dCurve::new: major_radius must be non-negative"
        );
        assert!(
            minor >= 0.0,
            "Ellipse2dCurve::new: minor_radius must be non-negative (pass the unsigned value; use reverse() to flip orientation)"
        );
        assert!(
            major >= minor,
            "Ellipse2dCurve::new: major_radius must be >= minor_radius"
        );
        Self {
            center: [cx, cy],
            major_radius: major,
            minor_radius: minor,
            angle: 0.0,
        }
    }

    /// Returns a copy of this ellipse rotated so its major axis makes `angle`
    /// radians with the global X axis.
    #[inline]
    pub fn with_rotation(mut self, angle: f64) -> Self {
        self.angle = angle;
        self
    }

    /// Returns the point on the ellipse at parameter `u`.
    ///
    /// ```text
    /// local_x = [cos(angle), sin(angle)]
    /// local_y = [-sin(angle), cos(angle)]
    /// P(U)    = center + major*cos(U)*local_x + minor*sin(U)*local_y
    /// ```
    #[inline]
    pub fn point_at(&self, u: f64) -> [f64; 2] {
        let (su, cu) = u.sin_cos();
        let (sa, ca) = self.angle.sin_cos();
        // local_x scaled by major*cos(u), local_y scaled by minor*sin(u)
        let lx = self.major_radius * cu;
        let ly = self.minor_radius * su;
        [
            self.center[0] + lx * ca - ly * sa,
            self.center[1] + lx * sa + ly * ca,
        ]
    }

    /// Returns the first derivative (tangent vector) at parameter `u`.
    ///
    /// ```text
    /// D1(U) = -major*sin(U)*local_x + minor*cos(U)*local_y
    /// ```
    #[inline]
    pub fn d1(&self, u: f64) -> [f64; 2] {
        let (su, cu) = u.sin_cos();
        let (sa, ca) = self.angle.sin_cos();
        let dlx = -self.major_radius * su;
        let dly = self.minor_radius * cu;
        [dlx * ca - dly * sa, dlx * sa + dly * ca]
    }

    /// The first parameter value: `0.0`.
    #[inline]
    pub fn first_parameter() -> f64 {
        0.0
    }

    /// The last parameter value: `2*PI`.
    #[inline]
    pub fn last_parameter() -> f64 {
        TWO_PI
    }

    /// Returns `true` — an ellipse is always a closed curve.
    #[inline]
    pub fn is_closed() -> bool {
        true
    }

    /// Returns `true` — an ellipse is always periodic.
    #[inline]
    pub fn is_periodic() -> bool {
        true
    }

    /// The period of the ellipse: `2*PI`.
    #[inline]
    pub fn period() -> f64 {
        TWO_PI
    }

    /// Returns the first focus of the ellipse.
    ///
    /// The foci lie along the major axis at distance `c` from the centre,
    /// where `c = sqrt(major² - minor²)`.
    ///
    /// ```text
    /// F1 = center + c * [cos(angle), sin(angle)]
    /// ```
    #[inline]
    pub fn focus1(&self) -> [f64; 2] {
        let c = self.focal_distance();
        let (sa, ca) = self.angle.sin_cos();
        [self.center[0] + c * ca, self.center[1] + c * sa]
    }

    /// Returns the second focus of the ellipse.
    ///
    /// ```text
    /// F2 = center - c * [cos(angle), sin(angle)]
    /// ```
    #[inline]
    pub fn focus2(&self) -> [f64; 2] {
        let c = self.focal_distance();
        let (sa, ca) = self.angle.sin_cos();
        [self.center[0] - c * ca, self.center[1] - c * sa]
    }

    /// Returns the eccentricity of the ellipse: `e = c / major_radius`.
    ///
    /// Returns `0.0` if `major_radius == 0` (degenerate point).  The result
    /// is stable after [`reverse()`](Self::reverse) has been called.
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        if self.major_radius == 0.0 {
            return 0.0;
        }
        self.focal_distance() / self.major_radius
    }

    /// Reverses the orientation of the ellipse.
    ///
    /// Mirroring OCCT's `Geom2d_Curve::Reverse`, the sense of traversal is
    /// inverted by negating the local Y axis of the ellipse frame, which maps
    /// `U → -U`.  The effect is that `point_at(u)` after `reverse()` equals
    /// `point_at(-u)` before `reverse()`.
    ///
    /// This is implemented by negating `minor_radius` (treating it as a signed
    /// quantity).  Callers that need the unsigned semi-axis length should use
    /// `minor_radius.abs()` after a reversal.
    #[inline]
    pub fn reverse(&mut self) {
        // Negate the signed minor radius to flip the local Y axis.
        // P(u) = a*cos(u)*local_x + b*sin(u)*local_y
        // With b negated: P(u) = a*cos(u)*local_x - b*sin(u)*local_y = P_orig(-u).
        self.minor_radius = -self.minor_radius;
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Half focal distance: `c = sqrt(major² - minor²)`.
    ///
    /// Uses the absolute value of `minor_radius` so the result is stable
    /// after [`reverse()`](Self::reverse) has been called.
    #[inline]
    fn focal_distance(&self) -> f64 {
        let b = self.minor_radius.abs();
        let d2 = self.major_radius * self.major_radius - b * b;
        if d2 <= 0.0 { 0.0 } else { d2.sqrt() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tolerance for floating-point comparisons.
    const EPS: f64 = 1e-12;

    #[test]
    fn point_at_zero_lies_on_major_axis() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0);
        let p = e.point_at(0.0);
        assert!((p[0] - 5.0).abs() < EPS, "x={}", p[0]);
        assert!(p[1].abs() < EPS, "y={}", p[1]);
    }

    #[test]
    fn point_at_half_pi_lies_on_minor_axis() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0);
        let p = e.point_at(PI / 2.0);
        assert!(p[0].abs() < EPS, "x={}", p[0]);
        assert!((p[1] - 3.0).abs() < EPS, "y={}", p[1]);
    }

    #[test]
    fn point_at_pi_returns_negative_major_end() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 4.0, 2.0);
        let p = e.point_at(PI);
        assert!((p[0] + 4.0).abs() < EPS, "x={}", p[0]);
        assert!(p[1].abs() < EPS, "y={}", p[1]);
    }

    #[test]
    fn point_at_is_periodic() {
        let e = Ellipse2dCurve::new(1.0, 2.0, 6.0, 3.0);
        let p0 = e.point_at(0.5);
        let p1 = e.point_at(0.5 + TWO_PI);
        assert!((p0[0] - p1[0]).abs() < EPS);
        assert!((p0[1] - p1[1]).abs() < EPS);
    }

    #[test]
    fn d1_is_orthogonal_to_radius_at_zero_for_circle() {
        // When major == minor the ellipse is a circle; d1 should be
        // perpendicular to the position vector.
        let e = Ellipse2dCurve::new(0.0, 0.0, 4.0, 4.0);
        let p = e.point_at(PI / 6.0);
        let v = e.d1(PI / 6.0);
        let dot = p[0] * v[0] + p[1] * v[1];
        assert!(dot.abs() < EPS, "dot product = {}", dot);
    }

    #[test]
    fn d1_at_zero_points_along_minor_axis() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0);
        let v = e.d1(0.0);
        // At u=0: D1 = [0, minor]
        assert!(v[0].abs() < EPS, "dx={}", v[0]);
        assert!((v[1] - 3.0).abs() < EPS, "dy={}", v[1]);
    }

    #[test]
    fn parameter_bounds_and_topology() {
        assert_eq!(Ellipse2dCurve::first_parameter(), 0.0);
        assert!((Ellipse2dCurve::last_parameter() - TWO_PI).abs() < 1e-15);
        assert!(Ellipse2dCurve::is_closed());
        assert!(Ellipse2dCurve::is_periodic());
        assert!((Ellipse2dCurve::period() - TWO_PI).abs() < 1e-15);
    }

    #[test]
    fn foci_lie_on_major_axis() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0);
        // c = sqrt(25 - 9) = 4
        let f1 = e.focus1();
        let f2 = e.focus2();
        assert!((f1[0] - 4.0).abs() < EPS, "f1.x={}", f1[0]);
        assert!(f1[1].abs() < EPS, "f1.y={}", f1[1]);
        assert!((f2[0] + 4.0).abs() < EPS, "f2.x={}", f2[0]);
        assert!(f2[1].abs() < EPS, "f2.y={}", f2[1]);
    }

    #[test]
    fn eccentricity_of_circle_is_zero() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 3.0, 3.0);
        assert!(e.eccentricity().abs() < EPS);
    }

    #[test]
    fn eccentricity_known_value() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0);
        // c = 4, e = 4/5 = 0.8
        assert!((e.eccentricity() - 0.8).abs() < EPS);
    }

    #[test]
    fn with_rotation_sets_angle() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0).with_rotation(PI / 4.0);
        assert!((e.angle - PI / 4.0).abs() < EPS);
    }

    #[test]
    fn rotated_major_end_at_u_zero() {
        // With angle = PI/2 the major axis points along Y.
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0).with_rotation(PI / 2.0);
        let p = e.point_at(0.0);
        // Expected: center + major * [cos(PI/2), sin(PI/2)] = [0, 5]
        assert!(p[0].abs() < EPS, "x={}", p[0]);
        assert!((p[1] - 5.0).abs() < EPS, "y={}", p[1]);
    }

    #[test]
    fn foci_rotate_with_angle() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0).with_rotation(PI / 2.0);
        // c = 4, foci at ±4 along Y
        let f1 = e.focus1();
        let f2 = e.focus2();
        assert!(f1[0].abs() < EPS, "f1.x={}", f1[0]);
        assert!((f1[1] - 4.0).abs() < EPS, "f1.y={}", f1[1]);
        assert!(f2[0].abs() < EPS, "f2.x={}", f2[0]);
        assert!((f2[1] + 4.0).abs() < EPS, "f2.y={}", f2[1]);
    }

    #[test]
    fn reverse_flips_traversal_direction() {
        let e = Ellipse2dCurve::new(0.0, 0.0, 5.0, 3.0);
        let mut er = e;
        er.reverse();
        // After reverse, point_at(u) should equal original point_at(-u).
        let u = 0.7_f64;
        let p_orig = e.point_at(-u);
        let p_rev = er.point_at(u);
        assert!((p_orig[0] - p_rev[0]).abs() < EPS, "x: {} vs {}", p_orig[0], p_rev[0]);
        assert!((p_orig[1] - p_rev[1]).abs() < EPS, "y: {} vs {}", p_orig[1], p_rev[1]);
    }

    #[test]
    fn centered_ellipse_with_offset() {
        let e = Ellipse2dCurve::new(2.0, -1.0, 4.0, 2.0);
        let p = e.point_at(0.0);
        assert!((p[0] - 6.0).abs() < EPS);
        assert!((p[1] + 1.0).abs() < EPS);
    }

    #[test]
    #[should_panic]
    fn new_panics_on_negative_major() {
        let _ = Ellipse2dCurve::new(0.0, 0.0, -1.0, 0.5);
    }

    #[test]
    #[should_panic]
    fn new_panics_on_negative_minor() {
        let _ = Ellipse2dCurve::new(0.0, 0.0, 2.0, -1.0);
    }

    #[test]
    #[should_panic]
    fn new_panics_when_minor_exceeds_major() {
        let _ = Ellipse2dCurve::new(0.0, 0.0, 1.0, 3.0);
    }
}
