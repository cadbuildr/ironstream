//! `Geom_SphericalSurface` — analytic spherical surface in 3D space,
//! mirroring OpenCascade's `Geom_SphericalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_SphericalSurface.hxx`).
//!
//! A spherical surface is defined by:
//! - A right-handed local coordinate system ([`Ax3`]) whose origin is the
//!   center of the sphere.
//! - A radius `R > 0`.
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = O + R·cos(V)·cos(U)·XDir
//!             + R·cos(V)·sin(U)·YDir
//!             + R·sin(V)·ZDir
//! ```
//! where:
//! - U ∈ [0, 2π) is the longitude (periodic in U),
//! - V ∈ [-π/2, π/2] is the latitude.
//!
//! The surface is closed (and periodic) in U and closed (but not periodic) in V.
//!
//! References:
//! - OCCT `Geom_SphericalSurface` (TKG3d)
//! - ISO 10303-42, spherical surface definition

use crate::geom_circle::GeomCircle;
use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use std::f64::consts::PI;

// occt-ref: Geom_SphericalSurface
/// `Geom_SphericalSurface` — an analytic spherical surface in 3D space.
///
/// The surface is parameterized by `(U, V)` as:
/// `P(U, V) = O + R·cos(V)·cos(U)·XDir + R·cos(V)·sin(U)·YDir + R·sin(V)·ZDir`
///
/// where `O`, `XDir`, `YDir`, `ZDir` are the origin and axes of the local
/// coordinate system, and `R` is the radius.
///
/// - U is periodic with period `2·π`; the sphere is closed in U.
/// - V ranges in `[-π/2, π/2]`; the sphere is closed in V (poles at `±π/2`).
#[derive(Clone, Copy, Debug)]
pub struct GeomSphericalSurface {
    /// The local coordinate system (`gp_Ax3`): origin = sphere center,
    /// Z = north-pole axis, X/Y span the equatorial plane.
    pos: Ax3,
    /// The sphere radius (must be > 0).
    radius: f64,
}

impl GeomSphericalSurface {
    // ─────────────────────────── constructors ───────────────────────────────

