//! `Geom_ToroidalSurface` — analytic toroidal surface in 3D space, mirroring
//! OpenCascade's `Geom_ToroidalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_ToroidalSurface.hxx`).
//!
//! A toroidal surface is defined by:
//! - a local coordinate system ([`Ax3`]) whose Z axis is the torus axis and
//!   whose origin is the center of the torus;
//! - a **major radius** `R > 0`: distance from the center of the torus to the
//!   center of the tube;
//! - a **minor radius** `r > 0`: radius of the tube.
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = O + (R + r·cos(V))·(cos(U)·XDir + sin(U)·YDir) + r·sin(V)·ZDir
//! ```
//! where:
//! - U ∈ [0, 2π) is the angular parameter around the main axis (periodic),
//! - V ∈ [0, 2π) is the angular parameter around the tube (periodic).
//!
//! Both U and V are periodic with period 2π.
//!
//! The iso-curves are:
//! - UIso(U₀): a circle of radius `r` around the tube at angular position U₀,
//!   in the plane containing ZDir and the radial direction `cos(U₀)·X + sin(U₀)·Y`.
//! - VIso(V₀): a circle of radius `R + r·cos(V₀)` in the XY plane of the local
//!   frame (normal to ZDir), centered at `O + r·sin(V₀)·ZDir`.
//!
//! References:
//! - OCCT `Geom_ToroidalSurface` (TKG3d)
//! - ISO 10303-42, toroidal surface definition

use crate::geom_circle::GeomCircle;
use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use std::f64::consts::PI;

// occt-ref: Geom_ToroidalSurface
/// `Geom_ToroidalSurface` — an analytic toroidal surface.
///
/// Parameterized as:
/// ```text
/// P(U, V) = O + (R + r·cos(V))·(cos(U)·XDir + sin(U)·YDir) + r·sin(V)·ZDir
/// ```
/// Both U and V have period 2π.
#[derive(Clone, Copy, Debug)]
pub struct GeomToroidalSurface {
    /// The local coordinate system: origin is the center of the torus,
    /// Z is the main axis, X/Y span the equatorial plane.
    pos: Ax3,
    /// Major radius: distance from torus center to tube center.
    major_radius: f64,
    /// Minor radius: radius of the tube.
    minor_radius: f64,
}

impl GeomToroidalSurface {
    // ─────────────────────────── constructors ───────────────────────────────

    /// `Geom_ToroidalSurface(const gp_Ax3& A3, Standard_Real MajorRadius, Standard_Real MinorRadius)`.
    ///
    /// - `a3` — the local coordinate system (Z is the torus axis).
    /// - `major_radius` — must be > 0.
    /// - `minor_radius` — must be >= 0.
    ///
    /// # Panics
    /// Panics if `major_radius <= 0` or `minor_radius < 0`
    /// (mirrors OCCT's `Standard_ConstructionError`).
    pub fn new(a3: Ax3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            major_radius > 0.0,
            "Geom_ToroidalSurface: MajorRadius must be > 0, got {major_radius}"
        );
        assert!(
            minor_radius >= 0.0,
            "Geom_ToroidalSurface: MinorRadius must be >= 0, got {minor_radius}"
        );
        Self {
            pos: a3,
            major_radius,
            minor_radius,
        }
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// `SetMajorRadius(R)` — change the major radius.
    ///
    /// # Panics
    /// Panics if `r <= 0`.
    pub fn set_major_radius(&mut self, r: f64) {
        assert!(
            r > 0.0,
            "Geom_ToroidalSurface::SetMajorRadius: R must be > 0"
        );
        self.major_radius = r;
    }

    /// `SetMinorRadius(r)` — change the minor radius.
    ///
    /// # Panics
    /// Panics if `r < 0`.
    pub fn set_minor_radius(&mut self, r: f64) {
        assert!(
            r >= 0.0,
            "Geom_ToroidalSurface::SetMinorRadius: R must be >= 0"
        );
        self.minor_radius = r;
    }

    /// Replace the local coordinate system.
    pub fn set_position(&mut self, a3: Ax3) {
        self.pos = a3;
    }

    // ─────────────────────────── getters ────────────────────────────────────

