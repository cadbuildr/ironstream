//! `ConicalSurface` — pure-Rust zero-dependency stub for an analytic conical
//! surface, modelled after OpenCascade's `Geom_ConicalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_ConicalSurface.hxx`).
//!
//! The surface is parameterised as:
//! ```text
//! P(u, v) = apex
//!         + (radius + v * tan(semi_angle)) * (cos(u) * e1 + sin(u) * e2)
//!         + v * axis
//! ```
//! where `e1`, `e2` are orthogonal unit vectors perpendicular to `axis`.
//!
//! - `u` is the angular parameter, periodic with period `2π`.
//! - `v` is the axial (height) parameter, unbounded.
//! - When `radius == 0` the origin of the parameterisation coincides with the
//!   apex; a non-zero `radius` shifts the reference circle to `v = 0`.

use std::f64::consts::PI;

// ── helpers (pure, no allocations) ──────────────────────────────────────────

#[inline]
fn vec_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn vec_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
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
fn vec_norm(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

#[inline]
fn vec_normalize(a: [f64; 3]) -> [f64; 3] {
    let n = vec_norm(a);
    if n > 0.0 {
        vec_scale(a, 1.0 / n)
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// Build two unit vectors perpendicular to `axis` (and to each other).
///
/// Returns `(e1, e2)` such that `{e1, e2, axis_norm}` forms a right-handed
/// orthonormal basis.
fn build_frame(axis: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let az = vec_normalize(axis);
    // Choose a helper vector not parallel to az.
    let helper: [f64; 3] = if az[0].abs() <= az[1].abs() && az[0].abs() <= az[2].abs() {
        [1.0, 0.0, 0.0]
    } else if az[1].abs() <= az[2].abs() {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    let e1 = vec_normalize(vec_cross(az, helper));
    let e2 = vec_cross(az, e1);
    (e1, e2)
}

// ── main type ────────────────────────────────────────────────────────────────

// occt: Geom_ConicalSurface
/// An analytic conical surface in 3-D space.
///
/// The surface is parameterised as:
/// ```text
/// P(u, v) = apex
///         + (radius + v·tan(semi_angle))·(cos(u)·e1 + sin(u)·e2)
///         + v·axis
/// ```
/// where `e1`, `e2` are the implicit radial frame vectors derived from `axis`.
///
/// `semi_angle` is the half-angle between the axis and the generatrix (radians).
/// `radius` is the reference radius at the V = 0 plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConicalSurface {
    /// Origin point of the parameterisation (apex when `radius == 0`).
    pub apex: [f64; 3],
    /// Unit direction of the cone axis (normalised on construction).
    pub axis: [f64; 3],
    /// Half-angle between the axis and the generatrix (radians).
    pub semi_angle: f64,
    /// Reference radius at the V = 0 plane.
    pub radius: f64,
}

impl ConicalSurface {
    // ── constructors ─────────────────────────────────────────────────────────

    /// Create a new `ConicalSurface` with `radius = 0` (apex at origin of
    /// parameterisation).
    ///
    /// `apex`       — the apex point of the cone.
    /// `axis`       — direction of the cone axis (need not be unit length).
    /// `semi_angle` — half-angle in radians; must satisfy `|semi_angle| < π/2`.
    ///
    /// # Panics
    /// Panics if `|semi_angle| >= π/2`.
    pub fn new(apex: [f64; 3], axis: [f64; 3], semi_angle: f64) -> Self {
        assert!(
            semi_angle.abs() < PI / 2.0,
            "ConicalSurface: |semi_angle| must be < π/2, got {semi_angle}"
        );
        Self {
            apex,
            axis: vec_normalize(axis),
            semi_angle,
            radius: 0.0,
        }
    }

    /// Return a copy of this surface with the reference radius changed to `r`.
    ///
    /// A non-zero radius shifts the V = 0 reference circle away from the apex.
    ///
    /// # Panics
    /// Panics if `r < 0`.
    pub fn with_radius(self, r: f64) -> Self {
        assert!(r >= 0.0, "ConicalSurface::with_radius: r must be >= 0, got {r}");
        Self { radius: r, ..self }
    }

    // ── internal frame ────────────────────────────────────────────────────────

    /// Implicit radial frame `(e1, e2)` derived from `self.axis`.
    #[inline]
    fn frame(&self) -> ([f64; 3], [f64; 3]) {
        build_frame(self.axis)
    }

    // ── evaluation ───────────────────────────────────────────────────────────

    /// `Value(u, v)` — point on the cone surface.
    ///
    /// ```text
    /// P(u, v) = apex
    ///         + (radius + v·tan(semi_angle))·(cos(u)·e1 + sin(u)·e2)
    ///         + v·axis
    /// ```
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let (e1, e2) = self.frame();
        let (su, cu) = u.sin_cos();
        let r = self.radius + v * self.semi_angle.tan();
        let radial = vec_add(vec_scale(e1, r * cu), vec_scale(e2, r * su));
        let axial = vec_scale(self.axis, v);
        vec_add(self.apex, vec_add(radial, axial))
    }

    /// `Normal(u, v)` — outward unit normal at `(u, v)`.
    ///
    /// Computed as `normalize(DU × DV)`.
    ///
    /// For the cone the closed-form unit normal is:
    /// `cos(α)·(cos(u)·e1 + sin(u)·e2) − sin(α)·axis`
    /// (flipped when `r(v) < 0`).
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let (e1, e2) = self.frame();
        let (su, cu) = u.sin_cos();
        let (sa, ca) = self.semi_angle.sin_cos();
        let r = self.radius + v * self.semi_angle.tan();
        // Outward unit normal (independent of r's magnitude):
        let n = vec_sub(
            vec_add(vec_scale(e1, ca * cu), vec_scale(e2, ca * su)),
            vec_scale(self.axis, sa),
        );
        if r >= 0.0 {
            vec_normalize(n)
        } else {
            vec_normalize(vec_scale(n, -1.0))
        }
    }

    /// `∂P/∂U` at `(u, v)`.
    ///
    /// `DU = (radius + v·tan(α))·(−sin(u)·e1 + cos(u)·e2)`
    pub fn d1u(&self, u: f64, v: f64) -> [f64; 3] {
        let (e1, e2) = self.frame();
        let (su, cu) = u.sin_cos();
        let r = self.radius + v * self.semi_angle.tan();
        vec_add(vec_scale(e1, -r * su), vec_scale(e2, r * cu))
    }

    /// `∂P/∂V` at `(u, v)`.
    ///
    /// `DV = tan(α)·(cos(u)·e1 + sin(u)·e2) + axis`
    pub fn d1v(&self, u: f64, v: f64) -> [f64; 3] {
        let (e1, e2) = self.frame();
        let (su, cu) = u.sin_cos();
        let ta = self.semi_angle.tan();
        let _ = v; // DV does not depend on v for the cone (linear in v)
        let radial = vec_add(vec_scale(e1, ta * cu), vec_scale(e2, ta * su));
        vec_add(radial, self.axis)
    }

    // ── topology queries ──────────────────────────────────────────────────────

    /// `IsUClosed()` — always `true`; the angular parameter is periodic.
    pub fn is_u_closed() -> bool {
        true
    }

    // ── getters ───────────────────────────────────────────────────────────────

    /// `Apex()` — the apex stored on construction.
    ///
    /// When `radius == 0` this is the true geometric apex.  When `radius > 0`
    /// the field stores the origin of the V = 0 reference plane; the true
    /// geometric apex is at `v = -radius / tan(semi_angle)` along the axis.
    pub fn apex(&self) -> [f64; 3] {
        self.apex
    }
}

// ── free functions ────────────────────────────────────────────────────────────

/// Lateral surface area of a (truncated) cone frustum.
///
/// For a frustum with base radii `r1` and `r2` and height `h`:
/// ```text
/// A = π · (r1 + r2) · slant_height
/// where slant_height = sqrt(h² + (r2 − r1)²)
/// ```
/// When `r2 == 0` this degenerates to the lateral area of a complete cone:
/// `A = π · r1 · sqrt(r1² + h²)`.
pub fn cone_surface_area(r1: f64, r2: f64, h: f64) -> f64 {
    let slant = (h * h + (r2 - r1) * (r2 - r1)).sqrt();
    PI * (r1 + r2) * slant
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1.0e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn vec_approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        approx_eq(a[0], b[0]) && approx_eq(a[1], b[1]) && approx_eq(a[2], b[2])
    }

    #[test]
    fn test_apex_at_v0_u0_radius0() {
        // With radius=0, evaluate(0,0) should return the apex.
        let c = ConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5);
        let p = c.evaluate(0.0, 0.0);
        assert!(vec_approx_eq(p, [0.0, 0.0, 0.0]), "{p:?}");
    }

    #[test]
    fn test_evaluate_along_axis() {
        // At u=0, v=h the point should lie at apex + h*axis + h*tan(a)*e1.
        // For axis=[0,0,1]: helper=[1,0,0], e1=cross([0,0,1],[1,0,0])=[0,1,0].
        let alpha = 0.3_f64;
        let c = ConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], alpha);
        let v = 2.0;
        let p = c.evaluate(0.0, v);
        // e1=[0,1,0] for z-axis cone; at u=0: cos(0)*e1=[0,1,0]
        let expected = [0.0, v * alpha.tan(), v];
        assert!(vec_approx_eq(p, expected), "{p:?} != {expected:?}");
    }

    #[test]
    fn test_is_u_closed() {
        assert!(ConicalSurface::is_u_closed());
    }

    #[test]
    fn test_normal_unit_length() {
        let c = ConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.4);
        let n = c.normal(1.2, 3.0);
        let len = vec_norm(n);
        assert!((len - 1.0).abs() < 1.0e-9, "normal length {len}");
    }

    #[test]
    fn test_d1u_perpendicular_to_axis() {
        let c = ConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.3);
        let du = c.d1u(0.7, 1.5);
        // DU = r*(-sin(u)*e1 + cos(u)*e2).  Both e1 and e2 are perpendicular
        // to the axis, so DU must also be perpendicular to axis [0,0,1].
        let dot = vec_dot(du, [0.0, 0.0, 1.0]);
        assert!(dot.abs() < EPS, "dot DU·axis = {dot}");
    }

    #[test]
    fn test_with_radius() {
        let c = ConicalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5)
            .with_radius(2.0);
        assert_eq!(c.radius, 2.0);
        // At v=0, u=0 the point should be at apex + r*e1.
        // For axis=[0,0,1]: e1=[0,1,0].  cos(0)=1 → radial = r*e1 = [0,2,0].
        let p = c.evaluate(0.0, 0.0);
        assert!(p[0].abs() < EPS, "{p:?}");
        assert!((p[1] - 2.0).abs() < EPS, "{p:?}");
        assert!(p[2].abs() < EPS);
    }

    #[test]
    fn test_cone_surface_area_full_cone() {
        // Lateral area of a full cone: π * r * slant = π * 1 * sqrt(1+1)
        let a = cone_surface_area(1.0, 0.0, 1.0);
        let expected = PI * (2.0_f64).sqrt();
        assert!((a - expected).abs() < 1.0e-12, "{a} != {expected}");
    }

    #[test]
    fn test_cone_surface_area_frustum() {
        // Frustum r1=3, r2=1, h=4 → slant = sqrt(16+4)=sqrt(20)
        let a = cone_surface_area(3.0, 1.0, 4.0);
        let slant = (16.0_f64 + 4.0_f64).sqrt();
        let expected = PI * 4.0 * slant;
        assert!((a - expected).abs() < 1.0e-12, "{a} != {expected}");
    }
}
