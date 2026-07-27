//! `Geom_Circle` — a circle in 3D space, modelled after OpenCascade's
//! `Geom_Circle` (`src/ModelingData/TKG3d/Geom/Geom_Circle.hxx`).
//!
//! A circle is defined by its center, radius, a unit normal to the plane, and a
//! unit X-axis direction within that plane.  The parametric equation is:
//!
//! ```text
//! P(u) = center + radius * cos(u) * x_axis + radius * sin(u) * y_axis
//! ```
//!
//! where `y_axis = normal × x_axis`.  The parameter range is `[0, 2π)`.
//!
//! Pure-Rust, zero external dependencies — only `std::f64`.

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Internal helpers (vectors as [f64; 3])
// ---------------------------------------------------------------------------

#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn norm(v: [f64; 3]) -> f64 {
    dot(v, v).sqrt()
}

#[inline]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let n = norm(v);
    if n == 0.0 {
        v
    } else {
        [v[0] / n, v[1] / n, v[2] / n]
    }
}

#[inline]
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

/// Choose an arbitrary unit vector perpendicular to `v` (which must be
/// non-zero).  Used when no explicit X-axis is provided.
fn any_perpendicular(v: [f64; 3]) -> [f64; 3] {
    // Pick the axis least aligned with v, then take the cross product.
    let ax = if v[0].abs() <= v[1].abs() && v[0].abs() <= v[2].abs() {
        [1.0_f64, 0.0, 0.0]
    } else if v[1].abs() <= v[2].abs() {
        [0.0_f64, 1.0, 0.0]
    } else {
        [0.0_f64, 0.0, 1.0]
    };
    normalize(cross(v, ax))
}

// ---------------------------------------------------------------------------
// GeomCircle
// ---------------------------------------------------------------------------

/// Describes a circle in 3D space.
///
/// Mirrors OCCT's `Geom_Circle`: stores center, radius, unit normal (Z axis of
/// the local frame) and unit X-axis direction within the circle's plane.
// occt-ref: Geom_Circle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomCircle {
    /// Origin / center of the circle.
    pub center: [f64; 3],
    /// Radius of the circle (>= 0).
    pub radius: f64,
    /// Unit normal to the plane of the circle (Z axis of the local frame).
    pub normal: [f64; 3],
    /// Unit X-axis direction within the circle's plane.
    pub x_axis: [f64; 3],
    /// Unit Y-axis direction within the circle's plane (derived from normal×x_axis).
    y_axis_stored: [f64; 3],
}

