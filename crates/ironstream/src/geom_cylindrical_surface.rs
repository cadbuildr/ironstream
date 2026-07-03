//! `CylindricalSurface` — pure-Rust zero-dependency stub modelled after
//! OpenCascade's `Geom_CylindricalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_CylindricalSurface.hxx`).
//!
//! A cylindrical surface is defined by:
//! - an `origin` point (the base-circle centre / axis location),
//! - a unit `axis` vector (the cylinder axis, i.e. OCCT's Z direction),
//! - a `radius` R > 0.
//!
//! The surface is parameterised as:
//! ```text
//! P(U, V) = origin
//!         + R·cos(U)·xdir
//!         + R·sin(U)·ydir
//!         + V·axis
//! ```
//! where `xdir` / `ydir` are any right-hand orthonormal pair perpendicular to
//! `axis`, U ∈ [0, 2π) is the angular parameter (periodic) and V ∈ ℝ is the
//! height along the axis.
//!
//! Because this is a self-contained stub no external crates are used; only
//! `std::f64::consts::PI` and the `f64` trig methods.

use std::f64::consts::PI;

// ──────────────────────────────── helpers ────────────────────────────────────

/// Build a right-hand orthonormal pair `(xdir, ydir)` perpendicular to `axis`.
///
/// The axis is assumed to be (or very close to) a unit vector.  We pick a
/// "reference" vector that is not (anti-)parallel to `axis`, cross it with
/// `axis` to get `xdir`, then cross `axis` with `xdir` to get `ydir`.
fn orthonormal_pair(axis: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    // Choose a reference vector that is not collinear with axis.
    let reference = if axis[0].abs() <= axis[1].abs() && axis[0].abs() <= axis[2].abs() {
        [1.0, 0.0, 0.0]
    } else if axis[1].abs() <= axis[2].abs() {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };

    let xdir = cross(reference, axis);
    let xdir = normalize(xdir);
    let ydir = cross(axis, xdir);
    (xdir, ydir)
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
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

#[inline]
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

// ─────────────────────────────── main type ───────────────────────────────────

// occt: Geom_CylindricalSurface
/// An infinite analytic cylindrical surface in 3D space.
///
/// The parameterisation is:
/// ```text
/// P(U, V) = origin + R·cos(U)·xdir + R·sin(U)·ydir + V·axis
/// ```
/// where `xdir` / `ydir` are computed automatically as a right-hand
/// orthonormal frame perpendicular to `axis`.
///
/// - `U` is periodic with period `2·π`; the surface is closed in U.
/// - `V` is unbounded; the surface is open in V.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalSurface {
    /// The base-circle centre (OCCT: axis location / `gp_Ax3` origin).
    pub origin: [f64; 3],
    /// The cylinder axis direction — should be a unit vector
    /// (OCCT: Z direction of the local coordinate system).
    pub axis: [f64; 3],
    /// The cylinder radius (must be ≥ 0).
    pub radius: f64,
}

impl CylindricalSurface {
    // ──────────────────────────── constructor ────────────────────────────────

    /// Construct a new cylindrical surface.
    ///
    /// Mirrors `Geom_CylindricalSurface(const gp_Ax3&, Standard_Real)`.
    ///
    /// # Panics
    /// Panics if `radius < 0`.
    pub fn new(origin: [f64; 3], axis: [f64; 3], radius: f64) -> Self {
        assert!(
            radius >= 0.0,
            "CylindricalSurface: radius must be >= 0, got {radius}"
        );
        Self {
            origin,
            axis: normalize(axis),
            radius,
        }
    }

    // ─────────────────────── topology / period queries ───────────────────────

