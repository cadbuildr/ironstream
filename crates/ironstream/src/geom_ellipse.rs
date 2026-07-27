//! `Geom_Ellipse` — pure-Rust zero-dependency stub modelling OCCT's
//! `Geom_Ellipse`.
//!
//! An ellipse is placed in 3D space by a centre point, a normal to the plane,
//! an implicit major-axis X direction (computed from the normal), and two
//! radii.  The parametric equation is:
//!
//! ```text
//! P(U) = center + major_radius*cos(U)*x_dir + minor_radius*sin(U)*y_dir
//! ```
//!
//! The parameter range is `[0, 2*PI]` (closed, periodic).

use std::f64::consts::PI;

// ─────────────────────────────── helpers ────────────────────────────────────

#[inline]
fn vec_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn vec_scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn vec_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn vec_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn vec_norm(v: [f64; 3]) -> f64 {
    vec_dot(v, v).sqrt()
}

#[inline]
fn vec_normalize(v: [f64; 3]) -> [f64; 3] {
    let n = vec_norm(v);
    if n == 0.0 {
        v
    } else {
        vec_scale(v, 1.0 / n)
    }
}

/// Build an orthonormal basis (x_dir, y_dir) for the plane with the given
/// normal, using the strategy from OCCT's `gp_Ax2::gp_Ax2`.
fn basis_from_normal(normal: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let n = vec_normalize(normal);
    // choose an initial guess for x_dir that is not parallel to n
    let guess: [f64; 3] = if n[0].abs() <= n[1].abs() && n[0].abs() <= n[2].abs() {
        [1.0, 0.0, 0.0]
    } else if n[1].abs() <= n[2].abs() {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    // Gram-Schmidt: remove the n component
    let proj = vec_scale(n, vec_dot(guess, n));
    let x_dir = vec_normalize(vec_sub(guess, proj));
    let y_dir = vec_cross(n, x_dir);
    (x_dir, y_dir)
}

// ─────────────────────────────── main type ──────────────────────────────────

/// `Geom_Ellipse` — a parameterised analytic ellipse in 3D space.
///
/// Mirrors OCCT's `Geom_Ellipse`: the ellipse is defined by its centre, a
/// plane normal (Z direction), and major/minor radii.  The major axis (X
/// direction) is derived from the normal via an orthogonal complement.
///
/// Parametric equation: `P(U) = center + semi_major*cos(U)*x_dir +
/// semi_minor*sin(U)*y_dir`, `U ∈ [0, 2π]`.
// occt-ref: Geom_Ellipse
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomEllipse {
    /// Centre of the ellipse in 3D space.
    pub center: [f64; 3],
    /// Half-length of the major axis (must be >= semi_minor >= 0).
    pub semi_major: f64,
    /// Half-length of the minor axis.
    pub semi_minor: f64,
    /// Unit normal to the plane of the ellipse (Z direction).
    pub normal: [f64; 3],
    /// Unit major-axis direction (X direction), derived from `normal`.
    x_dir: [f64; 3],
    /// Unit minor-axis direction (Y direction), derived from `normal`.
    y_dir: [f64; 3],
}

impl GeomEllipse {
    // ──────────────────────────────── constructors ────────────────────────────

    /// `Geom_Ellipse(center, major_radius, minor_radius)` — construct with the
    /// default normal `[0, 0, 1]` (the XY plane).
    ///
    /// # Panics
    /// Panics (matching OCCT's `Standard_ConstructionError`) if
    /// `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn new(center: [f64; 3], major: f64, minor: f64) -> Self {
        assert!(
            minor >= 0.0 && major >= minor,
            "GeomEllipse: require minor_radius >= 0 and major_radius >= minor_radius"
        );
        let normal = [0.0, 0.0, 1.0];
        let (x_dir, y_dir) = basis_from_normal(normal);
        Self {
            center,
            semi_major: major,
            semi_minor: minor,
            normal,
            x_dir,
            y_dir,
        }
    }

    /// Builder: override the plane normal, recomputing the local X/Y axes.
    ///
    /// Returns a new `GeomEllipse` with the given unit normal.
    pub fn with_normal(mut self, n: [f64; 3]) -> Self {
        self.normal = vec_normalize(n);
        let (x_dir, y_dir) = basis_from_normal(self.normal);
        self.x_dir = x_dir;
        self.y_dir = y_dir;
        self
    }

    // ──────────────────────────────── evaluation ──────────────────────────────

    /// `D0(U)` / `Value(U)` — the point of parameter `U`:
    /// `P = center + semi_major*cos(U)*x_dir + semi_minor*sin(U)*y_dir`.
    pub fn point_at(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let px = vec_scale(self.x_dir, self.semi_major * c);
        let py = vec_scale(self.y_dir, self.semi_minor * s);
        vec_add(self.center, vec_add(px, py))
    }

    /// `D1(U)` — the first derivative (tangent) vector at parameter `U` as `[f64; 3]`:
    /// `V = -semi_major*sin(U)*x_dir + semi_minor*cos(U)*y_dir`.
    pub fn d1_vec(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let vx = vec_scale(self.x_dir, -self.semi_major * s);
        let vy = vec_scale(self.y_dir, self.semi_minor * c);
        vec_add(vx, vy)
    }

    /// `D2(U)` — the second derivative vector at parameter `U` as `[f64; 3]`:
    /// `V = -semi_major*cos(U)*x_dir - semi_minor*sin(U)*y_dir`.
    pub fn d2_vec(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let vx = vec_scale(self.x_dir, -self.semi_major * c);
        let vy = vec_scale(self.y_dir, -self.semi_minor * s);
        vec_add(vx, vy)
    }

    /// `D3(U)` — the third derivative vector at parameter `U` as `[f64; 3]`:
    /// `V = semi_major*sin(U)*x_dir - semi_minor*cos(U)*y_dir`.
    pub fn d3_vec(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        let vx = vec_scale(self.x_dir, self.semi_major * s);
        let vy = vec_scale(self.y_dir, -self.semi_minor * c);
        vec_add(vx, vy)
    }

    // ──────────────────────────────── conic properties ────────────────────────

    /// `Focus1()` — first focus: `center + sqrt(a^2 - b^2) * x_dir`.
    pub fn focus1(&self) -> [f64; 3] {
        let c = (self.semi_major * self.semi_major
            - self.semi_minor * self.semi_minor)
            .max(0.0)
            .sqrt();
        vec_add(self.center, vec_scale(self.x_dir, c))
    }

    /// `Focus2()` — second focus: `center - sqrt(a^2 - b^2) * x_dir`.
    pub fn focus2(&self) -> [f64; 3] {
        let c = (self.semi_major * self.semi_major
            - self.semi_minor * self.semi_minor)
            .max(0.0)
            .sqrt();
        vec_sub(self.center, vec_scale(self.x_dir, c))
    }

    /// `Eccentricity()` — `e = sqrt(a^2 - b^2) / a`.
    ///
    /// Returns `0` for a degenerate (zero-major-radius) ellipse.
    pub fn eccentricity(&self) -> f64 {
        if self.semi_major == 0.0 {
            return 0.0;
        }
        (self.semi_major * self.semi_major - self.semi_minor * self.semi_minor)
            .max(0.0)
            .sqrt()
            / self.semi_major
    }

    /// `Focal()` — distance between the two foci: `2 * sqrt(a^2 - b^2)`.
    pub fn focal(&self) -> f64 {
        2.0 * (self.semi_major * self.semi_major - self.semi_minor * self.semi_minor)
            .max(0.0)
            .sqrt()
    }

    /// `Parameter()` — semi-latus rectum: `b^2 / a`.
    ///
    /// Returns `0` for a degenerate (zero-major-radius) ellipse.
    pub fn parameter(&self) -> f64 {
        if self.semi_major == 0.0 {
            return 0.0;
        }
        self.semi_minor * self.semi_minor / self.semi_major
    }

    // ──────────────────────────────── curve interface ─────────────────────────

    /// `FirstParameter()` — `0.0`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// `LastParameter()` — `2*PI`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        2.0 * PI
    }

    /// `IsClosed()` — always `true` for an ellipse.
    #[inline]
    pub fn is_closed(&self) -> bool {
        true
    }

    /// `IsPeriodic()` — always `true`; period is `2*PI`.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        true
    }

    /// `Reverse()` — reverse the orientation of the curve by negating the
    /// normal and swapping the Y direction, so that the parameterisation runs
    /// in the opposite direction.
    pub fn reverse(&mut self) {
        // Negate the normal (Z direction).
        self.normal = vec_scale(self.normal, -1.0);
        // Negate the Y direction so the orientation of the local frame flips
        // (X stays the same, matching OCCT's Reverse on Geom_Conic).
        self.y_dir = vec_scale(self.y_dir, -1.0);
    }

    // ------------------------------------------------------------------
    // Compatibility constructors and accessors (OCCT-style API)
    // ------------------------------------------------------------------

    /// Compatibility constructor from `gp::Ax3`.
    ///
    /// Allows callers that have a full local coordinate frame (`Ax3`) to
    /// create a `GeomEllipse` without manually extracting arrays.
    pub fn from_ax3(ax: crate::gp::Ax3, major: f64, minor: f64) -> Self {
        assert!(
            minor >= 0.0 && major >= minor,
            "GeomEllipse::from_ax3: require minor_radius >= 0 and major_radius >= minor_radius"
        );
        let center = [ax.location.x, ax.location.y, ax.location.z];
        let normal = [ax.z_dir.x, ax.z_dir.y, ax.z_dir.z];
        let (x_dir, y_dir) = basis_from_normal(normal);
        Self {
            center,
            semi_major: major,
            semi_minor: minor,
            normal,
            x_dir,
            y_dir,
        }
    }

    /// Returns the major radius (half-length of the major axis).
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.semi_major
    }

    /// Returns the minor radius (half-length of the minor axis).
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.semi_minor
    }

    /// Returns the full local coordinate frame as `gp::Ax3`.
    pub fn position(&self) -> crate::gp::Ax3 {
        let center = crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2]);
        let normal = crate::gp::Pnt::new(self.normal[0], self.normal[1], self.normal[2]);
        let x_hint = crate::gp::Pnt::new(self.x_dir[0], self.x_dir[1], self.x_dir[2]);
        crate::gp::Ax3::from_origin_normal(center, normal, x_hint)
    }

    /// `D0(u)` — point at parameter `u`. Mirrors OCCT `GeomCurve::D0`.
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
    pub fn d3(&self, u: f64) -> (crate::gp::Pnt, crate::gp::Pnt, crate::gp::Pnt, crate::gp::Pnt) {
        let p = self.point_at(u);
        let v1 = self.d1_vec(u);
        let v2 = self.d2_vec(u);
        let v3 = self.d3_vec(u);
        (
            crate::gp::Pnt::new(p[0], p[1], p[2]),
            crate::gp::Pnt::new(v1[0], v1[1], v1[2]),
            crate::gp::Pnt::new(v2[0], v2[1], v2[2]),
            crate::gp::Pnt::new(v3[0], v3[1], v3[2]),
        )
    }

    /// `DN(u, n)` — nth derivative at parameter `u`. Returns `Pnt`.
    pub fn dn(&self, u: f64, n: i32) -> crate::gp::Pnt {
        let v: [f64; 3] = match n {
            0 => self.point_at(u),
            1 => self.d1_vec(u),
            2 => self.d2_vec(u),
            3 => self.d3_vec(u),
            _ => [0.0, 0.0, 0.0],
        };
        crate::gp::Pnt::new(v[0], v[1], v[2])
    }

    /// `Transform` — apply a transformation to this ellipse in place.
    pub fn transform(&mut self, t: &crate::gp::Trsf) {
        let center = t.apply_point(crate::gp::Pnt::new(self.center[0], self.center[1], self.center[2]));
        let normal = t.apply_vector(crate::gp::Pnt::new(self.normal[0], self.normal[1], self.normal[2]));
        let x_hint = crate::gp::Pnt::new(self.x_dir[0], self.x_dir[1], self.x_dir[2]);
        let x_new = t.apply_vector(x_hint);
        let scale = t.scale_factor().abs();
        self.center = [center.x, center.y, center.z];
        self.normal = {
            let n = [normal.x, normal.y, normal.z];
            let len = vec_norm(n);
            if len > 1e-15 { vec_scale(n, 1.0 / len) } else { n }
        };
        self.semi_major *= scale;
        self.semi_minor *= scale;
        // Recompute basis from new normal and x hint
        let (xd, yd) = {
            let x_arr = [x_new.x, x_new.y, x_new.z];
            let proj = vec_scale(self.normal, vec_dot(x_arr, self.normal));
            let xd = vec_normalize(vec_sub(x_arr, proj));
            let yd = vec_cross(self.normal, xd);
            (xd, yd)
        };
        self.x_dir = xd;
        self.y_dir = yd;
    }

    /// `Period()` — the period of the ellipse (always `2π`).
    #[inline]
    pub fn period(&self) -> f64 {
        2.0 * PI
    }

    /// `ReversedParameter(u)` — parameter on the reversed ellipse.
    ///
    /// For an ellipse: `ReversedParameter(u) = 2π - u`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        2.0 * PI - u
    }
}

