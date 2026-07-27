//! `Geom_CylindricalSurface` — analytic cylindrical surface in 3D space,
//! mirroring OpenCascade's `Geom_CylindricalSurface`
//! (`src/ModelingData/TKG3d/Geom/Geom_CylindricalSurface.hxx`).
//!
//! A cylindrical surface is defined by:
//! - a local coordinate system (`gp_Ax3`, here [`Ax3`]) whose origin is the
//!   axis location and whose Z direction is the cylinder axis;
//! - a radius `R > 0`.
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = O + R*cos(U)*XDir + R*sin(U)*YDir + V*ZDir
//! ```
//! where:
//! - U ∈ [0, 2π) is the angular parameter (periodic),
//! - V ∈ (-∞, +∞) is the height parameter along the axis.
//!
//! References:
//! - OCCT `Geom_CylindricalSurface` (TKG3d)
//! - ISO 10303-42, cylindrical surface definition

use crate::geom_circle::GeomCircle;
use crate::geom_line::GeomLine;
use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use crate::precision::INFINITE;
use std::f64::consts::PI;

// occt-ref: Geom_CylindricalSurface
/// `Geom_CylindricalSurface` — an infinite analytic cylindrical surface.
///
/// The surface is parameterized by `(U, V)` as:
/// `P(U, V) = O + R*cos(U)*XDir + R*sin(U)*YDir + V*ZDir`
///
/// where `O`, `XDir`, `YDir`, `ZDir` are the origin and axes of the local
/// coordinate system, and `R` is the radius.
///
/// - U is periodic with period `2*Pi` and the surface is closed in U.
/// - V is unbounded.
#[derive(Clone, Copy, Debug)]
pub struct GeomCylindricalSurface {
    /// The local coordinate system (`gp_Ax3`): origin = axis location,
    /// Z = cylinder axis direction, X/Y span the base circle plane.
    pos: Ax3,
    /// The cylinder radius (must be > 0).
    radius: f64,
}

impl GeomCylindricalSurface {
    // ─────────────────────────── constructors ───────────────────────────────

    /// `Geom_CylindricalSurface(const gp_Ax3& A3, const Standard_Real Radius)`.
    ///
    /// - `a3` defines the local coordinate system: its origin is the axis
    ///   location, Z direction is the cylinder axis.
    /// - `radius` must be >= 0 (OCCT raises `Standard_ConstructionError` if < 0).
    ///
    /// # Panics
    /// Panics if `radius < 0`.
    pub fn new(a3: Ax3, radius: f64) -> Self {
        assert!(
            radius >= 0.0,
            "Geom_CylindricalSurface: Radius must be >= 0"
        );
        Self { pos: a3, radius }
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// `SetRadius(R)` — change the radius.
    ///
    /// # Panics
    /// Panics if `r < 0`.
    pub fn set_radius(&mut self, r: f64) {
        assert!(
            r >= 0.0,
            "Geom_CylindricalSurface::SetRadius: R must be >= 0"
        );
        self.radius = r;
    }

    /// `SetPosition(A3)` — change the local coordinate system.
    pub fn set_position(&mut self, a3: Ax3) {
        self.pos = a3;
    }

    // ─────────────────────────── getters ────────────────────────────────────

    /// `Radius()` — the cylinder radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// `Position()` — the local coordinate system (`gp_Ax3`).
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `Location()` — the origin of the local coordinate system.
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// `Axis()` — the cylinder axis (`Ax1`: origin + Z direction).
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    // ──────────────────── parameter / topology queries ───────────────────────

    /// `UReversedParameter(U)` — parameter on the U-reversed surface: `2*Pi - U`.
    ///
    /// Reversing U maps `cos(U)→cos(-U)=cos(U)` and `sin(U)→-sin(U)`, so the
    /// formula is `2*Pi - U` (consistent with OCCT's cylindrical surface).
    pub fn u_reversed_parameter(&self, u: f64) -> f64 {
        2.0 * PI - u
    }

    /// `VReversedParameter(V)` — parameter on the V-reversed surface: `-V`.
    pub fn v_reversed_parameter(&self, v: f64) -> f64 {
        -v
    }

    /// `UPeriod()` — `2*Pi` (the cylinder is periodic in U).
    pub fn u_period(&self) -> f64 {
        2.0 * PI
    }

    /// `IsUClosed()` — `true`; the cylinder is closed in U (seam at U=0/2π).
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed()` — `false`; the cylinder is unbounded in V.
    pub fn is_v_closed(&self) -> bool {
        false
    }

    /// `IsUPeriodic()` — `true`; U has period `2*Pi`.
    pub fn is_u_periodic(&self) -> bool {
        true
    }

    /// `IsVPeriodic()` — `false`; V is not periodic.
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// First U parameter: `0.0`.
    pub fn u_first_parameter(&self) -> f64 {
        0.0
    }

    /// Last U parameter: `2*Pi`.
    pub fn u_last_parameter(&self) -> f64 {
        2.0 * PI
    }

    /// First V parameter: `-Precision::Infinite()`.
    pub fn v_first_parameter(&self) -> f64 {
        -INFINITE
    }

    /// Last V parameter: `+Precision::Infinite()`.
    pub fn v_last_parameter(&self) -> f64 {
        INFINITE
    }

    // ────────────────────────── iso-curves ──────────────────────────────────

    /// `UIso(U)` — iso-curve for constant U parameter.
    ///
    /// Returns a line parallel to the cylinder axis (`ZDir`) passing through
    /// the point `O + R*cos(U)*XDir + R*sin(U)*YDir`.
    pub fn u_iso(&self, u: f64) -> GeomLine {
        let (s, c) = u.sin_cos();
        let origin = self.pos.location
            + self.pos.x_dir * (self.radius * c)
            + self.pos.y_dir * (self.radius * s);
        GeomLine::from_point_dir(origin, self.pos.z_dir)
    }

    /// `VIso(V)` — iso-curve for constant V parameter.
    ///
    /// Returns a circle of radius `R` whose plane is normal to the cylinder
    /// axis at height `V` from the origin: center = `O + V*ZDir`.
    pub fn v_iso(&self, v: f64) -> GeomCircle {
        let center = self.pos.location + self.pos.z_dir * v;
        let frame = Ax3 {
            location: center,
            x_dir: self.pos.x_dir,
            y_dir: self.pos.y_dir,
            z_dir: self.pos.z_dir,
        };
        GeomCircle::from_ax3(frame, self.radius)
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` / `D0` — the point on the cylinder surface.
    ///
    /// `P(U, V) = O + R*cos(U)*XDir + R*sin(U)*YDir + V*ZDir`.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (s, c) = u.sin_cos();
        self.pos.location
            + self.pos.x_dir * (self.radius * c)
            + self.pos.y_dir * (self.radius * s)
            + self.pos.z_dir * v
    }