    /// `MajorRadius()` — the major radius.
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// `MinorRadius()` — the minor radius.
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// `Position()` — the local coordinate system (`gp_Ax3`).
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `Location()` — the origin of the local coordinate system (center of
    /// the torus).
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// `Axis()` — the main axis of the torus (origin + Z direction).
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    // ──────────────────── parameter / topology queries ───────────────────────

    /// `UReversedParameter(U)` — parameter on the U-reversed surface: `2*Pi - U`.
    pub fn u_reversed_parameter(&self, u: f64) -> f64 {
        2.0 * PI - u
    }

    /// `VReversedParameter(V)` — parameter on the V-reversed surface: `2*Pi - V`.
    pub fn v_reversed_parameter(&self, v: f64) -> f64 {
        2.0 * PI - v
    }

    /// `UPeriod()` — 2π (the torus is periodic in U).
    pub fn u_period(&self) -> f64 {
        2.0 * PI
    }

    /// `VPeriod()` — 2π (the torus is periodic in V).
    pub fn v_period(&self) -> f64 {
        2.0 * PI
    }

    /// `IsUClosed()` — `true`; the torus is closed in U.
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed()` — `true`; the torus is closed in V.
    pub fn is_v_closed(&self) -> bool {
        true
    }

    /// `IsUPeriodic()` — `true`; period is 2π.
    pub fn is_u_periodic(&self) -> bool {
        true
    }

    /// `IsVPeriodic()` — `true`; period is 2π.
    pub fn is_v_periodic(&self) -> bool {
        true
    }

    /// First U parameter: `0.0`.
    pub fn u_first_parameter(&self) -> f64 {
        0.0
    }

    /// Last U parameter: `2*Pi`.
    pub fn u_last_parameter(&self) -> f64 {
        2.0 * PI
    }

    /// First V parameter: `0.0`.
    pub fn v_first_parameter(&self) -> f64 {
        0.0
    }

    /// Last V parameter: `2*Pi`.
    pub fn v_last_parameter(&self) -> f64 {
        2.0 * PI
    }

    // ────────────────────────── iso-curves ──────────────────────────────────

    /// `UIso(U)` — iso-curve for constant U parameter.
    ///
    /// Returns a circle of radius `r` (minor radius) in the plane defined by
    /// the radial direction `cos(U)·X + sin(U)·Y` and the axis direction Z.
    /// Its center is at `O + R·(cos(U)·XDir + sin(U)·YDir)`.
    ///
    /// The circle lies in the plane whose normal is perpendicular to both ZDir
    /// and the radial direction — i.e. it is in the `(radial, Z)` plane. In
    /// OCCT terms, the UIso circle has:
    /// - center: `O + R·radial`
    /// - X direction: `radial`  (the radial direction, toward the tube center)
    /// - Y direction: `ZDir`
    /// - Z direction: `tangential = -sin(U)·XDir + cos(U)·YDir` (= normal of the meridian plane)
    pub fn u_iso(&self, u: f64) -> GeomCircle {
        let (su, cu) = u.sin_cos();
        let radial = self.pos.x_dir * cu + self.pos.y_dir * su;
        let center = self.pos.location + radial * self.major_radius;
        // The meridian circle lies in the plane spanned by `radial` and `z_dir`.
        // The normal to this plane is the tangential direction: -sin(U)·X + cos(U)·Y.
        let tangential = self.pos.x_dir * (-su) + self.pos.y_dir * cu;
        let frame = Ax3 {
            location: center,
            x_dir: radial,
            y_dir: self.pos.z_dir,
            z_dir: tangential,
        };
        GeomCircle::from_ax3(frame, self.minor_radius)
    }

    /// `VIso(V)` — iso-curve for constant V parameter.
    ///
    /// Returns a circle of radius `R + r·cos(V)` in the XY plane of the local
    /// frame (normal to ZDir), centered at `O + r·sin(V)·ZDir`.
    pub fn v_iso(&self, v: f64) -> GeomCircle {
        let (sv, cv) = v.sin_cos();
        let radius = self.major_radius + self.minor_radius * cv;
        let center = self.pos.location + self.pos.z_dir * (self.minor_radius * sv);
        let frame = Ax3 {
            location: center,
            x_dir: self.pos.x_dir,
            y_dir: self.pos.y_dir,
            z_dir: self.pos.z_dir,
        };
        // Radius may be negative if minor_radius > major_radius and cos(V) is
        // sufficiently negative; take the absolute value per OCCT convention.
        GeomCircle::from_ax3(frame, radius.abs())
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` / `D0` — the point on the torus surface.
    ///
    /// `P(U, V) = O + (R + r·cos(V))·(cos(U)·XDir + sin(U)·YDir) + r·sin(V)·ZDir`
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let rho = self.major_radius + self.minor_radius * cv;
        self.pos.location
            + self.pos.x_dir * (rho * cu)
            + self.pos.y_dir * (rho * su)
            + self.pos.z_dir * (self.minor_radius * sv)
    }

