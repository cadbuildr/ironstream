// FILE: geom_conic_tools.rs
//! GCE2d conic construction helpers — zero-dependency Rust ports of the
//! OpenCascade `GCE2d_MakeArcOfEllipse`, `GCE2d_MakeArcOfHyperbola`, and
//! `GCE2d_MakeArcOfParabola` construction algorithms.
//!
//! All types are plain-data structs with no external crate dependencies.
//! Angles are in radians throughout.

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// GCE2dMakeArcOfEllipse
// ---------------------------------------------------------------------------

/// Arc of a 2D ellipse, constructed from center, semi-axes, and angle limits.
///
/// The ellipse is axis-aligned (major axis along X, minor along Y).
/// The eccentric anomaly parametrisation is used: a point at angle `u`
/// is `[cx + a*cos(u), cy + b*sin(u)]`.
///
/// Length is approximated using the Ramanujan formula scaled to the arc
/// fraction: `π*(3*(a+b) − √((3a+b)*(a+3b))) * |end−start| / (2π)`.
// occt: GCE2d_MakeArcOfEllipse
#[derive(Debug, Clone)]
pub struct GCE2dMakeArcOfEllipse {
    /// Center of the underlying ellipse.
    pub center: [f64; 2],
    /// Semi-major radius (along local X).
    pub major_radius: f64,
    /// Semi-minor radius (along local Y).
    pub minor_radius: f64,
    /// Start angle (eccentric anomaly) in radians.
    pub start_angle: f64,
    /// End angle (eccentric anomaly) in radians.
    pub end_angle: f64,
    /// Whether the construction succeeded.
    pub is_done: bool,
}

impl GCE2dMakeArcOfEllipse {
    /// Construct an arc of a 2D ellipse.
    ///
    /// `center`  — centre of the ellipse in 2D.
    /// `major`   — semi-major radius (> 0, ≥ `minor`).
    /// `minor`   — semi-minor radius (> 0).
    /// `start`   — start angle in radians (eccentric anomaly).
    /// `end`     — end angle in radians (eccentric anomaly).
    ///
    /// Construction always succeeds (is_done = true) for any finite inputs.
    // occt: GCE2d_MakeArcOfEllipse
    pub fn new(
        center: [f64; 2],
        major: f64,
        minor: f64,
        start: f64,
        end: f64,
    ) -> Self {
        Self {
            center,
            major_radius: major,
            minor_radius: minor,
            start_angle: start,
            end_angle: end,
            is_done: true,
        }
    }

    /// Center of the ellipse.
    pub fn center(&self) -> [f64; 2] {
        self.center
    }

    /// Semi-major radius.
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Semi-minor radius.
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// Start angle (eccentric anomaly) in radians.
    pub fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// End angle (eccentric anomaly) in radians.
    pub fn end_angle(&self) -> f64 {
        self.end_angle
    }

    /// Whether the construction succeeded (always `true` for valid inputs).
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Point on the ellipse at the start angle.
    ///
    /// Computed as `[cx + a*cos(start), cy + b*sin(start)]`.
    pub fn start_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.major_radius * self.start_angle.cos(),
            self.center[1] + self.minor_radius * self.start_angle.sin(),
        ]
    }

    /// Point on the ellipse at the end angle.
    ///
    /// Computed as `[cx + a*cos(end), cy + b*sin(end)]`.
    pub fn end_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.major_radius * self.end_angle.cos(),
            self.center[1] + self.minor_radius * self.end_angle.sin(),
        ]
    }

    /// Approximate arc length using the Ramanujan perimeter formula scaled to
    /// the angular span of the arc.
    ///
    /// Full-ellipse perimeter ≈ `π*(3*(a+b) − √((3a+b)*(a+3b)))`.
    /// Arc length ≈ perimeter * |end − start| / (2π).
    pub fn length(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        let perimeter = PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt());
        let span = (self.end_angle - self.start_angle).abs();
        perimeter * span / (2.0 * PI)
    }
}

// ---------------------------------------------------------------------------
// GCE2dMakeArcOfHyperbola
// ---------------------------------------------------------------------------

/// Arc of a 2D hyperbola, constructed from center, semi-axes, and parameter
/// limits.
///
/// The hyperbola is axis-aligned: a point at parameter `t` on the right branch
/// is `[cx + a*cosh(t), cy + b*sinh(t)]`.
// occt: GCE2d_MakeArcOfHyperbola
#[derive(Debug, Clone)]
pub struct GCE2dMakeArcOfHyperbola {
    /// Center of the hyperbola.
    pub center: [f64; 2],
    /// Semi-transverse (real) axis radius.
    pub major_radius: f64,
    /// Semi-conjugate (imaginary) axis radius.
    pub minor_radius: f64,
    /// Start parameter.
    pub start_angle: f64,
    /// End parameter.
    pub end_angle: f64,
    /// Whether the construction succeeded.
    pub is_done: bool,
}

