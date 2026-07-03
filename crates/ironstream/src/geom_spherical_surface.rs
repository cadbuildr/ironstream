//! `SphericalSurface` — a zero-dependency, pure-Rust stub for an analytic
//! spherical surface in 3-D space, modelled after OpenCascade's
//! `Geom_SphericalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_SphericalSurface.hxx`).
//!
//! The surface is parameterised as:
//! ```text
//! P(u, v) = center + r·cos(v)·cos(u)·X
//!                  + r·cos(v)·sin(u)·Y
//!                  + r·sin(v)·Z
//! ```
//! where `X = [1,0,0]`, `Y = [0,1,0]`, `Z = [0,0,1]` are the world axes,
//! `u ∈ [0, 2π)` is the longitude, and `v ∈ [-π/2, π/2]` is the latitude.
//!
//! No external crates are used; only `std` (f64 trig and `std::f64::consts`).

use std::f64::consts::PI;

// occt: Geom_SphericalSurface
/// A spherical surface defined by a centre point and a radius.
///
/// Parameterisation follows `Geom_SphericalSurface`:
/// ```text
/// P(u, v) = center + r·cos(v)·cos(u)·X + r·cos(v)·sin(u)·Y + r·sin(v)·Z
/// ```
/// - `u` (longitude) is periodic with period `2·π`; the surface is closed in u.
/// - `v` (latitude) ranges in `[-π/2, π/2]`; the surface is closed in v (poles).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphericalSurface {
    /// Centre of the sphere in world coordinates.
    pub center: [f64; 3],
    /// Sphere radius (must be > 0).
    pub radius: f64,
}

impl SphericalSurface {
    // ─────────────────────────── constructor ────────────────────────────────