    /// `D0(U, V)` — alias for [`value`](Self::value).
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V)` — point and first-order partial derivatives.
    ///
    /// Let `ρ(V) = R + r·cos(V)`.
    ///
    /// Returns `(P, DU, DV)` where:
    /// - `DU = ∂P/∂U = ρ·(-sin(U)·XDir + cos(U)·YDir)`
    /// - `DV = ∂P/∂V = -r·sin(V)·(cos(U)·XDir + sin(U)·YDir) + r·cos(V)·ZDir`
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let rho = self.major_radius + self.minor_radius * cv;

        let p = self.pos.location
            + self.pos.x_dir * (rho * cu)
            + self.pos.y_dir * (rho * su)
            + self.pos.z_dir * (self.minor_radius * sv);

        let du = self.pos.x_dir * (-rho * su) + self.pos.y_dir * (rho * cu);

        let dv = self.pos.x_dir * (-self.minor_radius * sv * cu)
            + self.pos.y_dir * (-self.minor_radius * sv * su)
            + self.pos.z_dir * (self.minor_radius * cv);

        (p, du, dv)
    }

    /// `DN(U, V, Nu, Nv)` — derivative of order `(Nu, Nv)`.
    ///
    /// The torus parameterization factors into:
    /// - U-dependence via  `cos(U)` / `sin(U)` (periodic, nth derivative adds `n·π/2` phase)
    /// - V-dependence via  `cos(V)` / `sin(V)` (same)
    ///
    /// Let:
    /// - `φ_U = U + Nu·π/2`  and  `φ_V = V + Nv·π/2`.
    /// - Radial unit vector `e_r(U) = cos(U)·X + sin(U)·Y`.
    /// - Tangential unit vector `e_t(U) = -sin(U)·X + cos(U)·Y`.
    ///
    /// Then:
    /// ```text
    /// d^Nu/dU^Nu [cos(U)] = cos(U + Nu·π/2)
    /// d^Nu/dU^Nu [sin(U)] = sin(U + Nu·π/2)
    /// ```
    ///
    /// The `(Nu, Nv)`-th mixed partial of `P(U, V)`:
    ///
    /// Case `Nu = 0, Nv = 0`: the point itself (invalid, panics).
    ///
    /// Case `Nu = 0, Nv ≥ 1`:
    /// ```text
    /// ∂^Nv P / ∂V^Nv = r · cos(V + Nv·π/2) · (cos(U)·X + sin(U)·Y)
    ///                + r · sin(V + Nv·π/2) · ZDir
    ///               [note: the R·(cos(U)·X+sin(U)·Y) term vanishes for Nv≥1]
    /// ```
    ///
    /// Case `Nu ≥ 1, Nv = 0`:
    /// ```text
    /// ∂^Nu P / ∂U^Nu = (R + r·cos(V)) · (cos(U + Nu·π/2)·X + sin(U + Nu·π/2)·Y)
    /// ```
    ///
    /// Case `Nu ≥ 1, Nv ≥ 1` (mixed):
    /// ```text
    /// ∂^(Nu+Nv) P / ∂U^Nu ∂V^Nv
    ///   = r · cos(V + Nv·π/2) · (cos(U + Nu·π/2)·X + sin(U + Nu·π/2)·Y)
    /// ```
    /// (The ZDir component is constant in U, so its U-derivatives are zero for Nu≥1.)
    ///
    /// # Panics
    /// Panics if `nu < 0`, `nv < 0`, or `nu + nv < 1`.
    pub fn dn(&self, u: f64, v: f64, nu: i32, nv: i32) -> Vec3 {
        assert!(nu >= 0, "Geom_ToroidalSurface::DN: Nu must be >= 0");
        assert!(nv >= 0, "Geom_ToroidalSurface::DN: Nv must be >= 0");
        assert!(
            nu + nv >= 1,
            "Geom_ToroidalSurface::DN: Nu + Nv must be >= 1"
        );

        let phase_u = u + (nu as f64) * (PI / 2.0);
        let (spu, cpu) = phase_u.sin_cos();

        let phase_v = v + (nv as f64) * (PI / 2.0);
        let (spv, cpv) = phase_v.sin_cos();

        if nu == 0 {
            // Pure V-derivative of order nv (nv >= 1).
            // ∂^Nv P / ∂V^Nv:
            //   The R-term is constant in V, so vanishes for nv >= 1.
            //   The r·cos(V) term contributes: r·cos(V + nv·π/2) for the radial direction.
            //   The r·sin(V) term contributes: r·sin(V + nv·π/2) for ZDir.
            let (su, cu) = u.sin_cos();
            let radial = self.pos.x_dir * cu + self.pos.y_dir * su;
            radial * (self.minor_radius * cpv) + self.pos.z_dir * (self.minor_radius * spv)
        } else if nv == 0 {
            // Pure U-derivative of order nu (nu >= 1).
            // ∂^Nu P / ∂U^Nu = ρ(V) · (cos(U + nu·π/2)·X + sin(U + nu·π/2)·Y)
            let rho = self.major_radius + self.minor_radius * v.cos();
            self.pos.x_dir * (rho * cpu) + self.pos.y_dir * (rho * spu)
        } else {
            // Mixed: nu >= 1, nv >= 1.
            // ∂^(nu+nv) P / ∂U^nu ∂V^nv
            //   = r · cos(V + nv·π/2) · (cos(U + nu·π/2)·X + sin(U + nu·π/2)·Y)
            // The Z component (r·sin(V)) is constant in U; its U-derivatives are 0.
            self.pos.x_dir * (self.minor_radius * cpv * cpu)
                + self.pos.y_dir * (self.minor_radius * cpv * spu)
        }
    }

    // ─────────────────────────── normal ─────────────────────────────────────

    /// `Normal(U, V)` — the unit outward normal at parameter `(U, V)`.
    ///
    /// The normal is `(DU × DV).normalized()`.
    ///
    /// For the standard torus:
    /// - `DU = ρ(-sin(U)·X + cos(U)·Y)`
    /// - `DV = -r·sin(V)·(cos(U)·X + sin(U)·Y) + r·cos(V)·Z`
    /// - `DU × DV = ρ·r·(cos(V)·(cos(U)·X + sin(U)·Y) + sin(V)·Z)`
    ///
    /// Unit normal: `cos(V)·(cos(U)·X + sin(U)·Y) + sin(V)·Z`
    /// (independent of the scale factor `ρ·r`; outward if ρ > 0).
    pub fn normal(&self, u: f64, v: f64) -> (Pnt, Vec3) {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let rho = self.major_radius + self.minor_radius * cv;

        let p = self.pos.location
            + self.pos.x_dir * (rho * cu)
            + self.pos.y_dir * (rho * su)
            + self.pos.z_dir * (self.minor_radius * sv);

        // The outward unit normal is:
        // n = cos(V) · e_r(U) + sin(V) · ZDir
        // where e_r = cos(U)·X + sin(U)·Y.
        let n = self.pos.x_dir * (cv * cu)
            + self.pos.y_dir * (cv * su)
            + self.pos.z_dir * sv;

        // n is already unit (cv²·(cu²+su²) + sv² = cv² + sv² = 1).
        (p, n)
    }

    // ─────────────── geometric properties ───────────────────────────────────

    /// `Area()` — surface area of the torus.
    ///
    /// For a torus with major radius R and minor radius r:
    /// `Area = 4·π²·R·r`
    pub fn area(&self) -> f64 {
        4.0 * PI * PI * self.major_radius * self.minor_radius
    }

    /// `Volume()` — volume enclosed by the torus.
    ///
    /// `Volume = 2·π²·R·r²`
    pub fn volume(&self) -> f64 {
        2.0 * PI * PI * self.major_radius * self.minor_radius * self.minor_radius
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// `Transform(T)` — apply the affine transform `T` in-place.
    ///
    /// Both radii are scaled by the absolute scale factor.
    pub fn transform(&mut self, t: &Trsf) {
        self.pos = self.pos.transformed(t);
        let s = t.scale_factor().abs();
        self.major_radius *= s;
        self.minor_radius *= s;
    }

    /// Return a copy with the transform `T` applied.
    pub fn transformed(&self, t: &Trsf) -> GeomToroidalSurface {
        let mut copy = *self;
        copy.transform(t);
        copy
    }

    /// `Copy()` — a new surface equal to this one.
    pub fn copy(&self) -> GeomToroidalSurface {
        *self
    }
}
