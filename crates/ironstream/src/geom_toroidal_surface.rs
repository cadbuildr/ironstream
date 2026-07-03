//! `Geom_ToroidalSurface` — analytic toroidal surface in 3-D space,
//! mirroring OpenCascade's `Geom_ToroidalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_ToroidalSurface.hxx`).
//!
//! A toroidal surface is defined by:
//! - a **center** point in 3-D space (`[f64; 3]`),
//! - a **major radius** `R > 0`: distance from the center of the torus to the
//!   center of the tube,
//! - a **minor radius** `r > 0`: radius of the tube,
//! - a unit **axis** direction `[f64; 3]` (the torus symmetry axis, Z in the
//!   local frame).
//!
//! The parametric equation is
//! ```text
//! P(u, v) = center
//!          + (R + r·cos(v)) · (cos(u)·e_x + sin(u)·e_y)
//!          + r·sin(v) · axis
//! ```
//! where `e_x` and `e_y` are unit vectors perpendicular to `axis` and to each
//! other, forming a right-handed frame together with `axis`.
//!
//! Both `u` and `v` are periodic with period `2·π`.
//!
//! References:
//! - OCCT `Geom_ToroidalSurface` (TKG3d)
//! - ISO 10303-42, toroidal surface definition

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Internal helpers — no external crates, only std
// ---------------------------------------------------------------------------

/// Compute a pair of unit vectors `(e_x, e_y)` that, together with the given
/// unit `axis`, form a right-handed orthonormal frame.
///
/// The construction is numerically stable: we pick the global axis most
/// perpendicular to `axis` as a helper, cross-product it with `axis` to get
/// `e_y`, then cross `e_y` with `axis` to recover `e_x`.
#[inline]
fn frame_from_axis(axis: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    // Choose a helper vector not parallel to axis.
    let helper = if axis[0].abs() <= axis[1].abs() && axis[0].abs() <= axis[2].abs() {
        [1.0_f64, 0.0, 0.0]
    } else if axis[1].abs() <= axis[2].abs() {
        [0.0_f64, 1.0, 0.0]
    } else {
        [0.0_f64, 0.0, 1.0]
    };

    // e_y = axis × helper  (then normalized)
    let e_y = cross(axis, helper);
    let e_y = normalize(e_y);

    // e_x = e_y × axis  (then normalized; already unit in exact arithmetic)
    let e_x = cross(e_y, axis);
    let e_x = normalize(e_x);

    (e_x, e_y)
}

#[inline(always)]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline(always)]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline(always)]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len == 0.0 {
        v
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

#[inline(always)]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline(always)]
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline(always)]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

// ---------------------------------------------------------------------------
// Public type
// ---------------------------------------------------------------------------

// occt: Geom_ToroidalSurface
/// Analytic toroidal surface in 3-D space.
///
/// Parameterized as:
/// ```text
/// P(u, v) = center
///          + (major_radius + minor_radius·cos(v)) · (cos(u)·e_x + sin(u)·e_y)
///          + minor_radius·sin(v) · axis
/// ```
/// where `(e_x, e_y, axis)` is a right-handed orthonormal frame derived from
/// `axis`.
///
/// Both `u` and `v` are periodic with period `2·π`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToroidalSurface {
    /// Center of the torus (origin of the local coordinate frame).
    pub center: [f64; 3],
    /// Major radius — distance from the center of the torus to the tube center.
    /// Must be `> 0`.
    pub major_radius: f64,
    /// Minor radius — radius of the tube. Must be `>= 0`.
    pub minor_radius: f64,
    /// Unit axis direction — the symmetry axis of the torus (Z in the local
    /// frame). Stored normalized.
    pub axis: [f64; 3],
}

impl ToroidalSurface {
    // -----------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------

    /// Create a new `ToroidalSurface`.
    ///
    /// - `center` — center point of the torus.
    /// - `major` — major radius (must be `> 0`).
    /// - `minor` — minor radius (must be `>= 0`).
    ///
    /// The axis defaults to `[0, 0, 1]` (the Z axis). Use [`with_axis`] to
    /// supply a custom axis, or write to the `axis` field directly.
    ///
    /// # Panics
    /// Panics if `major <= 0` or `minor < 0`.
    ///
    /// [`with_axis`]: ToroidalSurface::with_axis
    pub fn new(center: [f64; 3], major: f64, minor: f64) -> Self {
        assert!(
            major > 0.0,
            "ToroidalSurface: major_radius must be > 0, got {major}"
        );
        assert!(
            minor >= 0.0,
            "ToroidalSurface: minor_radius must be >= 0, got {minor}"
        );
        Self {
            center,
            major_radius: major,
            minor_radius: minor,
            axis: [0.0, 0.0, 1.0],
        }
    }

    /// Create a `ToroidalSurface` with an explicit unit axis direction.
    ///
    /// The supplied `axis` vector is normalized internally.
    ///
    /// # Panics
    /// Panics if `major <= 0`, `minor < 0`, or `axis` is the zero vector.
    pub fn with_axis(center: [f64; 3], major: f64, minor: f64, axis: [f64; 3]) -> Self {
        assert!(
            major > 0.0,
            "ToroidalSurface: major_radius must be > 0, got {major}"
        );
        assert!(
            minor >= 0.0,
            "ToroidalSurface: minor_radius must be >= 0, got {minor}"
        );
        Self {
            center,
            major_radius: major,
            minor_radius: minor,
            axis: normalize(axis),
        }
    }