// ─────────────────────────────── tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const PI2: f64 = 2.0 * PI;
    const EPS: f64 = 1e-10;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn pclose(a: [f64; 3], b: [f64; 3]) -> bool {
        close(a[0], b[0]) && close(a[1], b[1]) && close(a[2], b[2])
    }

    #[test]
    fn test_basic_circle() {
        // When a == b the ellipse is a circle.
        let e = GeomEllipse::new([0.0, 0.0, 0.0], 3.0, 3.0);
        assert!(close(e.eccentricity(), 0.0));
        assert!(close(e.focal(), 0.0));
        assert!(close(e.parameter(), 3.0));
        assert!(pclose(e.focus1(), [0.0, 0.0, 0.0]));
        assert!(pclose(e.focus2(), [0.0, 0.0, 0.0]));
    }

    #[test]
    fn test_point_at_endpoints() {
        let e = GeomEllipse::new([0.0, 0.0, 0.0], 5.0, 3.0);
        // U=0 → major-axis point
        let p0 = e.point_at(0.0);
        assert!(pclose(p0, [5.0, 0.0, 0.0]));
        // U=PI/2 → minor-axis point
        let p1 = e.point_at(PI / 2.0);
        assert!(pclose(p1, [0.0, 3.0, 0.0]));
        // U=PI → opposite major-axis point
        let p2 = e.point_at(PI);
        assert!(pclose(p2, [-5.0, 0.0, 0.0]));
    }

    #[test]
    fn test_d1_perpendicular_to_radius() {
        let e = GeomEllipse::new([0.0, 0.0, 0.0], 4.0, 2.0);
        for ui in 0..8 {
            let u = (ui as f64) * PI2 / 8.0;
            let p = e.point_at(u);
            let t = e.d1_vec(u);
            // For a general ellipse the tangent is not perpendicular to the
            // radius vector, but the dot product between the radius and the
            // tangent has a closed-form: r·t = (b^2-a^2)*sin(u)*cos(u).
            let r = vec_sub(p, e.center);
            let dot = vec_dot(r, t);
            let expected = (e.semi_minor * e.semi_minor
                - e.semi_major * e.semi_major)
                * u.sin()
                * u.cos();
            assert!((dot - expected).abs() < 1e-9, "u={u}: dot={dot} expected={expected}");
        }
    }

    #[test]
    fn test_foci_on_major_axis() {
        let e = GeomEllipse::new([1.0, 2.0, 3.0], 5.0, 4.0);
        let f1 = e.focus1();
        let f2 = e.focus2();
        // Both foci must lie in the plane (z == center.z for default normal).
        assert!(close(f1[2], 3.0));
        assert!(close(f2[2], 3.0));
        // Distance between foci == focal().
        let df = vec_norm(vec_sub(f1, f2));
        assert!(close(df, e.focal()));
    }

    #[test]
    fn test_parameter_range() {
        let e = GeomEllipse::new([0.0, 0.0, 0.0], 2.0, 1.0);
        assert!(close(e.first_parameter(), 0.0));
        assert!(close(e.last_parameter(), PI2));
        assert!(e.is_closed());
        assert!(e.is_periodic());
    }

    #[test]
    fn test_reverse() {
        let e_orig = GeomEllipse::new([0.0, 0.0, 0.0], 3.0, 2.0);
        // Capture a point at a non-trivial parameter before reversing.
        let p_neg = e_orig.point_at(-0.5);
        // The major-axis endpoint (u=0) is unaffected by reversing y_dir.
        let p_zero = e_orig.point_at(0.0);

        let mut e = e_orig;
        e.reverse();

        // After reversing, the y_dir is negated; cos stays the same, sin
        // flips.  Therefore point_at(u) on the reversed curve equals
        // point_at(-u) on the original curve.
        assert!(
            pclose(e.point_at(0.5), p_neg),
            "reversed curve at +0.5 should equal original at -0.5"
        );
        // The major-axis point (u=0, pure cosine) must be identical.
        assert!(
            pclose(e.point_at(0.0), p_zero),
            "reversed curve at 0 should equal original at 0"
        );
        // After reverse(), the normal (Z direction) is also negated along with
        // Y to maintain a right-handed frame (Z = X cross Y).
        assert!(
            close(e.normal[2], -e_orig.normal[2]),
            "normal z-component must be negated after reverse"
        );
    }

    #[test]
    fn test_with_normal() {
        // Ellipse in XZ-plane (normal = Y axis).
        let e = GeomEllipse::new([0.0, 0.0, 0.0], 4.0, 2.0)
            .with_normal([0.0, 1.0, 0.0]);
        assert!(close(e.normal[0], 0.0));
        assert!(close(e.normal[1], 1.0));
        assert!(close(e.normal[2], 0.0));
        // All points must have y == 0 (the plane through origin with normal Y).
        for i in 0..16 {
            let u = (i as f64) * PI2 / 16.0;
            let p = e.point_at(u);
            assert!(p[1].abs() < EPS, "point should stay in XZ plane, got y={}", p[1]);
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_radii() {
        let _ = GeomEllipse::new([0.0, 0.0, 0.0], 1.0, 2.0); // minor > major
    }
}
