//! `Geom_Plane` -- infinite analytic plane wrapper, mirroring OpenCascade's
//! `Geom_Plane` (`src/ModelingData/TKG3d/Geom/Geom_Plane.hxx`).
//!
//! A plane is defined and positioned in space with a right-handed local
//! coordinate system ([`crate::gp::Ax3`]):
//!
//! - The origin of the local system is the **location** of the plane.
//! - The Z direction of the local system is the **normal** to the plane.
//! - The X and Y directions are the parametric axes.
//!
//! The plane is parameterized as:
//! ```text
//! P(U, V) = O + U * XDir + V * YDir
//! ```
//!
//! where `O`, `XDir`, `YDir` are the origin and the X/Y directions of the
//! local coordinate system. The normal of the plane is `ZDir` (the Z direction
//! of the local system). The parameter ranges are `]-Infinite, +Infinite[` in
//! both U and V directions.
//!
//! UIso/VIso return [`crate::geom_line::GeomLine`] objects, one for each
//! constant-U / constant-V parameter value.

use crate::geom_line::GeomLine;
use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use crate::gp_prim::Pln;
use crate::precision::INFINITE;

// occt: Geom_Plane
/// `Geom_Plane` — an infinite plane in 3D space parameterized as
/// `P(U, V) = O + U*XDir + V*YDir`.
///
/// The plane stores a [`Pln`] (which contains a full [`Ax3`] local frame)
/// internally. The normal is `ZDir` of that local frame.
#[derive(Clone, Copy, Debug)]
pub struct GeomPlane {
    /// The underlying `gp_Pln` primitive (location + full orthonormal frame).
    pln: Pln,
}

impl GeomPlane {
    // ─────────────────────────── constructors ───────────────────────────────

    /// `Geom_Plane(const gp_Pln& Pl)` — construct from a `gp_Pln`.
    pub fn new(pl: Pln) -> Self {
        Self { pln: pl }
    }

    /// Construct from a full local coordinate system.
    ///
    /// Mirrors `Geom_Plane(const gp_Ax3&)`: the origin is `a3.location`,
    /// the parametric X/Y axes are `a3.x_dir`/`a3.y_dir`, and the normal is
    /// `a3.z_dir`.
    pub fn from_ax3(a3: Ax3) -> Self {
        Self {
            pln: Pln::new(a3),
        }
    }

    /// Construct from an origin point and a normal direction.
    ///
    /// Mirrors `Geom_Plane(const gp_Pnt&, const gp_Dir&)`.
    pub fn from_point_normal(p: Pnt, normal: Pnt) -> Self {
        Self {
            pln: Pln::new_pd(p, normal),
        }
    }