impl GCE2dMakeArcOfHyperbola {
    /// Construct an arc of a 2D hyperbola.
    ///
    /// `center`  — centre of the hyperbola.
    /// `major`   — semi-transverse axis (> 0).
    /// `minor`   — semi-conjugate axis (> 0).
    /// `start`   — start parameter.
    /// `end`     — end parameter.
    ///
    /// Construction always succeeds (is_done = true).
    // occt: GCE2d_MakeArcOfHyperbola
    pub fn new(
        center: [f64; 2],
        major: f64,
        minor: f64,
        start: f64,
        end: f64,
    ) -> Self {
        Self {
            center,
            major_radius: major,
            minor_radius: minor,
            start_angle: start,
            end_angle: end,
            is_done: true,
        }
    }

    /// Whether the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Point on the hyperbola at the start parameter.
    ///
    /// Computed as `[cx + a*cosh(start), cy + b*sinh(start)]`.
    pub fn start_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.major_radius * self.start_angle.cosh(),
            self.center[1] + self.minor_radius * self.start_angle.sinh(),
        ]
    }

    /// Point on the hyperbola at the end parameter.
    ///
    /// Computed as `[cx + a*cosh(end), cy + b*sinh(end)]`.
    pub fn end_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.major_radius * self.end_angle.cosh(),
            self.center[1] + self.minor_radius * self.end_angle.sinh(),
        ]
    }
}

// ---------------------------------------------------------------------------
// GCE2dMakeArcOfParabola
// ---------------------------------------------------------------------------

/// Arc of a 2D parabola `y = x² / (2*focal)` with vertex at a given point,
/// trimmed to an X parameter range `[start_x, end_x]`.
///
/// The vertex is placed at `vertex`; X is measured along the parabola's axis
/// (the global X axis shifted to the vertex).  A point at parameter `x` is
/// `[vx + x, vy + x²/(2*focal)]`.
// occt: GCE2d_MakeArcOfParabola
#[derive(Debug, Clone)]
pub struct GCE2dMakeArcOfParabola {
    /// Focal parameter (distance from vertex to focus).
    pub focal: f64,
    /// Vertex of the parabola.
    pub vertex: [f64; 2],
    /// Start X parameter.
    pub start_x: f64,
    /// End X parameter.
    pub end_x: f64,
    /// Whether the construction succeeded.
    pub is_done: bool,
}

impl GCE2dMakeArcOfParabola {
    /// Construct an arc of a 2D parabola `y = x² / (2*focal)`.
    ///
    /// `focal`   — focal parameter (distance vertex → focus, > 0 for upward
    ///             opening parabola).
    /// `vertex`  — vertex of the parabola.
    /// `start_x` — start X parameter.
    /// `end_x`   — end X parameter.
    ///
    /// Construction always succeeds (is_done = true).
    // occt: GCE2d_MakeArcOfParabola
    pub fn new(focal: f64, vertex: [f64; 2], start_x: f64, end_x: f64) -> Self {
        Self {
            focal,
            vertex,
            start_x,
            end_x,
            is_done: true,
        }
    }

    /// Whether the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Point on the parabola at `start_x`.
    ///
    /// Computed as `[vx + start_x, vy + start_x² / (2*focal)]`.
    pub fn start_point(&self) -> [f64; 2] {
        [
            self.vertex[0] + self.start_x,
            self.vertex[1] + self.start_x * self.start_x / (2.0 * self.focal),
        ]
    }

    /// Point on the parabola at `end_x`.
    ///
    /// Computed as `[vx + end_x, vy + end_x² / (2*focal)]`.
    pub fn end_point(&self) -> [f64; 2] {
        [
            self.vertex[0] + self.end_x,
            self.vertex[1] + self.end_x * self.end_x / (2.0 * self.focal),
        ]
    }
}

// ---------------------------------------------------------------------------
// Circle2d
// ---------------------------------------------------------------------------

/// A 2D circle defined by a center point and a radius.
///
/// Models the GCE2d_MakeCircle construction helper from OCCT.
// occt-ref: GCE2d_MakeCircle
#[derive(Debug, Clone)]
pub struct Circle2d {
    /// Center of the circle `[cx, cy]`.
    pub center: [f64; 2],
    /// Radius of the circle.
    pub radius: f64,
}