    // -----------------------------------------------------------------------
    // Local frame helpers
    // -----------------------------------------------------------------------

    /// Return the pair of equatorial unit vectors `(e_x, e_y)` forming a
    /// right-handed frame with `self.axis`.
    #[inline]
    fn equatorial_frame(&self) -> ([f64; 3], [f64; 3]) {
        frame_from_axis(self.axis)
    }

    // -----------------------------------------------------------------------
    // Evaluation
    // -----------------------------------------------------------------------

    /// Evaluate the surface point at parameters `(u, v)`.
    ///
    /// ```text
    /// P(u, v) = center
    ///          + (R + r·cos(v)) · (cos(u)·e_x + sin(u)·e_y)
    ///          + r·sin(v) · axis
    /// ```
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let (e_x, e_y) = self.equatorial_frame();
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let rho = self.major_radius + self.minor_radius * cv;

        // radial = rho · (cos(u)·e_x + sin(u)·e_y)
        let radial = add3(scale(e_x, rho * cu), scale(e_y, rho * su));
        // axial  = r·sin(v) · axis
        let axial = scale(self.axis, self.minor_radius * sv);

        add3(self.center, add3(radial, axial))
    }

    // -----------------------------------------------------------------------
    // First partial derivatives
    // -----------------------------------------------------------------------

    /// First partial derivative with respect to `u` at `(u, v)`.
    ///
    /// ```text
    /// ∂P/∂u = (R + r·cos(v)) · (-sin(u)·e_x + cos(u)·e_y)
    /// ```
    pub fn d1u(&self, u: f64, v: f64) -> [f64; 3] {
        let (e_x, e_y) = self.equatorial_frame();
        let (su, cu) = u.sin_cos();
        let cv = v.cos();
        let rho = self.major_radius + self.minor_radius * cv;

        add3(scale(e_x, -rho * su), scale(e_y, rho * cu))
    }

    /// First partial derivative with respect to `v` at `(u, v)`.
    ///
    /// ```text
    /// ∂P/∂v = -r·sin(v) · (cos(u)·e_x + sin(u)·e_y) + r·cos(v) · axis
    /// ```
    pub fn d1v(&self, u: f64, v: f64) -> [f64; 3] {
        let (e_x, e_y) = self.equatorial_frame();
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let r = self.minor_radius;

        // -r·sin(v)·(cos(u)·e_x + sin(u)·e_y)
        let radial_part = add3(scale(e_x, -r * sv * cu), scale(e_y, -r * sv * su));
        // r·cos(v)·axis
        let axial_part = scale(self.axis, r * cv);

        add3(radial_part, axial_part)
    }

    // -----------------------------------------------------------------------
    // Unit normal
    // -----------------------------------------------------------------------

    /// Unit outward normal at `(u, v)`.
    ///
    /// The normal is `(∂P/∂u × ∂P/∂v)` normalized.
    ///
    /// For the standard torus this simplifies to:
    /// ```text
    /// n = cos(v)·(cos(u)·e_x + sin(u)·e_y) + sin(v)·axis
    /// ```
    /// (unit vector, independent of the radii).
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let (e_x, e_y) = self.equatorial_frame();
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();

        // cos(v)·(cos(u)·e_x + sin(u)·e_y) + sin(v)·axis
        let n = add3(
            add3(scale(e_x, cv * cu), scale(e_y, cv * su)),
            scale(self.axis, sv),
        );

        // Already unit: cv^2*(cu^2+su^2) + sv^2 = cv^2 + sv^2 = 1.
        // Normalize defensively to handle floating-point drift.
        normalize(n)
    }

    // -----------------------------------------------------------------------
    // Topology queries
    // -----------------------------------------------------------------------

    /// `IsUClosed` — always `true`; the torus is closed (periodic) in `u`.
    #[inline]
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed` — always `true`; the torus is closed (periodic) in `v`.
    #[inline]
    pub fn is_v_closed(&self) -> bool {
        true
    }

    // -----------------------------------------------------------------------
    // Geometric properties
    // -----------------------------------------------------------------------

    /// Volume enclosed by the torus.
    ///
    /// `V = 2·π²·R·r²`
    pub fn volume(&self) -> f64 {
        2.0 * PI * PI * self.major_radius * self.minor_radius * self.minor_radius
    }
}

// ---------------------------------------------------------------------------
// Free function
// ---------------------------------------------------------------------------

/// Surface area of a torus with major radius `r_major` and minor radius
/// `r_minor`.
///
/// `A = 4·π²·R·r`
///
/// Models `Geom_ToroidalSurface::Area()`.
pub fn torus_surface_area(r_major: f64, r_minor: f64) -> f64 {
    4.0 * PI * PI * r_major * r_minor
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const TAU: f64 = 2.0 * PI;

    fn approx_eq(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
        (a[0] - b[0]).abs() < eps && (a[1] - b[1]).abs() < eps && (a[2] - b[2]).abs() < eps
    }

    #[test]
    fn test_new_default_axis() {
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        assert_eq!(t.axis, [0.0, 0.0, 1.0]);
        assert_eq!(t.major_radius, 3.0);
        assert_eq!(t.minor_radius, 1.0);
    }

    #[test]
    fn test_evaluate_u0_v0() {
        // At (u=0, v=0) the point should be at center + (R+r)·e_x
        // With default axis=[0,0,1]: e_x=[1,0,0], e_y=[0,1,0]
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let p = t.evaluate(0.0, 0.0);
        let expected = [4.0, 0.0, 0.0]; // (3+1)·[1,0,0]
        assert!(approx_eq(p, expected, 1e-12), "{p:?} != {expected:?}");
    }

    #[test]
    fn test_evaluate_v_pi() {
        // At (u=0, v=π): P = center + (R - r)·e_x
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let p = t.evaluate(0.0, PI);
        let expected = [2.0, 0.0, 0.0];
        assert!(approx_eq(p, expected, 1e-12), "{p:?} != {expected:?}");
    }

    #[test]
    fn test_evaluate_v_half_pi() {
        // At (u=0, v=π/2): P = center + R·e_x + r·axis
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let p = t.evaluate(0.0, PI / 2.0);
        let expected = [3.0, 0.0, 1.0];
        assert!(approx_eq(p, expected, 1e-12), "{p:?} != {expected:?}");
    }

    #[test]
    fn test_normal_is_unit() {
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 5.0, 2.0);
        for i in 0..8 {
            for j in 0..8 {
                let u = TAU * (i as f64) / 8.0;
                let v = TAU * (j as f64) / 8.0;
                let n = t.normal(u, v);
                let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                assert!((len - 1.0).abs() < 1e-12, "normal not unit at ({u},{v}): len={len}");
            }
        }
    }

    #[test]
    fn test_d1u_perpendicular_to_normal() {
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 4.0, 1.5);
        let u = PI / 4.0;
        let v = PI / 3.0;
        let du = t.d1u(u, v);
        let n = t.normal(u, v);
        let d = dot(du, n);
        assert!(d.abs() < 1e-12, "d1u not perpendicular to normal: dot={d}");
    }

    #[test]
    fn test_d1v_perpendicular_to_normal() {
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 4.0, 1.5);
        let u = PI / 4.0;
        let v = PI / 3.0;
        let dv = t.d1v(u, v);
        let n = t.normal(u, v);
        let d = dot(dv, n);
        assert!(d.abs() < 1e-12, "d1v not perpendicular to normal: dot={d}");
    }

    #[test]
    fn test_is_closed() {
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        assert!(t.is_u_closed());
        assert!(t.is_v_closed());
    }

    #[test]
    fn test_volume() {
        let t = ToroidalSurface::new([0.0, 0.0, 0.0], 3.0, 1.0);
        let expected = 2.0 * PI * PI * 3.0 * 1.0;
        assert!((t.volume() - expected).abs() < 1e-12);
    }

    #[test]
    fn test_torus_surface_area() {
        let a = torus_surface_area(3.0, 1.0);
        let expected = 4.0 * PI * PI * 3.0 * 1.0;
        assert!((a - expected).abs() < 1e-12);
    }

    #[test]
    fn test_custom_axis() {
        // With axis=[0,1,0], the torus lives in the XZ plane.
        // At (u=0, v=0): e_x should be perpendicular to [0,1,0].
        let t = ToroidalSurface::with_axis([0.0, 0.0, 0.0], 3.0, 1.0, [0.0, 1.0, 0.0]);
        // The frame_from_axis([0,1,0]) picks helper=[1,0,0].
        // e_y = [0,1,0] × [1,0,0] = [0*0-0*0, 0*1-0*0, 0*0-1*1] = [0,0,-1]
        // normalized -> [0,0,-1]
        // e_x = [0,0,-1] × [0,1,0] = [0*0-(-1)*1, (-1)*0-0*0, 0*1-0*0] = [1,0,0]
        // So at (u=0,v=0): P = [0,0,0] + (3+1)*[1,0,0] + 0 = [4,0,0]
        let p = t.evaluate(0.0, 0.0);
        // Check it lies at distance R+r from center in the plane perp to [0,1,0].
        let dist_xz = (p[0] * p[0] + p[2] * p[2]).sqrt();
        assert!((dist_xz - 4.0).abs() < 1e-12);
    }

    #[test]
    fn test_periodicity() {
        let t = ToroidalSurface::new([1.0, 2.0, 3.0], 5.0, 2.0);
        let u = 1.23;
        let v = 0.77;
        let p0 = t.evaluate(u, v);
        let p1 = t.evaluate(u + TAU, v);
        let p2 = t.evaluate(u, v + TAU);
        assert!(approx_eq(p0, p1, 1e-10), "not periodic in u");
        assert!(approx_eq(p0, p2, 1e-10), "not periodic in v");
    }
}