    /// `Geom_SphericalSurface(const gp_Ax3& A3, const Standard_Real Radius)`.
    ///
    /// - `a3` defines the local coordinate system: its origin is the sphere
    ///   center, Z direction is the north-pole axis.
    /// - `radius` must be > 0 (OCCT raises `Standard_ConstructionError` if ≤ 0).
    ///
    /// # Panics
    /// Panics if `radius <= 0`.
    pub fn new(a3: Ax3, radius: f64) -> Self {
        assert!(
            radius > 0.0,
            "Geom_SphericalSurface: Radius must be > 0, got {radius}"
        );
        Self { pos: a3, radius }
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// `SetRadius(R)` — change the radius.
    ///
    /// # Panics
    /// Panics if `r <= 0`.
    pub fn set_radius(&mut self, r: f64) {
        assert!(
            r > 0.0,
            "Geom_SphericalSurface::SetRadius: R must be > 0, got {r}"
        );
        self.radius = r;
    }

    /// `SetPosition(A3)` — change the local coordinate system.
    pub fn set_position(&mut self, a3: Ax3) {
        self.pos = a3;
    }

    // ─────────────────────────── getters ────────────────────────────────────

    /// `Radius()` — the sphere radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// `Position()` — the local coordinate system (`gp_Ax3`).
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `Location()` — the center of the sphere (origin of the local frame).
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// `Axis()` — the main axis of the sphere (Z axis of the local frame,
    /// through the north pole).
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    // ──────────────────── parameter / topology queries ───────────────────────

    /// `UReversedParameter(U)` — parameter on the U-reversed surface: `2·π - U`.
    pub fn u_reversed_parameter(&self, u: f64) -> f64 {
        2.0 * PI - u
    }

    /// `VReversedParameter(V)` — parameter on the V-reversed surface: `-V`.
    pub fn v_reversed_parameter(&self, v: f64) -> f64 {
        -v
    }

    /// `UPeriod()` — `2·π` (the sphere is periodic in U).
    pub fn u_period(&self) -> f64 {
        2.0 * PI
    }

    /// `IsUClosed()` — `true`; the sphere is closed in U (seam at U=0/2π).
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed()` — `true`; the sphere is closed in V (poles at V = ±π/2).
    pub fn is_v_closed(&self) -> bool {
        true
    }

    /// `IsUPeriodic()` — `true`; U has period `2·π`.
    pub fn is_u_periodic(&self) -> bool {
        true
    }

    /// `IsVPeriodic()` — `false`; V is bounded (latitude).
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// First U parameter: `0.0`.
    pub fn u_first_parameter(&self) -> f64 {
        0.0
    }

    /// Last U parameter: `2·π`.
    pub fn u_last_parameter(&self) -> f64 {
        2.0 * PI
    }

    /// First V parameter: `-π/2` (south pole).
    pub fn v_first_parameter(&self) -> f64 {
        -PI / 2.0
    }

    /// Last V parameter: `+π/2` (north pole).
    pub fn v_last_parameter(&self) -> f64 {
        PI / 2.0
    }

    // ────────────────────────── iso-curves ──────────────────────────────────

    /// `UIso(U)` — iso-curve for constant longitude U.
    ///
    /// This is a great circle (meridian) in the plane spanned by
    /// `cos(U)·XDir + sin(U)·YDir` and `ZDir`, centered at the sphere center
    /// with radius `R`.
    ///
    /// In OCCT the UIso of a sphere is a `Geom_Circle`. The circle lies in the
    /// plane whose X direction is `cos(U)·XDir + sin(U)·YDir` and whose Z
    /// direction is `-sin(U)·XDir + cos(U)·YDir` (so that the circle is
    /// oriented correctly — its parameter V=0 lands at the equator).
    ///
    /// More precisely:
    /// - Center: sphere center `O`.
    /// - X direction of circle frame: `cos(U)·XDir + sin(U)·YDir` (equatorial radial).
    /// - Y direction of circle frame: `ZDir` (toward north pole).
    /// - Z direction of circle frame: `-sin(U)·XDir + cos(U)·YDir` (outward normal of meridian plane).
    /// - The circle is parameterized so that parameter 0 lands on the equator
    ///   in the `+X` radial direction and `π/2` at the north pole.
    pub fn u_iso(&self, u: f64) -> GeomCircle {
        let (su, cu) = u.sin_cos();
        // Radial direction at longitude U in the equatorial plane.
        let x_radial = self.pos.x_dir * cu + self.pos.y_dir * su;
        // Tangential direction (perpendicular to x_radial in the equatorial plane).
        let z_normal = self.pos.x_dir * (-su) + self.pos.y_dir * cu;
        // Y of the meridian circle: ZDir (toward north pole).
        let y_toward_pole = self.pos.z_dir;
        let frame = Ax3 {
            location: self.pos.location,
            x_dir: x_radial,
            y_dir: y_toward_pole,
            z_dir: z_normal,
        };
        GeomCircle::from_ax3(frame, self.radius)
    }

    /// `VIso(V)` — iso-curve for constant latitude V.
    ///
    /// This is a circle (parallel) at latitude V:
    /// - Center: `O + R·sin(V)·ZDir`
    /// - Radius: `R·cos(V)`
    /// - The circle lies in a plane parallel to the XY plane of the local frame.
    pub fn v_iso(&self, v: f64) -> GeomCircle {
        let (sv, cv) = v.sin_cos();
        let center = self.pos.location + self.pos.z_dir * (self.radius * sv);
        let frame = Ax3 {
            location: center,
            x_dir: self.pos.x_dir,
            y_dir: self.pos.y_dir,
            z_dir: self.pos.z_dir,
        };
        let r = (self.radius * cv).abs();
        GeomCircle::from_ax3(frame, r)
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` / `D0` — the point on the sphere surface.
    ///
    /// `P(U, V) = O + R·cos(V)·cos(U)·XDir + R·cos(V)·sin(U)·YDir + R·sin(V)·ZDir`.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        self.pos.location
            + self.pos.x_dir * (self.radius * cv * cu)
            + self.pos.y_dir * (self.radius * cv * su)
            + self.pos.z_dir * (self.radius * sv)
    }