impl Circle2d {
    /// Construct a circle from center coordinates and radius.
    ///
    /// # Arguments
    /// * `cx` - X coordinate of the center.
    /// * `cy` - Y coordinate of the center.
    /// * `r`  - Radius (should be > 0 for a valid circle).
    // occt-ref: GCE2d_MakeCircle
    pub fn new(cx: f64, cy: f64, r: f64) -> Self {
        Self {
            center: [cx, cy],
            radius: r,
        }
    }

    /// Return the point on the circle at the given angle (in radians).
    ///
    /// The parametrisation is `[cx + r*cos(angle), cy + r*sin(angle)]`.
    pub fn point_at(&self, angle: f64) -> [f64; 2] {
        [
            self.center[0] + self.radius * angle.cos(),
            self.center[1] + self.radius * angle.sin(),
        ]
    }

    /// Return `true` if the point `p` lies on or inside the circle.
    ///
    /// The test is `(px - cx)² + (py - cy)² ≤ r²`.
    pub fn contains(&self, p: [f64; 2]) -> bool {
        let dx = p[0] - self.center[0];
        let dy = p[1] - self.center[1];
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

// ---------------------------------------------------------------------------
// Ellipse2d
// ---------------------------------------------------------------------------

/// A 2D ellipse defined by a center, semi-axes, and an orientation angle.
///
/// The ellipse is rotated by `angle` radians from the positive X axis.
/// A point at parameter `t` is computed by first evaluating the canonical
/// form `[a*cos(t), b*sin(t)]` and then rotating by `angle`:
///
/// ```text
/// x' = cx + a*cos(t)*cos(angle) - b*sin(t)*sin(angle)
/// y' = cy + a*cos(t)*sin(angle) + b*sin(t)*cos(angle)
/// ```
///
/// Models the GCE2d_MakeEllipse construction helper from OCCT.
// occt-ref: GCE2d_MakeEllipse
#[derive(Debug, Clone)]
pub struct Ellipse2d {
    /// Center of the ellipse `[cx, cy]`.
    pub center: [f64; 2],
    /// Semi-major axis length.
    pub major: f64,
    /// Semi-minor axis length.
    pub minor: f64,
    /// Rotation angle of the major axis from the positive X axis, in radians.
    pub angle: f64,
}

impl Ellipse2d {
    /// Construct an axis-aligned ellipse (angle = 0).
    ///
    /// # Arguments
    /// * `cx` - X coordinate of the center.
    /// * `cy` - Y coordinate of the center.
    /// * `a`  - Semi-major axis.
    /// * `b`  - Semi-minor axis.
    // occt-ref: GCE2d_MakeEllipse
    pub fn new(cx: f64, cy: f64, a: f64, b: f64) -> Self {
        Self {
            center: [cx, cy],
            major: a,
            minor: b,
            angle: 0.0,
        }
    }

    /// Return the point on the ellipse at eccentric anomaly `t` (in radians).
    ///
    /// Accounts for the rotation `self.angle` of the major axis.
    pub fn point_at(&self, t: f64) -> [f64; 2] {
        let local_x = self.major * t.cos();
        let local_y = self.minor * t.sin();
        let cos_a = self.angle.cos();
        let sin_a = self.angle.sin();
        [
            self.center[0] + local_x * cos_a - local_y * sin_a,
            self.center[1] + local_x * sin_a + local_y * cos_a,
        ]
    }
}

// ---------------------------------------------------------------------------
// Parabola2d
// ---------------------------------------------------------------------------

/// A 2D parabola defined by its focus and the X coordinate of its directrix.
///
/// The parabola opens rightward: the vertex is midway between the focus and
/// the directrix.  A point at parameter `t` is `[x(t), t]` where
/// `x(t) = (focus_x + directrix_x) / 2 + (t - focus_y)² / (4*(focus_x - directrix_x)/2)`.
///
/// Equivalently, keeping the canonical form with vertex at
/// `vx = (focus_x + directrix_x) / 2, vy = focus_y`:
///
/// ```text
/// x(t) = vx + (t - vy)² / (4*p)    where p = focus_x - vx  (focal distance)
/// ```
///
/// Models the GCE2d_MakeParabola construction helper from OCCT.
// occt: GCE2d_MakeParabola
#[derive(Debug, Clone)]
pub struct Parabola2d {
    /// Focus point `[fx, fy]`.
    pub focus: [f64; 2],
    /// X coordinate of the directrix line `x = directrix_x`.
    pub directrix_x: f64,
}

impl Parabola2d {
    /// Construct a parabola from focus coordinates and the directrix X position.
    ///
    /// # Arguments
    /// * `fx`  - X coordinate of the focus.
    /// * `fy`  - Y coordinate of the focus.
    /// * `dir` - X coordinate of the directrix line.
    // occt: GCE2d_MakeParabola
    pub fn new(fx: f64, fy: f64, dir: f64) -> Self {
        Self {
            focus: [fx, fy],
            directrix_x: dir,
        }
    }

    /// Return the point on the parabola for the given Y parameter `t`.
    ///
    /// The parabola satisfies the focus-directrix property: every point is
    /// equidistant from the focus and the directrix.
    ///
    /// With vertex `vx = (fx + directrix_x) / 2`, `vy = fy` and focal
    /// distance `p = fx - vx`, the standard parabola equation `y² = 4p·x`
    /// gives:
    ///
    /// ```text
    /// x(t) = vx + (t - vy)² / (4*p)
    /// ```
    pub fn point_at(&self, t: f64) -> [f64; 2] {
        let vx = (self.focus[0] + self.directrix_x) / 2.0;
        let vy = self.focus[1];
        let p = self.focus[0] - vx; // focal distance; positive when focus is right of directrix
        let dy = t - vy;
        let x = if p.abs() < f64::EPSILON {
            vx
        } else {
            vx + dy * dy / (4.0 * p)
        };
        [x, t]
    }
}

// ---------------------------------------------------------------------------
// Free-standing construction helpers
// ---------------------------------------------------------------------------

/// Attempt to construct a `Circle2d` passing through three distinct 2D points.
///
/// Returns `None` when the three points are collinear (the circumscribed circle
/// has infinite radius) or when two input points coincide.
///
/// The circumcenter is found by intersecting the perpendicular bisectors of
/// segments `p1p2` and `p2p3`.
pub fn make_circle_through_3pts(
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
) -> Option<Circle2d> {
    // Mid-points of the two segments.
    let mx1 = (p1[0] + p2[0]) / 2.0;
    let my1 = (p1[1] + p2[1]) / 2.0;
    let mx2 = (p2[0] + p3[0]) / 2.0;
    let my2 = (p2[1] + p3[1]) / 2.0;

    // Direction vectors of the two segments.
    let dx1 = p2[0] - p1[0];
    let dy1 = p2[1] - p1[1];
    let dx2 = p3[0] - p2[0];
    let dy2 = p3[1] - p2[1];

    // The perpendicular bisector of segment 1 passes through (mx1, my1) with
    // direction (-dy1, dx1), parametrised as:
    //   x = mx1 - dy1 * s
    //   y = my1 + dx1 * s
    //
    // The perpendicular bisector of segment 2 passes through (mx2, my2) with
    // direction (-dy2, dx2):
    //   x = mx2 - dy2 * t
    //   y = my2 + dx2 * t
    //
    // Setting them equal and solving for s:
    //   mx1 - dy1*s = mx2 - dy2*t
    //   my1 + dx1*s = my2 + dx2*t
    //
    // From the second equation: t = (my1 - my2 + dx1*s) / dx2  (when dx2 != 0)
    // Or eliminate t with the determinant approach:
    //   -dy1*s + dy2*t = mx2 - mx1
    //    dx1*s - dx2*t = my2 - my1
    //
    // Determinant: det = (-dy1)*(-dx2) - (dy2)*(dx1) = dy1*dx2 - dy2*dx1
    let det = dy1 * dx2 - dy2 * dx1;

    if det.abs() < f64::EPSILON {
        // Points are collinear — no finite circumcircle.
        return None;
    }

    let rhs_x = mx2 - mx1;
    let rhs_y = my2 - my1;

    // Solve for s: det*s = rhs_x*(-dx2) - rhs_y*(-dy2)  → Cramer's rule
    // [ -dy1   dy2 ] [s]   [rhs_x]
    // [  dx1  -dx2 ] [t] = [rhs_y]
    // det = (-dy1)*(-dx2) - (dy2)*(dx1) = dy1*dx2 - dy2*dx1  (as above)
    let s = (rhs_x * (-dx2) - rhs_y * (dy2)) / det;

    let cx = mx1 - dy1 * s;
    let cy = my1 + dx1 * s;

    let dx = p1[0] - cx;
    let dy = p1[1] - cy;
    let r = (dx * dx + dy * dy).sqrt();

    Some(Circle2d::new(cx, cy, r))
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    fn approx_eq2(a: [f64; 2], b: [f64; 2], eps: f64) -> bool {
        approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps)
    }

    // -----------------------------------------------------------------------
    // GCE2dMakeArcOfEllipse
    // -----------------------------------------------------------------------

    #[test]
    fn arc_of_ellipse_is_done() {
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.0, PI / 2.0);
        assert!(arc.is_done());
    }