impl GeomCircle {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// Construct a circle with the given center and radius.
    ///
    /// The normal defaults to `[0, 0, 1]` (XY plane) and the X-axis to
    /// `[1, 0, 0]`.
    ///
    /// # Panics
    /// Panics if `radius < 0`.
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        assert!(radius >= 0.0, "GeomCircle::new: radius must be >= 0");
        let normal = [0.0, 0.0, 1.0];
        let x_axis = [1.0, 0.0, 0.0];
        let y_axis_stored = normalize(cross(normal, x_axis));
        Self {
            center,
            radius,
            normal,
            x_axis,
            y_axis_stored,
        }
    }

    /// Return a copy of `self` with the normal replaced by `n` (normalised).
    ///
    /// The X-axis is recomputed to be perpendicular to the new normal.
    pub fn with_normal(mut self, n: [f64; 3]) -> Self {
        self.normal = normalize(n);
        // Reorthogonalise x_axis against the new normal.
        let xa = self.x_axis;
        // Project xa onto the plane: xa - (xa·n)n
        let proj = dot(xa, self.normal);
        let candidate = add(xa, scale(self.normal, -proj));
        let candidate_len = norm(candidate);
        self.x_axis = if candidate_len > 1e-10 {
            normalize(candidate)
        } else {
            any_perpendicular(self.normal)
        };
        self.y_axis_stored = normalize(cross(self.normal, self.x_axis));
        self
    }

    /// Return a copy of `self` with the X-axis replaced by `xa` (normalised
    /// and orthogonalised against the current normal).
    pub fn with_x_axis(mut self, xa: [f64; 3]) -> Self {
        let proj = dot(xa, self.normal);
        let candidate = add(xa, scale(self.normal, -proj));
        let candidate_len = norm(candidate);
        self.x_axis = if candidate_len > 1e-10 {
            normalize(candidate)
        } else {
            any_perpendicular(self.normal)
        };
        self.y_axis_stored = normalize(cross(self.normal, self.x_axis));
        self
    }

    // ------------------------------------------------------------------
    // Derived frame
    // ------------------------------------------------------------------

    /// Unit Y-axis of the local frame.
    #[inline]
    fn y_axis(&self) -> [f64; 3] {
        self.y_axis_stored
    }

    // ------------------------------------------------------------------
    // Parametric evaluation  (P(u) = center + R·cos u·X + R·sin u·Y)
    // ------------------------------------------------------------------

    /// Point on the circle at parameter `u`.
    ///
    /// `P(u) = center + radius·cos(u)·x_axis + radius·sin(u)·y_axis`
    pub fn point_at(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let y = self.y_axis();
        add(
            self.center,
            add(scale(self.x_axis, self.radius * c), scale(y, self.radius * s)),
        )
    }

    /// First derivative vector at parameter `u` as `[f64; 3]`.
    ///
    /// `D1(u) = -radius·sin(u)·x_axis + radius·cos(u)·y_axis`
    pub fn d1_vec(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let y = self.y_axis();
        add(
            scale(self.x_axis, -self.radius * s),
            scale(y, self.radius * c),
        )
    }

    /// Second derivative vector at parameter `u` as `[f64; 3]`.
    ///
    /// `D2(u) = -radius·cos(u)·x_axis - radius·sin(u)·y_axis`
    pub fn d2_vec(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let y = self.y_axis();
        add(
            scale(self.x_axis, -self.radius * c),
            scale(y, -self.radius * s),
        )
    }

    // ------------------------------------------------------------------
    // Curve properties
    // ------------------------------------------------------------------

    /// First parameter: `0.0`.
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// Last parameter: `2π`.
    pub fn last_parameter(&self) -> f64 {
        2.0 * PI
    }

    /// A circle is a closed curve.
    pub fn is_closed(&self) -> bool {
        true
    }

    /// A circle is periodic with period `2π`.
    pub fn is_periodic(&self) -> bool {
        true
    }

    /// Period of the circle: `2π`.
    pub fn period(&self) -> f64 {
        2.0 * PI
    }

    // ------------------------------------------------------------------
    // Modification
    // ------------------------------------------------------------------

    /// Reverse the orientation of the circle.
    ///
    /// Mirrors OCCT `Reverse()`: the normal is negated so the parametric
    /// direction is reversed.  The X-axis is kept, which means the Y-axis
    /// flips sign — consistent with OCCT where reversing inverts the
    /// handedness.
    pub fn reverse(&mut self) {
        self.normal = scale(self.normal, -1.0);
        self.y_axis_stored = scale(self.y_axis_stored, -1.0);
    }

    // ------------------------------------------------------------------
    // Compatibility constructors and accessors (OCCT-style API)
    // ------------------------------------------------------------------

    /// Compatibility constructor from `gp::Ax3`.
    ///
    /// Allows callers that have a full local coordinate frame (`Ax3`) to
    /// create a `GeomCircle` without manually extracting arrays.
    pub fn from_ax3(ax: crate::gp::Ax3, radius: f64) -> Self {
        assert!(radius >= 0.0, "GeomCircle::from_ax3: radius must be >= 0");
        Self {
            center: [ax.location.x, ax.location.y, ax.location.z],
            radius,
            normal: [ax.z_dir.x, ax.z_dir.y, ax.z_dir.z],
            x_axis: [ax.x_dir.x, ax.x_dir.y, ax.x_dir.z],
            y_axis_stored: [ax.y_dir.x, ax.y_dir.y, ax.y_dir.z],
        }
    }

    /// Compatibility constructor from a `gp_prim::Circ`.
    pub fn from_circ(c: &crate::gp_prim::Circ) -> Self {
        Self::from_ax3(c.position(), c.radius())
    }

    /// Returns the circle's center as a `gp::Pnt`.
    pub fn location(&self) -> crate::gp::Pnt {
        crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2])
    }

    /// Returns the axis of the circle (Z axis through the center).
    pub fn axis(&self) -> crate::gp::Ax1 {
        let center = crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2]);
        let normal = crate::gp::Pnt::new(self.normal[0], self.normal[1], self.normal[2]);
        crate::gp::Ax1::new(center, normal)
    }

    /// Returns the circle's radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns a `gp_prim::Circ` for callers that need the full placement.
    pub fn circ(&self) -> crate::gp_prim::Circ {
        let center = crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2]);
        let normal = crate::gp::Pnt::new(self.normal[0], self.normal[1], self.normal[2]);
        let x_hint = crate::gp::Pnt::new(self.x_axis[0], self.x_axis[1], self.x_axis[2]);
        let ax3 = crate::gp::Ax3::from_origin_normal(center, normal, x_hint);
        crate::gp_prim::Circ::new(ax3, self.radius)
    }

    /// Returns the full local frame as `gp::Ax3`.
    pub fn position(&self) -> crate::gp::Ax3 {
        let center = crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2]);
        let normal = crate::gp::Pnt::new(self.normal[0], self.normal[1], self.normal[2]);
        let x_hint = crate::gp::Pnt::new(self.x_axis[0], self.x_axis[1], self.x_axis[2]);
        crate::gp::Ax3::from_origin_normal(center, normal, x_hint)
    }

    /// `D0(u)` — point at parameter `u`.  Mirrors OCCT `GeomCurve::D0`.
    #[inline]
    pub fn d0(&self, u: f64) -> crate::gp::Pnt {
        let p = self.point_at(u);
        crate::gp::Pnt::new(p[0], p[1], p[2])
    }

    /// `D1(u)` — the point and first derivative at parameter `u`.
    pub fn d1(&self, u: f64) -> (crate::gp::Pnt, crate::gp::Pnt) {
        let p = self.point_at(u);
        let v = self.d1_vec(u);
        (
            crate::gp::Pnt::new(p[0], p[1], p[2]),
            crate::gp::Pnt::new(v[0], v[1], v[2]),
        )
    }

    /// `D2(u)` — the point, first, and second derivatives at parameter `u`.
    pub fn d2(&self, u: f64) -> (crate::gp::Pnt, crate::gp::Pnt, crate::gp::Pnt) {
        let p = self.point_at(u);
        let v1 = self.d1_vec(u);
        let v2 = self.d2_vec(u);
        (
            crate::gp::Pnt::new(p[0], p[1], p[2]),
            crate::gp::Pnt::new(v1[0], v1[1], v1[2]),
            crate::gp::Pnt::new(v2[0], v2[1], v2[2]),
        )
    }

    /// `D3(u)` — the point, first, second, and third derivatives at parameter `u`.
    ///
    /// For a circle the third derivative is: `radius·sin(u)·x_axis - radius·cos(u)·y_axis`
    pub fn d3(&self, u: f64) -> (crate::gp::Pnt, crate::gp::Pnt, crate::gp::Pnt, crate::gp::Pnt) {
        let p = self.point_at(u);
        let v1 = self.d1_vec(u);
        let v2 = self.d2_vec(u);
        // D3 = radius·sin(u)·X - radius·cos(u)·Y = -D1 (for unit circle)
        // More precisely: D3(u) = radius*sin(u)*x_axis - radius*cos(u)*y_axis
        let (s, c) = u.sin_cos();
        let y = self.y_axis();
        let v3 = add(
            scale(self.x_axis, self.radius * s),
            scale(y, -self.radius * c),
        );
        (
            crate::gp::Pnt::new(p[0], p[1], p[2]),
            crate::gp::Pnt::new(v1[0], v1[1], v1[2]),
            crate::gp::Pnt::new(v2[0], v2[1], v2[2]),
            crate::gp::Pnt::new(v3[0], v3[1], v3[2]),
        )
    }

    /// `DN(u, n)` — nth derivative at parameter `u`. Returns `Pnt`.
    ///
    /// For n=0 returns the point; for n>0, the nth derivative vector.
    /// The circle's derivative vectors cycle with period 4 for n≥1:
    /// - n≡1 mod 4: `-R·sin(u)·X + R·cos(u)·Y`
    /// - n≡2 mod 4: `-R·cos(u)·X - R·sin(u)·Y`
    /// - n≡3 mod 4: `R·sin(u)·X - R·cos(u)·Y`
    /// - n≡0 mod 4 (n>0): `R·cos(u)·X + R·sin(u)·Y`
    pub fn dn(&self, u: f64, n: i32) -> crate::gp::Pnt {
        if n == 0 {
            return self.d0(u);
        }
        let (s, c) = u.sin_cos();
        let y = self.y_axis();
        let r = self.radius;
        let v = match n.rem_euclid(4) {
            0 => add(scale(self.x_axis, r * c), scale(y, r * s)),  // n≡0 mod 4, n>0
            1 => add(scale(self.x_axis, -r * s), scale(y, r * c)),
            2 => add(scale(self.x_axis, -r * c), scale(y, -r * s)),
            3 => add(scale(self.x_axis, r * s), scale(y, -r * c)),
            _ => unreachable!(),
        };
        crate::gp::Pnt::new(v[0], v[1], v[2])
    }

    /// `Transform` — apply a transformation to this circle in place.
    pub fn transform(&mut self, t: &crate::gp::Trsf) {
        let center = t.apply_point(crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2]));
        let normal = t.apply_vector(crate::gp::Pnt::new(self.normal[0], self.normal[1], self.normal[2]));
        let x_axis = t.apply_vector(crate::gp::Pnt::new(self.x_axis[0], self.x_axis[1], self.x_axis[2]));
        let y_axis = t.apply_vector(crate::gp::Pnt::new(self.y_axis_stored[0], self.y_axis_stored[1], self.y_axis_stored[2]));
        self.center = [center.x, center.y, center.z];
        self.normal = normalize([normal.x, normal.y, normal.z]);
        self.x_axis = normalize([x_axis.x, x_axis.y, x_axis.z]);
        self.y_axis_stored = normalize([y_axis.x, y_axis.y, y_axis.z]);
        self.radius *= t.scale_factor().abs();
    }

    /// `ReversedParameter(u)` — parameter on the reversed circle.
    ///
    /// For a circle: `ReversedParameter(u) = 2π - u`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        2.0 * PI - u
    }
}