    /// `IsUClosed()` — `true`; the cylinder is closed (seam at U = 0 / 2π).
    ///
    /// Mirrors `Geom_CylindricalSurface::IsUClosed`.
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed()` — `false`; the cylinder is unbounded along the axis.
    ///
    /// Mirrors `Geom_CylindricalSurface::IsVClosed`.
    pub fn is_v_closed(&self) -> bool {
        false
    }

    /// `Radius()` — the cylinder radius.
    ///
    /// Mirrors `Geom_CylindricalSurface::Radius`.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    // ──────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` — the point on the cylinder surface at parameters `(u, v)`.
    ///
    /// ```text
    /// P(U, V) = origin + R·cos(U)·xdir + R·sin(U)·ydir + V·axis
    /// ```
    ///
    /// Mirrors `Geom_CylindricalSurface::D0` / `Value`.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let (xdir, ydir) = orthonormal_pair(self.axis);
        let (s, c) = u.sin_cos();
        let mut p = self.origin;
        p = add(p, scale(xdir, self.radius * c));
        p = add(p, scale(ydir, self.radius * s));
        p = add(p, scale(self.axis, v));
        p
    }

    /// `Normal(U, V)` — the outward unit normal at `(u, v)`.
    ///
    /// For a cylinder the outward normal at angular parameter U is simply the
    /// radial direction in the base plane:
    /// ```text
    /// N(U, V) = cos(U)·xdir + sin(U)·ydir
    /// ```
    /// (independent of V).
    ///
    /// Mirrors `Geom_CylindricalSurface::Normal`.
    pub fn normal(&self, u: f64, _v: f64) -> [f64; 3] {
        let (xdir, ydir) = orthonormal_pair(self.axis);
        let (s, c) = u.sin_cos();
        add(scale(xdir, c), scale(ydir, s))
    }

    /// `∂P/∂U` — first partial derivative with respect to `U`.
    ///
    /// ```text
    /// dP/dU = -R·sin(U)·xdir + R·cos(U)·ydir
    /// ```
    ///
    /// Mirrors the DU component of `Geom_CylindricalSurface::D1`.
    pub fn d1u(&self, u: f64, _v: f64) -> [f64; 3] {
        let (xdir, ydir) = orthonormal_pair(self.axis);
        let (s, c) = u.sin_cos();
        add(scale(xdir, -self.radius * s), scale(ydir, self.radius * c))
    }

    /// `∂P/∂V` — first partial derivative with respect to `V`.
    ///
    /// ```text
    /// dP/dV = axis
    /// ```
    ///
    /// Mirrors the DV component of `Geom_CylindricalSurface::D1`.
    pub fn d1v(&self, _u: f64, _v: f64) -> [f64; 3] {
        self.axis
    }
}

// ─────────────────────────── free functions ──────────────────────────────────

/// Volume of a finite right circular cylinder.
///
/// `V = π · r² · h`
pub fn cylinder_volume(r: f64, h: f64) -> f64 {
    PI * r * r * h
}

/// Total surface area of a finite right circular cylinder (lateral + two caps).
///
/// `A = 2·π·r·(r + h)`
pub fn cylinder_surface_area(r: f64, h: f64) -> f64 {
    2.0 * PI * r * (r + h)
}

// ───────────────────────────────── tests ─────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        a.iter()
            .zip(b.iter())
            .all(|(x, y)| (x - y).abs() < 1.0e-10)
    }

    #[test]
    fn evaluate_at_zero() {
        // For a Z-axis cylinder the xdir should be X or Y; the point at U=0
        // must be at (origin + radius * xdir).
        let s = CylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
        let p = s.evaluate(0.0, 0.0);
        // The radius-direction component must have magnitude == radius.
        let r2 = p[0] * p[0] + p[1] * p[1];
        assert!((r2.sqrt() - 3.0).abs() < 1.0e-10);
        assert!(p[2].abs() < 1.0e-10);
    }

    #[test]
    fn evaluate_height() {
        let s = CylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
        let p = s.evaluate(0.0, 5.0);
        assert!((p[2] - 5.0).abs() < 1.0e-10);
    }

    #[test]
    fn normal_is_unit_radial() {
        let s = CylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0);
        let n = s.normal(PI / 4.0, 1.0);
        let mag = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!((mag - 1.0).abs() < 1.0e-10);
        // Normal should have no Z component for a Z-axis cylinder.
        assert!(n[2].abs() < 1.0e-10);
    }

    #[test]
    fn d1v_equals_axis() {
        let axis = [0.0, 0.0, 1.0];
        let s = CylindricalSurface::new([0.0, 0.0, 0.0], axis, 1.0);
        assert!(approx_eq(s.d1v(0.0, 0.0), axis));
    }

    #[test]
    fn closed_in_u_open_in_v() {
        let s = CylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
        assert!(s.is_u_closed());
        assert!(!s.is_v_closed());
    }

    #[test]
    fn radius_accessor() {
        let s = CylindricalSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 7.5);
        assert!((s.radius() - 7.5).abs() < 1.0e-15);
    }

    #[test]
    fn cylinder_volume_formula() {
        // V = π·r²·h
        let v = cylinder_volume(1.0, 1.0);
        assert!((v - PI).abs() < 1.0e-10);
    }

    #[test]
    fn cylinder_surface_area_formula() {
        // A = 2·π·r·(r + h), with r=1, h=0: A = 2π (two unit circles)
        let a = cylinder_surface_area(1.0, 0.0);
        assert!((a - 2.0 * PI).abs() < 1.0e-10);
    }
}