    /// Create a new `SphericalSurface` from a centre point and a radius.
    ///
    /// Mirrors `Geom_SphericalSurface(const gp_Ax3&, Standard_Real)`.
    ///
    /// # Panics
    /// Panics if `radius <= 0`.
    #[inline]
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        assert!(
            radius > 0.0,
            "SphericalSurface: radius must be > 0, got {radius}"
        );
        Self { center, radius }
    }

    // ─────────────────────────── surface point ──────────────────────────────

    /// `P(u, v)` — point on the sphere surface at longitude `u` and latitude `v`.
    ///
    /// ```text
    /// P(u, v) = center + r·cos(v)·cos(u)·X + r·cos(v)·sin(u)·Y + r·sin(v)·Z
    /// ```
    #[inline]
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let cv = v.cos();
        let sv = v.sin();
        let cu = u.cos();
        let su = u.sin();
        let r = self.radius;
        [
            self.center[0] + r * cv * cu,
            self.center[1] + r * cv * su,
            self.center[2] + r * sv,
        ]
    }

    // ─────────────────────────── unit normal ────────────────────────────────

    /// Outward unit normal at `(u, v)`.
    ///
    /// For a sphere centred at the origin the outward normal is simply
    /// `(P - center) / r`, which equals `[cos(v)·cos(u), cos(v)·sin(u), sin(v)]`.
    #[inline]
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let cv = v.cos();
        let sv = v.sin();
        [cv * u.cos(), cv * u.sin(), sv]
    }

    // ─────────────────────────── first derivatives ──────────────────────────

    /// First partial derivative with respect to `u` (longitude).
    ///
    /// `∂P/∂u = r·cos(v)·(-sin(u)·X + cos(u)·Y)`
    #[inline]
    pub fn d1u(&self, u: f64, v: f64) -> [f64; 3] {
        let cv = v.cos();
        let r = self.radius;
        [-r * cv * u.sin(), r * cv * u.cos(), 0.0]
    }

    /// First partial derivative with respect to `v` (latitude).
    ///
    /// `∂P/∂v = r·(-sin(v)·cos(u)·X - sin(v)·sin(u)·Y + cos(v)·Z)`
    #[inline]
    pub fn d1v(&self, u: f64, v: f64) -> [f64; 3] {
        let sv = v.sin();
        let cv = v.cos();
        let r = self.radius;
        [-r * sv * u.cos(), -r * sv * u.sin(), r * cv]
    }

    // ─────────────────────────── topology queries ────────────────────────────

    /// `IsUClosed` — `true`; the sphere is closed (and periodic) in longitude.
    #[inline]
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed` — `true`; the sphere is closed in latitude (north/south poles).
    #[inline]
    pub fn is_v_closed(&self) -> bool {
        true
    }

    // ─────────────────────────── geometry ───────────────────────────────────

    /// Total surface area of the sphere: `4·π·r²`.
    #[inline]
    pub fn area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    /// Volume enclosed by the sphere: `(4/3)·π·r³`.
    #[inline]
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.radius * self.radius * self.radius
    }
}

// ──────────────────────── free function ─────────────────────────────────────

/// Compute the `(u, v)` parameters on a sphere (centre `center`, radius `r`)
/// corresponding to the surface point `pt`.
///
/// Returns `[u, v]` where:
/// - `u ∈ [0, 2π)` is the longitude (`atan2` of the projected-to-equator coords),
/// - `v ∈ [-π/2, π/2]` is the latitude (`asin` of the normalised Z offset).
///
/// If `pt` coincides with `center` (or `r` is 0) the result is `[0.0, 0.0]`.
pub fn sphere_uv_from_point(center: [f64; 3], r: f64, pt: [f64; 3]) -> [f64; 2] {
    let dx = pt[0] - center[0];
    let dy = pt[1] - center[1];
    let dz = pt[2] - center[2];

    let len = f64::sqrt(dx * dx + dy * dy + dz * dz);
    if len == 0.0 {
        return [0.0, 0.0];
    }

    // Normalise to unit vector on the sphere.
    let inv = 1.0 / len;
    let nx = dx * inv;
    let ny = dy * inv;
    let nz = dz * inv;

    // Clamp nz to [-1, 1] to guard against floating-point overshoot before asin.
    let nz_clamped = nz.clamp(-1.0, 1.0);

    let v = f64::asin(nz_clamped);

    // Longitude: atan2(ny, nx), shifted to [0, 2π).
    let u_raw = f64::atan2(ny, nx);
    let u = if u_raw < 0.0 { u_raw + 2.0 * PI } else { u_raw };

    // r is accepted for API symmetry with evaluate/normal but not needed beyond
    // the normalisation above (which uses `len` directly).
    let _ = r;

    [u, v]
}

// ─────────────────────────────── tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn vec_approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        approx_eq(a[0], b[0]) && approx_eq(a[1], b[1]) && approx_eq(a[2], b[2])
    }

    #[test]
    fn evaluate_north_pole() {
        let s = SphericalSurface::new([0.0, 0.0, 0.0], 2.0);
        let p = s.evaluate(0.0, PI / 2.0);
        // At v = π/2 the point should be [0, 0, r].
        assert!(vec_approx_eq(p, [0.0, 0.0, 2.0]));
    }

    #[test]
    fn evaluate_equator_x() {
        let s = SphericalSurface::new([1.0, 2.0, 3.0], 5.0);
        let p = s.evaluate(0.0, 0.0);
        // u=0, v=0 → center + [r, 0, 0]
        assert!(vec_approx_eq(p, [6.0, 2.0, 3.0]));
    }

    #[test]
    fn normal_is_unit() {
        let s = SphericalSurface::new([0.0, 0.0, 0.0], 3.0);
        let n = s.normal(1.0, 0.5);
        let len2 = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
        assert!((len2 - 1.0).abs() < EPS);
    }

    #[test]
    fn area_and_volume() {
        let s = SphericalSurface::new([0.0, 0.0, 0.0], 1.0);
        assert!((s.area() - 4.0 * PI).abs() < EPS);
        assert!((s.volume() - (4.0 / 3.0) * PI).abs() < EPS);
    }

    #[test]
    fn closed_flags() {
        let s = SphericalSurface::new([0.0, 0.0, 0.0], 1.0);
        assert!(s.is_u_closed());
        assert!(s.is_v_closed());
    }

    #[test]
    fn roundtrip_uv() {
        let center = [1.0, -2.0, 3.0];
        let r = 4.0;
        let s = SphericalSurface::new(center, r);

        let u0 = 1.2_f64;
        let v0 = 0.4_f64;
        let pt = s.evaluate(u0, v0);
        let [u1, v1] = sphere_uv_from_point(center, r, pt);

        assert!(approx_eq(u1, u0), "u: {u1} != {u0}");
        assert!(approx_eq(v1, v0), "v: {v1} != {v0}");
    }

    #[test]
    fn sphere_uv_north_pole() {
        // At the north pole u is conventionally 0.
        let [u, v] = sphere_uv_from_point([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0]);
        assert!(approx_eq(v, PI / 2.0), "v={v}");
        // u is 0 because atan2(0,0) == 0.
        assert!(approx_eq(u, 0.0), "u={u}");
    }

    #[test]
    #[should_panic]
    fn zero_radius_panics() {
        SphericalSurface::new([0.0, 0.0, 0.0], 0.0);
    }
}