    /// Construct from the implicit-form coefficients `a*x + b*y + c*z + d = 0`.
    ///
    /// Mirrors `Geom_Plane(const Standard_Real A, B, C, D)`.
    pub fn from_coefficients(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self {
            pln: Pln::new_coeffs(a, b, c, d),
        }
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// `SetPln(const gp_Pln&)` — replace the underlying `gp_Pln`.
    pub fn set_pln(&mut self, pl: Pln) {
        self.pln = pl;
    }

    // ─────────────────────────── getters ────────────────────────────────────

    /// `Pln()` — return the underlying `gp_Pln`.
    pub fn pln(&self) -> Pln {
        self.pln
    }

    /// The local coordinate system (`Position()` / `gp_Ax3`).
    pub fn position(&self) -> Ax3 {
        self.pln.position()
    }

    /// The location (origin) of the plane.
    pub fn location(&self) -> Pnt {
        self.pln.location()
    }

    /// The unit normal of the plane (`ZDir` of the local frame).
    ///
    /// Equivalent to `Axis().Direction()` in OCCT.
    pub fn normal_direction(&self) -> Vec3 {
        self.pln.position().z_dir
    }

    /// `Axis()` — the main axis of the plane (origin + normal direction).
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pln.location(), self.pln.axis_direction())
    }

    /// Implicit-form coefficients `(a, b, c, d)` for `a*x + b*y + c*z + d = 0`.
    pub fn coefficients(&self) -> (f64, f64, f64, f64) {
        self.pln.coefficients()
    }

    // ─────────────────── parameter / topology queries ───────────────────────

    /// `UReversedParameter(U)` — parameter on the reversed plane: `-U`.
    pub fn u_reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

    /// `VReversedParameter(V)` — parameter on the reversed plane: `-V`.
    pub fn v_reversed_parameter(&self, v: f64) -> f64 {
        -v
    }

    /// `IsUClosed()` — `false`; a plane is unbounded in U.
    pub fn is_u_closed(&self) -> bool {
        false
    }

    /// `IsVClosed()` — `false`; a plane is unbounded in V.
    pub fn is_v_closed(&self) -> bool {
        false
    }

    /// `IsUPeriodic()` — `false`.
    pub fn is_u_periodic(&self) -> bool {
        false
    }

    /// `IsVPeriodic()` — `false`.
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// First U parameter: `-Precision::Infinite()`.
    pub fn u_first_parameter(&self) -> f64 {
        -INFINITE
    }

    /// Last U parameter: `+Precision::Infinite()`.
    pub fn u_last_parameter(&self) -> f64 {
        INFINITE
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

    /// `UIso(U)` — the iso-curve for constant U parameter.
    ///
    /// Returns a [`GeomLine`] parallel to the V direction (`YDir`) of the plane,
    /// passing through `O + U * XDir`.
    pub fn u_iso(&self, u: f64) -> GeomLine {
        let pos = self.pln.position();
        // Origin: advance U along X from the plane origin.
        let origin = pos.location + pos.x_dir * u;
        // Direction: along Y (the V-parametric direction).
        GeomLine::from_point_dir(origin, pos.y_dir)
    }

    /// `VIso(V)` — the iso-curve for constant V parameter.
    ///
    /// Returns a [`GeomLine`] parallel to the U direction (`XDir`) of the plane,
    /// passing through `O + V * YDir`.
    pub fn v_iso(&self, v: f64) -> GeomLine {
        let pos = self.pln.position();
        // Origin: advance V along Y from the plane origin.
        let origin = pos.location + pos.y_dir * v;
        // Direction: along X (the U-parametric direction).
        GeomLine::from_point_dir(origin, pos.x_dir)
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` / `D0(U, V)` — the point on the plane.
    ///
    /// `P(U, V) = O + U * XDir + V * YDir`.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let pos = self.pln.position();
        pos.location + pos.x_dir * u + pos.y_dir * v
    }

    /// `D0(U, V)` — alias for [`value`](Self::value).
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V)` — point and first-order partial derivatives.
    ///
    /// Returns `(P, DU, DV)` where:
    /// - `DU = XDir` (partial derivative w.r.t. U),
    /// - `DV = YDir` (partial derivative w.r.t. V).
    ///
    /// Both partials are constant (the plane is affine).
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let pos = self.pln.position();
        let p = pos.location + pos.x_dir * u + pos.y_dir * v;
        (p, pos.x_dir, pos.y_dir)
    }

    /// `D2(U, V)` — point, first and second-order derivatives.
    ///
    /// Returns `(P, DU, DV, D2U, D2V, D2UV)`. All second-order partials are the
    /// null vector (a plane has constant first-order derivatives).
    pub fn d2(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let pos = self.pln.position();
        let p = pos.location + pos.x_dir * u + pos.y_dir * v;
        let zero = Pnt::origin();
        (p, pos.x_dir, pos.y_dir, zero, zero, zero)
    }

    /// `DN(U, V, Nu, Nv)` — derivative of total order `Nu + Nv`.
    ///
    /// For a plane:
    /// - `(Nu=1, Nv=0)` → `XDir`
    /// - `(Nu=0, Nv=1)` → `YDir`
    /// - All other orders → null vector (plane is degree 1)
    ///
    /// # Panics
    /// Panics if `nu < 0` or `nv < 0` or `nu + nv < 1`, mirroring OCCT's
    /// `Standard_RangeError`.
    pub fn dn(&self, _u: f64, _v: f64, nu: i32, nv: i32) -> Vec3 {
        assert!(nu >= 0, "GeomPlane::DN: Nu must be >= 0");
        assert!(nv >= 0, "GeomPlane::DN: Nv must be >= 0");
        assert!(nu + nv >= 1, "GeomPlane::DN: Nu + Nv must be >= 1");
        let pos = self.pln.position();
        if nu == 1 && nv == 0 {
            pos.x_dir
        } else if nu == 0 && nv == 1 {
            pos.y_dir
        } else {
            // All second- and higher-order derivatives of a plane are zero.
            Pnt::origin()
        }
    }

    // ─────────────────────────── normal ─────────────────────────────────────

    /// `Normal(U, V)` — the unit normal at parameter `(U, V)`.
    ///
    /// Returns `(P, N)` where:
    /// - `P = Value(U, V)`,
    /// - `N = ZDir` (constant for a plane).
    pub fn normal(&self, u: f64, v: f64) -> (Pnt, Vec3) {
        let pos = self.pln.position();
        let p = pos.location + pos.x_dir * u + pos.y_dir * v;
        (p, pos.z_dir)
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// `Transform(T)` — apply the affine transform `T` to this plane in-place.
    pub fn transform(&mut self, t: &Trsf) {
        self.pln = self.pln.transformed(t);
    }

    /// Return a copy of this plane with the transform `T` applied.
    pub fn transformed(&self, t: &Trsf) -> GeomPlane {
        GeomPlane {
            pln: self.pln.transformed(t),
        }
    }

    /// `Copy()` — a new plane equal to this one.
    pub fn copy(&self) -> GeomPlane {
        *self
    }
}