    /// `D0(U, V)` — alias for [`value`](Self::value).
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V)` — point and first-order partial derivatives.
    ///
    /// Returns `(P, DU, DV)` where:
    /// - `DU = ∂P/∂U = -R*sin(U)*XDir + R*cos(U)*YDir`
    /// - `DV = ∂P/∂V = ZDir`
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let (s, c) = u.sin_cos();
        let p = self.pos.location
            + self.pos.x_dir * (self.radius * c)
            + self.pos.y_dir * (self.radius * s)
            + self.pos.z_dir * v;
        let du = self.pos.x_dir * (-self.radius * s) + self.pos.y_dir * (self.radius * c);
        let dv = self.pos.z_dir;
        (p, du, dv)
    }

    /// `D2(U, V)` — point, first- and second-order derivatives.
    ///
    /// Returns `(P, DU, DV, D2U, D2V, D2UV)` where:
    /// - `D2U  = ∂²P/∂U² = -R*cos(U)*XDir - R*sin(U)*YDir`
    /// - `D2V  = ∂²P/∂V² = 0`
    /// - `D2UV = ∂²P/∂U∂V = 0`
    pub fn d2(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let (s, c) = u.sin_cos();
        let p = self.pos.location
            + self.pos.x_dir * (self.radius * c)
            + self.pos.y_dir * (self.radius * s)
            + self.pos.z_dir * v;
        let du = self.pos.x_dir * (-self.radius * s) + self.pos.y_dir * (self.radius * c);
        let dv = self.pos.z_dir;
        let d2u = self.pos.x_dir * (-self.radius * c) + self.pos.y_dir * (-self.radius * s);
        let d2v = Pnt::origin();
        let d2uv = Pnt::origin();
        (p, du, dv, d2u, d2v, d2uv)
    }

    /// `DN(U, V, Nu, Nv)` — derivative of total order `Nu + Nv`.
    ///
    /// The surface has independent U and V behavior:
    /// - Derivatives in V are: order 1 → `ZDir`, order ≥ 2 → 0.
    /// - Derivatives in U are rotations of `(cos/sin)` scaled by `R`:
    ///   the `Nu`-th U-derivative is `R * cos(U + Nu*π/2) * XDir + R * sin(U + Nu*π/2) * YDir`.
    /// - Mixed derivatives `Nu ≥ 1, Nv ≥ 1`: always zero (U and V decouple).
    ///
    /// # Panics
    /// Panics if `nu < 0`, `nv < 0`, or `nu + nv < 1`.
    pub fn dn(&self, u: f64, _v: f64, nu: i32, nv: i32) -> Vec3 {
        assert!(nu >= 0, "Geom_CylindricalSurface::DN: Nu must be >= 0");
        assert!(nv >= 0, "Geom_CylindricalSurface::DN: Nv must be >= 0");
        assert!(
            nu + nv >= 1,
            "Geom_CylindricalSurface::DN: Nu + Nv must be >= 1"
        );

        if nv == 0 {
            // Pure U-derivative of order nu.
            // d^nu/dU^nu [R*cos(U)*X + R*sin(U)*Y]
            //   = R*cos(U + nu*π/2)*X + R*sin(U + nu*π/2)*Y
            let phase = u + (nu as f64) * (PI / 2.0);
            let (s, c) = phase.sin_cos();
            self.pos.x_dir * (self.radius * c) + self.pos.y_dir * (self.radius * s)
        } else if nu == 0 {
            // Pure V-derivative of order nv.
            // d/dV [V*ZDir] = ZDir; d^n/dV^n = 0 for n >= 2.
            if nv == 1 {
                self.pos.z_dir
            } else {
                Pnt::origin()
            }
        } else {
            // Mixed: nu >= 1 and nv >= 1.
            // ∂/∂V of any U-derivative is zero (U and V are independent).
            Pnt::origin()
        }
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// `Transform(T)` — apply the affine transform `T` in-place.
    ///
    /// The radius is scaled by `|T.ScaleFactor()|`.
    pub fn transform(&mut self, t: &Trsf) {
        self.pos = self.pos.transformed(t);
        self.radius = (self.radius * t.scale_factor()).abs();
    }

    /// Return a copy with the transform `T` applied.
    pub fn transformed(&self, t: &Trsf) -> GeomCylindricalSurface {
        let mut copy = *self;
        copy.transform(t);
        copy
    }

    /// `Copy()` — a new surface equal to this one.
    pub fn copy(&self) -> GeomCylindricalSurface {
        *self
    }
}
