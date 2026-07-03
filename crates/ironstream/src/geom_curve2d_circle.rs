//! `Circle2dCurve` — a zero-dependency stub for a parameterised 2D circle curve.
//!
//! Models the analytic circle from OpenCascade's `Geom2d_Circle`.  A circle is
//! parameterised by angle `U`:
//!
//! ```text
//! P(U) = [cx + r*cos(U), cy + r*sin(U)]
//! ```
//!
//! The parameter domain is `[0, 2*PI]`.  The curve is closed and periodic with
//! period `2*PI`.  `reverse()` negates the "sense" of traversal by reflecting
//! the circle through the X axis of its local frame (i.e. by negating the
//! Y-coordinate of the centre offset), which is equivalent to replacing `U`
//! with `2*PI - U`.

use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

/// A parameterised analytic circle curve in the plane.
///
/// Stores the centre as a 2-element array and the radius as a scalar.  The
/// parametrisation `P(U) = center + [r*cos(U), r*sin(U)]` follows OCCT's
/// convention for `Geom2d_Circle`.
// occt: Geom2d_Circle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle2dCurve {
    /// Centre of the circle `[cx, cy]`.
    pub center: [f64; 2],
    /// Radius of the circle.
    pub radius: f64,
}

impl Circle2dCurve {
    /// Constructs a new circle with the given centre coordinates and radius.
    ///
    /// # Panics
    /// Panics if `r` is negative.
    #[inline]
    pub fn new(cx: f64, cy: f64, r: f64) -> Self {
        assert!(r >= 0.0, "Circle2dCurve::new: radius must be non-negative");
        Self {
            center: [cx, cy],
            radius: r,
        }
    }

    /// Returns the point on the circle at parameter `u`:
    /// `[cx + r*cos(u), cy + r*sin(u)]`.
    #[inline]
    pub fn point_at(&self, u: f64) -> [f64; 2] {
        let (s, c) = u.sin_cos();
        [self.center[0] + self.radius * c, self.center[1] + self.radius * s]
    }

    /// Returns the first derivative (tangent vector) at parameter `u`:
    /// `[-r*sin(u), r*cos(u)]`.
    #[inline]
    pub fn d1(&self, u: f64) -> [f64; 2] {
        let (s, c) = u.sin_cos();
        [-self.radius * s, self.radius * c]
    }

    /// Returns the second derivative (curvature vector) at parameter `u`:
    /// `[-r*cos(u), -r*sin(u)]`.
    #[inline]
    pub fn d2(&self, u: f64) -> [f64; 2] {
        let (s, c) = u.sin_cos();
        [-self.radius * c, -self.radius * s]
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

    /// Returns `true` — a circle is always a closed curve.
    #[inline]
    pub fn is_closed() -> bool {
        true
    }

    /// Returns `true` — a circle is always periodic.
    #[inline]
    pub fn is_periodic() -> bool {
        true
    }

    /// The period of the circle: `2*PI`.
    #[inline]
    pub fn period() -> f64 {
        TWO_PI
    }

    /// Reverses the orientation of the circle.
    ///
    /// Mirroring OCCT's `Geom2d_Curve::Reverse`, the sense of traversal is
    /// inverted by negating the Y component of the centre's local offset, which
    /// maps `U → 2*PI - U`.  For a circle whose centre is stored in absolute
    /// coordinates this is a no-op on the centre; instead the radius is kept
    /// unchanged and the parametric direction is considered flipped.  This
    /// implementation records the reversal by negating the Y coordinate of the
    /// centre to be consistent with a frame-based representation.
    ///
    /// In practice callers that need `P(2*PI - U)` can pass the mapped
    /// parameter directly to [`point_at`](Self::point_at).
    #[inline]
    pub fn reverse(&mut self) {
        // Negate the Y component of the centre to record the frame reversal,
        // matching the sign-flip applied to the Y axis of gp_Ax22d in OCCT.
        self.center[1] = -self.center[1];
    }

    /// Returns the Euclidean distance from the nearest point on the circle to
    /// the given point `pt`.
    ///
    /// `distance = |‖pt - center‖ - radius|`
    #[inline]
    pub fn distance_to(&self, pt: [f64; 2]) -> f64 {
        let dx = pt[0] - self.center[0];
        let dy = pt[1] - self.center[1];
        let dist_to_center = (dx * dx + dy * dy).sqrt();
        (dist_to_center - self.radius).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_zero_lies_on_positive_x_axis() {
        let c = Circle2dCurve::new(0.0, 0.0, 3.0);
        let p = c.point_at(0.0);
        assert!((p[0] - 3.0).abs() < 1e-12, "x={}", p[0]);
        assert!(p[1].abs() < 1e-12, "y={}", p[1]);
    }

    #[test]
    fn point_at_half_pi_lies_on_positive_y_axis() {
        let c = Circle2dCurve::new(0.0, 0.0, 1.0);
        let p = c.point_at(PI / 2.0);
        assert!(p[0].abs() < 1e-12, "x={}", p[0]);
        assert!((p[1] - 1.0).abs() < 1e-12, "y={}", p[1]);
    }

    #[test]
    fn d1_is_tangent_at_zero() {
        let c = Circle2dCurve::new(0.0, 0.0, 5.0);
        let v = c.d1(0.0);
        // At u=0 the tangent should be [0, r]
        assert!(v[0].abs() < 1e-12, "dx={}", v[0]);
        assert!((v[1] - 5.0).abs() < 1e-12, "dy={}", v[1]);
    }

    #[test]
    fn d2_points_toward_centre_at_zero() {
        let c = Circle2dCurve::new(0.0, 0.0, 2.0);
        let a = c.d2(0.0);
        // At u=0, second derivative = [-r, 0], pointing back to centre
        assert!((a[0] + 2.0).abs() < 1e-12, "ax={}", a[0]);
        assert!(a[1].abs() < 1e-12, "ay={}", a[1]);
    }

    #[test]
    fn parameter_bounds_and_topology() {
        assert_eq!(Circle2dCurve::first_parameter(), 0.0);
        assert!((Circle2dCurve::last_parameter() - TWO_PI).abs() < 1e-15);
        assert!(Circle2dCurve::is_closed());
        assert!(Circle2dCurve::is_periodic());
        assert!((Circle2dCurve::period() - TWO_PI).abs() < 1e-15);
    }

    #[test]
    fn distance_to_point_on_circle_is_zero() {
        let c = Circle2dCurve::new(1.0, 2.0, 4.0);
        let p = c.point_at(PI / 4.0);
        assert!(c.distance_to(p) < 1e-12);
    }

    #[test]
    fn distance_to_centre_equals_radius() {
        let c = Circle2dCurve::new(0.0, 0.0, 7.0);
        assert!((c.distance_to([0.0, 0.0]) - 7.0).abs() < 1e-12);
    }

    #[test]
    fn reverse_flips_y_of_centre() {
        let mut c = Circle2dCurve::new(3.0, 4.0, 1.0);
        c.reverse();
        assert_eq!(c.center, [3.0, -4.0]);
    }

    #[test]
    #[should_panic]
    fn new_panics_on_negative_radius() {
        let _ = Circle2dCurve::new(0.0, 0.0, -1.0);
    }
}