    /// `D0(U, V)` — alias for [`value`](Self::value).
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V)` — point and first-order partial derivatives.
    ///
    /// Returns `(P, DU, DV)` where:
    /// - `DU = ∂P/∂U = R·cos(V)·(-sin(U)·XDir + cos(U)·YDir)`
    /// - `DV = ∂P/∂V = R·(-sin(V)·cos(U)·XDir - sin(V)·sin(U)·YDir + cos(V)·ZDir)`
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let p = self.pos.location
            + self.pos.x_dir * (self.radius * cv * cu)
            + self.pos.y_dir * (self.radius * cv * su)
            + self.pos.z_dir * (self.radius * sv);
        // ∂P/∂U = R·cos(V)·(-sin(U)·X + cos(U)·Y)
        let du = self.pos.x_dir * (-self.radius * cv * su)
            + self.pos.y_dir * (self.radius * cv * cu);
        // ∂P/∂V = R·(-sin(V)·cos(U)·X - sin(V)·sin(U)·Y + cos(V)·Z)
        let dv = self.pos.x_dir * (-self.radius * sv * cu)
            + self.pos.y_dir * (-self.radius * sv * su)
            + self.pos.z_dir * (self.radius * cv);
        (p, du, dv)
    }

    /// `D2(U, V)` — point, first- and second-order derivatives.
    ///
    /// Returns `(P, DU, DV, D2U, D2V, D2UV)` where:
    /// - `D2U  = ∂²P/∂U²   = R·cos(V)·(-cos(U)·XDir - sin(U)·YDir)`
    /// - `D2V  = ∂²P/∂V²   = -R·(cos(V)·cos(U)·XDir + cos(V)·sin(U)·YDir + sin(V)·ZDir)` = `-P` relative to center
    /// - `D2UV = ∂²P/∂U∂V  = R·sin(V)·(sin(U)·XDir - cos(U)·YDir)`
    pub fn d2(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let p = self.pos.location
            + self.pos.x_dir * (self.radius * cv * cu)
            + self.pos.y_dir * (self.radius * cv * su)
            + self.pos.z_dir * (self.radius * sv);
        let du = self.pos.x_dir * (-self.radius * cv * su)
            + self.pos.y_dir * (self.radius * cv * cu);
        let dv = self.pos.x_dir * (-self.radius * sv * cu)
            + self.pos.y_dir * (-self.radius * sv * su)
            + self.pos.z_dir * (self.radius * cv);
        // D2U = ∂/∂U [DU] = R·cos(V)·(-cos(U)·X - sin(U)·Y)
        let d2u = self.pos.x_dir * (-self.radius * cv * cu)
            + self.pos.y_dir * (-self.radius * cv * su);
        // D2V = ∂/∂V [DV] = R·(-cos(V)·cos(U)·X - cos(V)·sin(U)·Y - sin(V)·Z)
        let d2v = self.pos.x_dir * (-self.radius * cv * cu)
            + self.pos.y_dir * (-self.radius * cv * su)
            + self.pos.z_dir * (-self.radius * sv);
        // D2UV = ∂/∂V [DU] = R·sin(V)·(sin(U)·X - cos(U)·Y)
        let d2uv = self.pos.x_dir * (self.radius * sv * su)
            + self.pos.y_dir * (-self.radius * sv * cu);
        (p, du, dv, d2u, d2v, d2uv)
    }

    /// `DN(U, V, Nu, Nv)` — derivative of order `(nu, nv)`.
    ///
    /// The parametric form factors as:
    ///
    /// - The U-part at fixed V: `R·cos(V)·(cos(U)·X + sin(U)·Y)`,
    ///   whose `nu`-th derivative is `R·cos(V)·(cos(U+nu·π/2)·X + sin(U+nu·π/2)·Y)`.
    /// - The V-part at fixed U:
    ///   - `X` component: `R·cos(V)·cos(U)` → `nu`-th V-derivative picks up
    ///     a factor of `cos(V + nv·π/2)`
    ///   - `Y` component: `R·cos(V)·sin(U)` → same V-derivative
    ///   - `Z` component: `R·sin(V)` → `nu`-th V-derivative: `R·sin(V + nv·π/2)`
    ///
    /// General formula for mixed `(nu, nv)`:
    /// ```text
    /// D^(nu+nv)P / DU^nu DV^nv =
    ///     R·cos(V + nv·π/2)·(cos(U + nu·π/2)·X + sin(U + nu·π/2)·Y)   [XY terms]
    ///   + R·sin(V + nv·π/2)·Z                                              [Z term, only if nu==0]
    /// ```
    ///
    /// The Z component is only present when `nu == 0` (differentiating in U
    /// removes the Z dependence since `∂/∂U [sin(V)·Z] = 0`).
    ///
    /// # Panics
    /// Panics if `nu < 0`, `nv < 0`, or `nu + nv < 1`.
    pub fn dn(&self, u: f64, v: f64, nu: i32, nv: i32) -> Vec3 {
        assert!(nu >= 0, "Geom_SphericalSurface::DN: Nu must be >= 0");
        assert!(nv >= 0, "Geom_SphericalSurface::DN: Nv must be >= 0");
        assert!(
            nu + nv >= 1,
            "Geom_SphericalSurface::DN: Nu + Nv must be >= 1"
        );

        let phase_u = u + (nu as f64) * (PI / 2.0);
        let phase_v = v + (nv as f64) * (PI / 2.0);

        // sin_cos() returns (sin, cos)
        let (sin_pu, cos_pu) = phase_u.sin_cos();
        let (sin_pv, cos_pv) = phase_v.sin_cos();

        // XY contribution: R · cos(V + nv·π/2) · (cos(U + nu·π/2)·X + sin(U + nu·π/2)·Y)
        let xy = self.pos.x_dir * (self.radius * cos_pv * cos_pu)
            + self.pos.y_dir * (self.radius * cos_pv * sin_pu);

        if nu == 0 {
            // Z contribution: R · sin(V + nv·π/2) · Z
            xy + self.pos.z_dir * (self.radius * sin_pv)
        } else {
            xy
        }
    }

    // ─────────────────────────── surface normal ──────────────────────────────

    /// `Normal(U, V)` — the outward unit normal at parameter `(U, V)`.
    ///
    /// Returns `(P, N)`. For a sphere the outward normal is simply
    /// `(P - O) / R`.
    pub fn normal(&self, u: f64, v: f64) -> (Pnt, Vec3) {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let p = self.pos.location
            + self.pos.x_dir * (self.radius * cv * cu)
            + self.pos.y_dir * (self.radius * cv * su)
            + self.pos.z_dir * (self.radius * sv);
        // The outward normal is the unit vector from center to surface point.
        let n = (p - self.pos.location) * (1.0 / self.radius);
        (p, n)
    }

    // ─────────────────────────── area ────────────────────────────────────────

    /// `Area()` — the total surface area of the sphere: `4·π·R²`.
    pub fn area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// `Transform(T)` — apply the affine transform `T` in-place.
    ///
    /// The position is transformed; the radius is scaled by `|T.ScaleFactor()|`.
    pub fn transform(&mut self, t: &Trsf) {
        self.pos = self.pos.transformed(t);
        self.radius = (self.radius * t.scale_factor()).abs();
    }

    /// Return a copy with the transform `T` applied.
    pub fn transformed(&self, t: &Trsf) -> GeomSphericalSurface {
        let mut copy = *self;
        copy.transform(t);
        copy
    }

    /// `Copy()` — a new surface equal to this one.
    pub fn copy(&self) -> GeomSphericalSurface {
        *self
    }
}