// ---------------------------------------------------------------------------
// Free-standing constructor
// ---------------------------------------------------------------------------

/// Construct a `GeomCircle` through three non-collinear points.
///
/// Returns `None` if the three points are collinear (no unique circle) or if
/// any two points coincide.
///
/// The algorithm follows OCCT `gce_MakeCirc`:
/// 1. Find the circumcenter by intersecting the two perpendicular bisectors.
/// 2. Derive radius, normal, and X-axis from the result.
pub fn circle_from_3pts(
    p1: [f64; 3],
    p2: [f64; 3],
    p3: [f64; 3],
) -> Option<GeomCircle> {
    // Edge vectors
    let a = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
    let b = [p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]];

    // Normal to the plane of the three points
    let n_raw = cross(a, b);
    let n_len = norm(n_raw);
    if n_len < 1e-14 {
        // Collinear or coincident points
        return None;
    }
    let normal = scale(n_raw, 1.0 / n_len);

    // Circumcenter via the standard formula using the lengths of the edges.
    // Let |a|^2 = dot(a,a), |b|^2 = dot(b,b), |a×b|^2 = n_len^2.
    // Center = p1 + [(|b|^2*(a·a) - (a·b)*(b·a))/(2|a×b|^2)] * a_perp + ...
    //
    // Compact closed form (see, e.g., "circumcenter from three points"):
    //   C = p1 + [ (|b|²·(a·a - a·b)) * a  +  (|a|²·(b·b - a·b)) * b ]
    //           -------------------------------------------------------
    //                       2 · |a×b|²
    //
    // Simpler equivalent used here:
    //   u_s  = |b|²·(a·a) - (a·b)²
    //   v_s  = |a|²·(b·b) - (a·b)²  ... (not used directly)
    // We use the "barycentric" method:

    let aa = dot(a, a);
    let bb = dot(b, b);
    let ab = dot(a, b);
    let denom = 2.0 * n_len * n_len; // = 2·|a×b|²

    if denom.abs() < 1e-28 {
        return None;
    }

    // Coefficients for a and b in: center = p1 + alpha*a + beta*b
    let alpha = (bb * (aa - ab)) / denom;
    let beta = (aa * (bb - ab)) / denom;

    let center = [
        p1[0] + alpha * a[0] + beta * b[0],
        p1[1] + alpha * a[1] + beta * b[1],
        p1[2] + alpha * a[2] + beta * b[2],
    ];

    // Radius = distance from center to p1
    let r_vec = [p1[0] - center[0], p1[1] - center[1], p1[2] - center[2]];
    let radius = norm(r_vec);
    if radius < 1e-14 {
        return None;
    }

    // X-axis points from center toward p1
    let x_axis = normalize(r_vec);
    let y_axis_stored = normalize(cross(normal, x_axis));

    Some(GeomCircle {
        center,
        radius,
        normal,
        x_axis,
        y_axis_stored,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        let d = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
        norm(d) < EPS
    }

    #[test]
    fn test_new_defaults() {
        let c = GeomCircle::new([0.0, 0.0, 0.0], 1.0);
        assert_eq!(c.normal, [0.0, 0.0, 1.0]);
        assert_eq!(c.x_axis, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_point_at_origin() {
        let c = GeomCircle::new([0.0, 0.0, 0.0], 2.0);
        // u=0 → (2, 0, 0)
        let p = c.point_at(0.0);
        assert!((p[0] - 2.0).abs() < EPS);
        assert!(p[1].abs() < EPS);
        assert!(p[2].abs() < EPS);

        // u=PI/2 → (0, 2, 0)
        let p2 = c.point_at(PI / 2.0);
        assert!(p2[0].abs() < EPS);
        assert!((p2[1] - 2.0).abs() < EPS);
        assert!(p2[2].abs() < EPS);
    }

    #[test]
    fn test_d1_is_tangent() {
        let c = GeomCircle::new([1.0, 2.0, 3.0], 5.0);
        let u = 0.7;
        let pt = c.point_at(u);
        let d1 = c.d1_vec(u);
        // d1 should be perpendicular to (point - center)
        let r = [pt[0] - c.center[0], pt[1] - c.center[1], pt[2] - c.center[2]];
        let dp = dot(r, d1);
        assert!(dp.abs() < EPS * 100.0, "d1 not perpendicular to radius: {dp}");
    }

    #[test]
    fn test_d2_points_toward_center() {
        let c = GeomCircle::new([0.0, 0.0, 0.0], 3.0);
        let u = 1.1;
        let pt = c.point_at(u);
        let d2 = c.d2_vec(u);
        // d2 = -R*cos(u)*X - R*sin(u)*Y = -(point - center) for unit circle scaled
        let expected = scale(pt, -1.0);
        assert!(
            approx_eq(d2, expected),
            "d2={d2:?} expected={expected:?}"
        );
    }

    #[test]
    fn test_period_closed() {
        let c = GeomCircle::new([0.0, 0.0, 0.0], 1.0);
        assert!(c.is_closed());
        assert!(c.is_periodic());
        assert!((c.period() - 2.0 * PI).abs() < EPS);
        assert_eq!(c.first_parameter(), 0.0);
        assert!((c.last_parameter() - 2.0 * PI).abs() < EPS);
    }

    #[test]
    fn test_reverse() {
        let mut c = GeomCircle::new([0.0, 0.0, 0.0], 1.0);
        c.reverse();
        assert_eq!(c.normal, [0.0, 0.0, -1.0]);
    }

    #[test]
    fn test_circle_from_3pts_unit_xy() {
        // Three points on the unit circle in the XY plane
        let p1 = [1.0_f64, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let gc = circle_from_3pts(p1, p2, p3).expect("should succeed");
        assert!((gc.center[0]).abs() < EPS, "cx={}", gc.center[0]);
        assert!((gc.center[1]).abs() < EPS, "cy={}", gc.center[1]);
        assert!((gc.center[2]).abs() < EPS, "cz={}", gc.center[2]);
        assert!((gc.radius - 1.0).abs() < EPS, "r={}", gc.radius);
    }

    #[test]
    fn test_circle_from_3pts_collinear_returns_none() {
        let p1 = [0.0_f64, 0.0, 0.0];
        let p2 = [1.0, 0.0, 0.0];
        let p3 = [2.0, 0.0, 0.0];
        assert!(circle_from_3pts(p1, p2, p3).is_none());
    }

    #[test]
    fn test_circle_from_3pts_point_lies_on_circle() {
        let p1 = [3.0_f64, 0.0, 0.0];
        let p2 = [0.0, 3.0, 0.0];
        let p3 = [-3.0, 0.0, 0.0];
        let gc = circle_from_3pts(p1, p2, p3).expect("should succeed");
        // p2 should lie on the circle
        let d = [
            p2[0] - gc.center[0],
            p2[1] - gc.center[1],
            p2[2] - gc.center[2],
        ];
        assert!((norm(d) - gc.radius).abs() < 1e-9);
    }

    #[test]
    fn test_with_normal_reorthogonalises_x_axis() {
        let c = GeomCircle::new([0.0, 0.0, 0.0], 1.0)
            .with_normal([0.0, 1.0, 0.0]); // XZ plane
        // x_axis must be perpendicular to the new normal
        let dp = dot(c.x_axis, c.normal);
        assert!(dp.abs() < EPS, "x_axis not perp to normal: {dp}");
    }
}