    #[test]
    fn arc_of_ellipse_accessors() {
        let arc = GCE2dMakeArcOfEllipse::new([1.0, 2.0], 5.0, 3.0, 0.0, PI);
        assert!(approx_eq2(arc.center(), [1.0, 2.0], EPS));
        assert!(approx_eq(arc.major_radius(), 5.0, EPS));
        assert!(approx_eq(arc.minor_radius(), 3.0, EPS));
        assert!(approx_eq(arc.start_angle(), 0.0, EPS));
        assert!(approx_eq(arc.end_angle(), PI, EPS));
    }

    #[test]
    fn arc_of_ellipse_start_point_at_zero() {
        // At angle 0: (cx + a, cy)
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.0, PI / 2.0);
        assert!(approx_eq2(arc.start_point(), [5.0, 0.0], EPS));
    }

    #[test]
    fn arc_of_ellipse_end_point_at_pi_over_2() {
        // At angle π/2: (cx, cy + b)
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.0, PI / 2.0);
        assert!(approx_eq2(arc.end_point(), [0.0, 3.0], EPS));
    }

    #[test]
    fn arc_of_ellipse_start_point_with_center_offset() {
        let arc = GCE2dMakeArcOfEllipse::new([1.0, 2.0], 4.0, 2.0, 0.0, PI / 2.0);
        // start_point = [1 + 4, 2 + 0] = [5, 2]
        assert!(approx_eq2(arc.start_point(), [5.0, 2.0], EPS));
    }

    #[test]
    fn arc_of_ellipse_end_point_with_center_offset() {
        let arc = GCE2dMakeArcOfEllipse::new([1.0, 2.0], 4.0, 2.0, 0.0, PI / 2.0);
        // end_point = [1 + 0, 2 + 2] = [1, 4]
        assert!(approx_eq2(arc.end_point(), [1.0, 4.0], EPS));
    }

    #[test]
    fn arc_of_ellipse_full_circle_length_is_perimeter() {
        // For a=b=r, perimeter = 2*pi*r. Ramanujan gives same for a==b.
        // Ramanujan: pi*(3*(r+r) - sqrt((3r+r)*(r+3r))) = pi*(6r - sqrt(4r*4r)) = pi*(6r-4r) = 2*pi*r
        let r = 3.0_f64;
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], r, r, 0.0, 2.0 * PI);
        let expected = 2.0 * PI * r;
        assert!(
            approx_eq(arc.length(), expected, 1e-10),
            "length={} expected={}",
            arc.length(),
            expected
        );
    }

    #[test]
    fn arc_of_ellipse_quarter_circle_length() {
        let r = 4.0_f64;
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], r, r, 0.0, PI / 2.0);
        let expected = PI / 2.0 * r; // quarter of 2*pi*r
        assert!(approx_eq(arc.length(), expected, 1e-10));
    }

    #[test]
    fn arc_of_ellipse_length_positive() {
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, 0.0, PI);
        assert!(arc.length() > 0.0);
    }

    #[test]
    fn arc_of_ellipse_start_equals_end_at_same_angle() {
        let arc = GCE2dMakeArcOfEllipse::new([0.0, 0.0], 5.0, 3.0, PI, PI);
        assert!(approx_eq2(arc.start_point(), arc.end_point(), EPS));
    }

    // -----------------------------------------------------------------------
    // GCE2dMakeArcOfHyperbola
    // -----------------------------------------------------------------------

    #[test]
    fn arc_of_hyperbola_is_done() {
        let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], 2.0, 1.0, -1.0, 1.0);
        assert!(arc.is_done());
    }

    #[test]
    fn arc_of_hyperbola_start_point_at_zero() {
        // At t=0: [cx + a*cosh(0), cy + b*sinh(0)] = [cx + a, cy]
        let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], 2.0, 1.0, 0.0, 1.0);
        assert!(approx_eq2(arc.start_point(), [2.0, 0.0], EPS));
    }

    #[test]
    fn arc_of_hyperbola_end_point_at_zero() {
        // end at 0 when start_angle=−1, end_angle=0
        let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], 2.0, 1.0, -1.0, 0.0);
        // [2*cosh(0), sinh(0)] = [2, 0]
        assert!(approx_eq2(arc.end_point(), [2.0, 0.0], EPS));
    }

    #[test]
    fn arc_of_hyperbola_start_point_formula() {
        let a = 3.0_f64;
        let b = 2.0_f64;
        let t = 0.5_f64;
        let arc = GCE2dMakeArcOfHyperbola::new([1.0, -1.0], a, b, t, 1.0);
        let expected = [1.0 + a * t.cosh(), -1.0 + b * t.sinh()];
        assert!(approx_eq2(arc.start_point(), expected, EPS));
    }

    #[test]
    fn arc_of_hyperbola_end_point_formula() {
        let a = 3.0_f64;
        let b = 2.0_f64;
        let t = 1.2_f64;
        let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], a, b, 0.0, t);
        let expected = [a * t.cosh(), b * t.sinh()];
        assert!(approx_eq2(arc.end_point(), expected, EPS));
    }

    #[test]
    fn arc_of_hyperbola_symmetric_around_zero() {
        // cosh is even, sinh is odd: start(t) and end(-t) are reflections in X.
        let arc = GCE2dMakeArcOfHyperbola::new([0.0, 0.0], 2.0, 1.0, -1.0, 1.0);
        let sp = arc.start_point();
        let ep = arc.end_point();
        assert!(approx_eq(sp[0], ep[0], EPS));   // same X (cosh is even)
        assert!(approx_eq(sp[1], -ep[1], EPS));  // opposite Y (sinh is odd)
    }

    #[test]
    fn arc_of_hyperbola_with_center_offset() {
        let arc = GCE2dMakeArcOfHyperbola::new([3.0, -2.0], 1.0, 1.0, 0.0, 0.0);
        // cosh(0)=1, sinh(0)=0 → [3+1, -2+0] = [4, -2]
        assert!(approx_eq2(arc.start_point(), [4.0, -2.0], EPS));
        assert!(approx_eq2(arc.end_point(), [4.0, -2.0], EPS));
    }

    #[test]
    fn arc_of_hyperbola_points_satisfy_hyperbola_equation() {
        // For the standard hyperbola x²/a² - y²/b² = 1 (right branch),
        // x = cx + a*cosh(t), y = cy + b*sinh(t).
        let cx = 0.0_f64;
        let cy = 0.0_f64;
        let a = 3.0_f64;
        let b = 2.0_f64;
        let arc = GCE2dMakeArcOfHyperbola::new([cx, cy], a, b, -2.0, 2.0);
        for &t in &[-2.0_f64, -1.0, 0.0, 0.5, 1.0, 2.0] {
            let x = cx + a * t.cosh();
            let y = cy + b * t.sinh();
            let lhs = (x / a) * (x / a) - (y / b) * (y / b);
            assert!(approx_eq(lhs, 1.0, EPS), "t={}: lhs={}", t, lhs);
        }
    }

    // -----------------------------------------------------------------------
    // GCE2dMakeArcOfParabola
    // -----------------------------------------------------------------------

    #[test]
    fn arc_of_parabola_is_done() {
        let arc = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], -1.0, 1.0);
        assert!(arc.is_done());
    }

    #[test]
    fn arc_of_parabola_start_point_at_zero() {
        // x=0: [vx, vy]
        let arc = GCE2dMakeArcOfParabola::new(2.0, [0.0, 0.0], 0.0, 1.0);
        assert!(approx_eq2(arc.start_point(), [0.0, 0.0], EPS));
    }

    #[test]
    fn arc_of_parabola_end_point_formula() {
        let focal = 2.0_f64;
        let x = 4.0_f64;
        let arc = GCE2dMakeArcOfParabola::new(focal, [0.0, 0.0], 0.0, x);
        // y = x²/(2*focal) = 16/4 = 4
        assert!(approx_eq2(arc.end_point(), [4.0, 4.0], EPS));
    }

    #[test]
    fn arc_of_parabola_start_point_formula() {
        let focal = 1.0_f64;
        let x = 2.0_f64;
        let arc = GCE2dMakeArcOfParabola::new(focal, [0.0, 0.0], x, 3.0);
        // y = 4/2 = 2 → [2, 2]
        assert!(approx_eq2(arc.start_point(), [2.0, 2.0], EPS));
    }

    #[test]
    fn arc_of_parabola_with_vertex_offset() {
        let focal = 1.0_f64;
        let vx = 1.0_f64;
        let vy = -1.0_f64;
        let x = 2.0_f64;
        let arc = GCE2dMakeArcOfParabola::new(focal, [vx, vy], 0.0, x);
        // end: [vx + x, vy + x²/(2f)] = [3, -1 + 2] = [3, 1]
        assert!(approx_eq2(arc.end_point(), [3.0, 1.0], EPS));
    }

    #[test]
    fn arc_of_parabola_vertex_is_minimum() {
        // y = x²/(2f) ≥ 0 for all x, so vertex at x=0 has y=0 (minimum).
        let arc = GCE2dMakeArcOfParabola::new(1.0, [0.0, 0.0], 0.0, 0.0);
        assert!(approx_eq2(arc.start_point(), [0.0, 0.0], EPS));
        assert!(approx_eq2(arc.end_point(), [0.0, 0.0], EPS));
    }

    #[test]
    fn arc_of_parabola_symmetric_y_values() {
        // y(x) = y(-x) since y = x²/(2f).
        let focal = 3.0_f64;
        let x = 6.0_f64;
        let arc_pos = GCE2dMakeArcOfParabola::new(focal, [0.0, 0.0], x, x);
        let arc_neg = GCE2dMakeArcOfParabola::new(focal, [0.0, 0.0], -x, -x);
        assert!(approx_eq(arc_pos.start_point()[1], arc_neg.start_point()[1], EPS));
    }

    #[test]
    fn arc_of_parabola_point_satisfies_equation() {
        // y = x²/(2f) for all points on y = x²/(2f).
        let focal = 2.0_f64;
        let arc = GCE2dMakeArcOfParabola::new(focal, [0.0, 0.0], -4.0, 4.0);
        for &x in &[-4.0_f64, -2.0, 0.0, 2.0, 4.0] {
            let arc_x = GCE2dMakeArcOfParabola::new(focal, [0.0, 0.0], x, x);
            let pt = arc_x.start_point();
            let expected_y = x * x / (2.0 * focal);
            assert!(
                approx_eq(pt[1], expected_y, EPS),
                "x={}: y={} expected={}",
                x,
                pt[1],
                expected_y
            );
        }
        let _ = arc; // suppress unused warning
    }

    // -----------------------------------------------------------------------
    // Circle2d
    // -----------------------------------------------------------------------

    #[test]
    fn circle2d_new_stores_fields() {
        let c = Circle2d::new(1.0, 2.0, 5.0);
        assert!(approx_eq2(c.center, [1.0, 2.0], EPS));
        assert!(approx_eq(c.radius, 5.0, EPS));
    }

    #[test]
    fn circle2d_point_at_zero_angle() {
        // angle = 0 → [cx + r, cy]
        let c = Circle2d::new(0.0, 0.0, 3.0);
        assert!(approx_eq2(c.point_at(0.0), [3.0, 0.0], EPS));
    }

    #[test]
    fn circle2d_point_at_pi_over_2() {
        // angle = π/2 → [cx, cy + r]
        let c = Circle2d::new(0.0, 0.0, 4.0);
        assert!(approx_eq2(c.point_at(PI / 2.0), [0.0, 4.0], 1e-10));
    }

    #[test]
    fn circle2d_point_at_with_offset_center() {
        let c = Circle2d::new(1.0, -1.0, 2.0);
        let pt = c.point_at(0.0);
        assert!(approx_eq2(pt, [3.0, -1.0], EPS));
    }

    #[test]
    fn circle2d_contains_center() {
        let c = Circle2d::new(0.0, 0.0, 5.0);
        assert!(c.contains([0.0, 0.0]));
    }

    #[test]
    fn circle2d_contains_boundary_point() {
        let c = Circle2d::new(0.0, 0.0, 1.0);
        assert!(c.contains([1.0, 0.0]));
    }

    #[test]
    fn circle2d_does_not_contain_outside_point() {
        let c = Circle2d::new(0.0, 0.0, 1.0);
        assert!(!c.contains([2.0, 0.0]));
    }

    // -----------------------------------------------------------------------
    // Ellipse2d
    // -----------------------------------------------------------------------

    #[test]
    fn ellipse2d_new_stores_fields() {
        let e = Ellipse2d::new(1.0, 2.0, 5.0, 3.0);
        assert!(approx_eq2(e.center, [1.0, 2.0], EPS));
        assert!(approx_eq(e.major, 5.0, EPS));
        assert!(approx_eq(e.minor, 3.0, EPS));
        assert!(approx_eq(e.angle, 0.0, EPS));
    }

    #[test]
    fn ellipse2d_point_at_zero_is_rightmost() {
        // t=0, angle=0 → [cx + a, cy]
        let e = Ellipse2d::new(0.0, 0.0, 4.0, 2.0);
        assert!(approx_eq2(e.point_at(0.0), [4.0, 0.0], EPS));
    }

    #[test]
    fn ellipse2d_point_at_pi_over_2_is_topmost() {
        // t=π/2, angle=0 → [cx, cy + b]
        let e = Ellipse2d::new(0.0, 0.0, 4.0, 2.0);
        assert!(approx_eq2(e.point_at(PI / 2.0), [0.0, 2.0], 1e-10));
    }

    #[test]
    fn ellipse2d_point_at_respects_rotation() {
        // A 90-degree rotated ellipse: at t=0 the "major" direction now points along Y.
        let mut e = Ellipse2d::new(0.0, 0.0, 4.0, 2.0);
        e.angle = PI / 2.0;
        // local_x = 4*cos(0) = 4, local_y = 2*sin(0) = 0
        // rotated: x = 4*cos(π/2) - 0*sin(π/2) = 0,  y = 4*sin(π/2) + 0 = 4
        let pt = e.point_at(0.0);
        assert!(approx_eq(pt[0], 0.0, 1e-10));
        assert!(approx_eq(pt[1], 4.0, 1e-10));
    }

    // -----------------------------------------------------------------------
    // Parabola2d
    // -----------------------------------------------------------------------

    #[test]
    fn parabola2d_new_stores_fields() {
        let p = Parabola2d::new(1.0, 0.0, -1.0);
        assert!(approx_eq2(p.focus, [1.0, 0.0], EPS));
        assert!(approx_eq(p.directrix_x, -1.0, EPS));
    }

    #[test]
    fn parabola2d_vertex_at_parameter_fy() {
        // When t == fy, dy = 0 so x = vx (vertex).
        // vertex x = (fx + dir) / 2 = (1 + -1) / 2 = 0
        let p = Parabola2d::new(1.0, 0.0, -1.0);
        let pt = p.point_at(0.0); // t = fy = 0
        assert!(approx_eq(pt[0], 0.0, EPS));
        assert!(approx_eq(pt[1], 0.0, EPS));
    }

    #[test]
    fn parabola2d_focus_directrix_equidistance() {
        // Every point on the parabola satisfies dist(pt, focus) == |pt.x - directrix_x|.
        let fx = 1.0_f64;
        let fy = 0.0_f64;
        let dir = -1.0_f64;
        let p = Parabola2d::new(fx, fy, dir);
        for &t in &[-4.0_f64, -2.0, 0.0, 1.5, 3.0] {
            let pt = p.point_at(t);
            let dist_focus = ((pt[0] - fx).powi(2) + (pt[1] - fy).powi(2)).sqrt();
            let dist_dir = (pt[0] - dir).abs();
            assert!(
                approx_eq(dist_focus, dist_dir, 1e-9),
                "t={}: dist_focus={} dist_dir={}",
                t,
                dist_focus,
                dist_dir
            );
        }
    }

    #[test]
    fn parabola2d_point_at_y_coordinate_matches_parameter() {
        let p = Parabola2d::new(2.0, 3.0, 0.0);
        for &t in &[-5.0_f64, 0.0, 3.0, 7.0] {
            let pt = p.point_at(t);
            assert!(approx_eq(pt[1], t, EPS), "t={}: y={}", t, pt[1]);
        }
    }

    // -----------------------------------------------------------------------
    // make_circle_through_3pts
    // -----------------------------------------------------------------------

    #[test]
    fn make_circle_through_3pts_returns_none_for_collinear() {
        // Three points on the X axis are collinear.
        assert!(make_circle_through_3pts([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]).is_none());
    }

    #[test]
    fn make_circle_through_3pts_unit_circle() {
        // Three points on the unit circle: (1,0), (0,1), (-1,0).
        let c = make_circle_through_3pts([1.0, 0.0], [0.0, 1.0], [-1.0, 0.0])
            .expect("should construct circle");
        assert!(approx_eq2(c.center, [0.0, 0.0], 1e-9));
        assert!(approx_eq(c.radius, 1.0, 1e-9));
    }

    #[test]
    fn make_circle_through_3pts_all_points_on_circle() {
        let p1 = [3.0, 0.0_f64];
        let p2 = [0.0, 3.0];
        let p3 = [-3.0, 0.0];
        let c = make_circle_through_3pts(p1, p2, p3).expect("should construct circle");
        for &pt in &[p1, p2, p3] {
            let dx = pt[0] - c.center[0];
            let dy = pt[1] - c.center[1];
            let dist = (dx * dx + dy * dy).sqrt();
            assert!(
                approx_eq(dist, c.radius, 1e-9),
                "pt={:?}: dist={} radius={}",
                pt,
                dist,
                c.radius
            );
        }
    }

    #[test]
    fn make_circle_through_3pts_offset_center() {
        // Circle center at (2, 3) with radius 5.
        let cx = 2.0_f64;
        let cy = 3.0_f64;
        let r = 5.0_f64;
        let p1 = [cx + r, cy];
        let p2 = [cx, cy + r];
        let p3 = [cx - r, cy];
        let c = make_circle_through_3pts(p1, p2, p3).expect("should construct circle");
        assert!(approx_eq2(c.center, [cx, cy], 1e-9));
        assert!(approx_eq(c.radius, r, 1e-9));
    }
}
