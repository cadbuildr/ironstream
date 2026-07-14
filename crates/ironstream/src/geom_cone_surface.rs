//! `Geom_ConicalSurface` — analytic conical surface, mirroring OpenCascade's
//! `Geom_ConicalSurface` (`src/ModelingData/TKG3d/Geom/Geom_ConicalSurface.hxx`).
//!
//! A conical surface is defined by:
//! - A right-handed local coordinate system ([`Ax3`]) whose Z axis is the cone
//!   axis and whose origin is the apex of the cone (when `radius == 0`), or more
//!   generally the point on the axis at parameter V = 0.
//! - A **semi-angle** `α` (in radians, `|α| < π/2`, `α ≠ 0`), measured between
//!   the axis and the generatrix (slant height direction).
//! - A **reference radius** `R ≥ 0`: the radius of the circle at V = 0.
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = O + (R + V·tan(α))·(cos(U)·XDir + sin(U)·YDir) + V·ZDir
//! ```
//! where `O`, `XDir`, `YDir`, `ZDir` come from the local coordinate system.
//!
//! - U is the angular parameter in `[0, 2π[`, periodic.
//! - V is the axial parameter (along the cone axis), unbounded.
//! - UIso(U0) is a line through `P(U0, 0)` along the generatrix direction.
//! - VIso(V0) is a circle at constant axial distance V0.

use crate::geom_circle::GeomCircle;
use crate::geom_line::GeomLine;
use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};
use std::f64::consts::PI;

// occt-ref: Geom_ConicalSurface
/// `Geom_ConicalSurface` — analytic cone surface in 3D space.
///
/// The parameterization is:
/// ```text
/// P(U, V) = O + (R + V·tan(α))·(cos(U)·XDir + sin(U)·YDir) + V·ZDir
/// ```
/// where `α` is the semi-angle and `R` is the reference radius at V = 0.
#[derive(Clone, Copy, Debug)]
pub struct GeomConicalSurface {
    /// Local coordinate system. Origin = point on axis at V = 0, Z = axis
    /// direction, X/Y define the angular origin.
    pos: Ax3,
    /// Semi-angle of the cone (radians). Positive for an expanding cone in +Z.
    semi_angle: f64,
    /// Reference radius at V = 0 (>= 0).
    ref_radius: f64,
}

impl GeomConicalSurface {
    // ─────────────────────────── constructors ───────────────────────────────

    /// `Geom_ConicalSurface(const gp_Ax3& A3, Standard_Real Ang, Standard_Real Radius)`.
    ///
    /// - `a3`  — the local coordinate system (Z is the cone axis direction).
    /// - `semi_angle` — the half-angle at the apex (radians); must satisfy
    ///   `|semi_angle| < π/2` and `semi_angle ≠ 0`.
    /// - `radius` — the reference radius at V = 0 (must be ≥ 0).
    ///
    /// # Panics
    /// Raises if `radius < 0` or `|semi_angle| >= π/2` (Standard_ConstructionError).
    pub fn new(a3: Ax3, semi_angle: f64, radius: f64) -> Self {
        assert!(
            radius >= 0.0,
            "Geom_ConicalSurface: Radius must be >= 0, got {radius}"
        );
        assert!(
            semi_angle.abs() < PI / 2.0,
            "Geom_ConicalSurface: |SemiAngle| must be < π/2, got {semi_angle}"
        );
        Self {
            pos: a3,
            semi_angle,
            ref_radius: radius,
        }
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// `SetSemiAngle(Ang)` — change the half-angle.
    ///
    /// # Panics
    /// Raises if `|ang| >= π/2`.
    pub fn set_semi_angle(&mut self, ang: f64) {
        assert!(
            ang.abs() < PI / 2.0,
            "Geom_ConicalSurface::SetSemiAngle: |Ang| must be < π/2"
        );
        self.semi_angle = ang;
    }

    /// `SetRadius(R)` — change the reference radius.
    ///
    /// # Panics
    /// Raises if `r < 0`.
    pub fn set_radius(&mut self, r: f64) {
        assert!(r >= 0.0, "Geom_ConicalSurface::SetRadius: R must be >= 0");
        self.ref_radius = r;
    }

    /// Replace the local coordinate system.
    pub fn set_position(&mut self, a3: Ax3) {
        self.pos = a3;
    }

    // ─────────────────────────── getters ────────────────────────────────────

    /// `SemiAngle()` — the half-angle between the axis and the generatrix.
    pub fn semi_angle(&self) -> f64 {
        self.semi_angle
    }

    /// `RefRadius()` — the reference radius at parameter V = 0.
    pub fn ref_radius(&self) -> f64 {
        self.ref_radius
    }

    /// The local coordinate system (`Position()` / `gp_Ax3`).
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `Location()` — the origin of the local coordinate system.
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// `Axis()` — the main axis of the cone (Z axis of the local frame).
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// `Apex()` — the apex of the cone.
    ///
    /// This is the point on the Z axis where the radius becomes zero:
    /// V_apex = -R / tan(α),  Apex = O + V_apex * ZDir.
    ///
    /// For `semi_angle == 0` there is no real apex; we return the location.
    pub fn apex(&self) -> Pnt {
        let tan_a = self.semi_angle.tan();
        if tan_a.abs() < 1.0e-12 {
            self.pos.location
        } else {
            let v_apex = -self.ref_radius / tan_a;
            self.pos.location + self.pos.z_dir * v_apex
        }
    }

    // ─────────────────── parameter / topology queries ───────────────────────

    /// `IsUClosed()` — `true`; the angular parameter is closed (periodic).
    pub fn is_u_closed(&self) -> bool {
        true
    }

    /// `IsVClosed()` — `false`; the axial parameter is unbounded.
    pub fn is_v_closed(&self) -> bool {
        false
    }

    /// `IsUPeriodic()` — `true`; period is 2π.
    pub fn is_u_periodic(&self) -> bool {
        true
    }

    /// `IsVPeriodic()` — `false`.
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// `UPeriod()` — 2π.
    pub fn u_period(&self) -> f64 {
        2.0 * PI
    }

    /// Radius of the circle at axial parameter `v`.
    ///
    /// `r(v) = R + v * tan(α)`
    pub fn radius_at(&self, v: f64) -> f64 {
        self.ref_radius + v * self.semi_angle.tan()
    }

    // ─────────────────────────── iso-curves ─────────────────────────────────

    /// `UIso(U)` — iso-curve for constant U.
    ///
    /// This is the generatrix (slant line) at angular position U.
    /// Direction: `sin(α)*cos(U)*XDir + sin(α)*sin(U)*YDir + cos(α)*ZDir`
    /// (unit vector along the slant, pointing in +V direction).
    ///
    /// Origin: `P(U, 0) = O + R*(cos(U)*XDir + sin(U)*YDir)`.
    pub fn u_iso(&self, u: f64) -> GeomLine {
        let (su, cu) = u.sin_cos();
        let origin = self.pos.location
            + self.pos.x_dir * (self.ref_radius * cu)
            + self.pos.y_dir * (self.ref_radius * su);
        // Generatrix direction: derivative of P(U, V) w.r.t. V at constant U.
        // dP/dV = tan(α)*(cos(U)*XDir + sin(U)*YDir) + ZDir
        let tan_a = self.semi_angle.tan();
        let dir = self.pos.x_dir * (tan_a * cu)
            + self.pos.y_dir * (tan_a * su)
            + self.pos.z_dir;
        GeomLine::from_point_dir(origin, dir)
    }

    /// `VIso(V)` — iso-curve for constant V.
    ///
    /// This is a circle of radius `R + V*tan(α)` centered at
    /// `O + V * ZDir`, in the plane normal to ZDir.
    pub fn v_iso(&self, v: f64) -> GeomCircle {
        let center = self.pos.location + self.pos.z_dir * v;
        let r = (self.ref_radius + v * self.semi_angle.tan()).abs();
        // Build a frame for the circle: center + same X/Y, Z = cone Z.
        let frame = Ax3 {
            location: center,
            x_dir: self.pos.x_dir,
            y_dir: self.pos.y_dir,
            z_dir: self.pos.z_dir,
        };
        GeomCircle::from_ax3(frame, r)
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` / `D0` — the point on the cone surface.
    ///
    /// `P(U, V) = O + (R + V·tan(α))·(cos(U)·XDir + sin(U)·YDir) + V·ZDir`
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (su, cu) = u.sin_cos();
        let r = self.ref_radius + v * self.semi_angle.tan();
        self.pos.location
            + self.pos.x_dir * (r * cu)
            + self.pos.y_dir * (r * su)
            + self.pos.z_dir * v
    }

    /// `D0` — alias for [`value`](Self::value).
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V)` — point and first-order partial derivatives.
    ///
    /// Returns `(P, DU, DV)` where:
    /// - `DU = ∂P/∂U = (R + V·tan(α))·(-sin(U)·XDir + cos(U)·YDir)`
    /// - `DV = ∂P/∂V = tan(α)·(cos(U)·XDir + sin(U)·YDir) + ZDir`
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let (su, cu) = u.sin_cos();
        let r = self.ref_radius + v * self.semi_angle.tan();
        let tan_a = self.semi_angle.tan();

        let p = self.pos.location
            + self.pos.x_dir * (r * cu)
            + self.pos.y_dir * (r * su)
            + self.pos.z_dir * v;

        let du = self.pos.x_dir * (-r * su) + self.pos.y_dir * (r * cu);
        let dv = self.pos.x_dir * (tan_a * cu)
            + self.pos.y_dir * (tan_a * su)
            + self.pos.z_dir;

        (p, du, dv)
    }

    /// `DN(U, V, Nu, Nv)` — derivative of order `(nu, nv)`.
    ///
    /// Let `r(v) = R + v·tan(α)`.
    ///
    /// The mixed and pure derivatives:
    /// - `(1,0)`: `r·(-sin U·X + cos U·Y)`
    /// - `(2,0)`: `r·(-cos U·X - sin U·Y)`
    /// - `(n,0)` for n≥1: `r · (cos(U + n·π/2)·X + sin(U + n·π/2)·Y)`
    /// - `(0,1)`: `tan(α)·(cos U·X + sin U·Y) + Z`
    /// - `(n,1)` for n≥1: `tan(α)·(cos(U + n·π/2)·X + sin(U + n·π/2)·Y)`
    /// - `(0,m)` for m≥2: zero vector (linear in V)
    /// - `(n,m)` for n≥1, m≥2: zero vector
    ///
    /// # Panics
    /// Panics if `nu < 0` or `nv < 0` or `nu + nv < 1`.
    pub fn dn(&self, u: f64, v: f64, nu: i32, nv: i32) -> Vec3 {
        assert!(nu >= 0, "Geom_ConicalSurface::DN: Nu must be >= 0");
        assert!(nv >= 0, "Geom_ConicalSurface::DN: Nv must be >= 0");
        assert!(
            nu + nv >= 1,
            "Geom_ConicalSurface::DN: Nu + Nv must be >= 1"
        );

        if nv >= 2 {
            // All second and higher V-derivatives are zero (surface is linear in V).
            return Pnt::origin();
        }

        let (su, cu) = u.sin_cos();
        let tan_a = self.semi_angle.tan();

        if nu == 0 && nv == 1 {
            // ∂P/∂V = tan(α)·(cos(U)·X + sin(U)·Y) + Z
            return self.pos.x_dir * (tan_a * cu)
                + self.pos.y_dir * (tan_a * su)
                + self.pos.z_dir;
        }

        if nu >= 1 && nv == 0 {
            // nth angular derivative: r · (cos(U + n·π/2)·X + sin(U + n·π/2)·Y)
            let r = self.ref_radius + v * tan_a;
            let phase = u + (nu as f64) * (PI / 2.0);
            let (sp, cp) = phase.sin_cos();
            return self.pos.x_dir * (r * cp) + self.pos.y_dir * (r * sp);
        }

        // nu >= 1 && nv == 1:
        // mixed: d/dV of [r · (cos(U + n·π/2)·X + sin(U + n·π/2)·Y)]
        // = tan(α) · (cos(U + n·π/2)·X + sin(U + n·π/2)·Y)
        let phase = u + (nu as f64) * (PI / 2.0);
        let (sp, cp) = phase.sin_cos();
        self.pos.x_dir * (tan_a * cp) + self.pos.y_dir * (tan_a * sp)
    }

    // ─────────────────────────── normal ─────────────────────────────────────

    /// `Normal(U, V)` — the unit normal at parameter `(U, V)`.
    ///
    /// Returns `(P, N)`.
    ///
    /// The surface normal is `DU × DV` (normalized).
    /// For the cone:
    /// - `DU = r·(-sin U·X + cos U·Y)`
    /// - `DV = tan(α)·(cos U·X + sin U·Y) + Z`
    /// - `DU × DV = r·(cos(α)·(cos U·X + sin U·Y) - sin(α)·Z)` (unnormalized by r)
    ///
    /// The unit normal (pointing outward) is:
    /// `N = cos(α)·(cos U·X + sin U·Y) - sin(α)·Z`
    pub fn normal(&self, u: f64, v: f64) -> (Pnt, Vec3) {
        let (su, cu) = u.sin_cos();
        let tan_a = self.semi_angle.tan();
        let r = self.ref_radius + v * tan_a;

        let p = self.pos.location
            + self.pos.x_dir * (r * cu)
            + self.pos.y_dir * (r * su)
            + self.pos.z_dir * v;

        // DU cross DV:
        // DU = r*(-su*X + cu*Y)
        // DV = tan(a)*(cu*X + su*Y) + Z
        // DU x DV = r * [ (-su*X + cu*Y) x (tan(a)*(cu*X + su*Y) + Z) ]
        // (-su*X + cu*Y) x Z = cu*(Y x Z) + (-su)*(X x Z)
        //                    = cu*X - (-su)*Y  -- actually:
        // X x Z = -Y, Y x Z = X
        // (-su*X) x Z = -su*(X x Z) = -su*(-Y) = su*Y
        // (cu*Y) x Z = cu*(Y x Z) = cu*X
        // so (-su*X + cu*Y) x Z = cu*X + su*Y
        //
        // (-su*X + cu*Y) x (cu*X) = -su*(X x X)*... let's use the cross product directly.
        // Let Xu = -su*X + cu*Y (local), Xv = tan(a)*(cu*X+su*Y) + Z.
        // Cross = Xu x Xv = r * [(-su*X+cu*Y) x (tan(a)*(cu*X+su*Y) + Z)]
        // Expand:
        // = r*[ (-su*X+cu*Y) x (tan(a)*cu*X) + (-su*X+cu*Y) x (tan(a)*su*Y)
        //       + (-su*X+cu*Y) x Z ]
        // (-su*X+cu*Y) x (tan(a)*cu*X) = tan(a)*cu*(-su*(X x X) + cu*(Y x X))
        //   = tan(a)*cu*(cu*(-Z)) = -tan(a)*cu²*Z
        // (-su*X+cu*Y) x (tan(a)*su*Y) = tan(a)*su*(-su*(X x Y) + cu*(Y x Y))
        //   = tan(a)*su*(-su*Z + 0) = -tan(a)*su²*Z
        // (-su*X+cu*Y) x Z = (-su)*(X x Z) + cu*(Y x Z)
        //   = (-su)*(-Y) + cu*(X) = su*Y + cu*X
        // Sum: -tan(a)*cu²*Z - tan(a)*su²*Z + su*Y + cu*X
        //    = -tan(a)*Z + cu*X + su*Y
        //
        // So DU x DV = r*(-tan(a)*Z + cu*X + su*Y)
        // Normalizing (the magnitude is r * sqrt(1 + tan²(a)) = r/cos(a)):
        // Unit normal = cos(a)*(cu*X + su*Y) - sin(a)*Z
        // (for positive r and positive cos(a), pointing outward)

        let (sa, ca) = self.semi_angle.sin_cos();
        let n_raw = self.pos.x_dir * (ca * cu) + self.pos.y_dir * (ca * su)
            - self.pos.z_dir * sa;
        // The sign of r affects orientation; if r < 0 (negative angle region), flip.
        let n = if r >= 0.0 { n_raw } else { -n_raw };

        (p, n.normalized())
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// `Transform(T)` — apply the affine transform `T` in-place.
    ///
    /// The position is transformed; the radius is scaled by the scale factor;
    /// the semi-angle is unchanged (angles are preserved under similarity).
    pub fn transform(&mut self, t: &Trsf) {
        self.pos = self.pos.transformed(t);
        self.ref_radius = (self.ref_radius * t.scale_factor()).abs();
    }

    /// Return a copy of this surface with the transform `T` applied.
    pub fn transformed(&self, t: &Trsf) -> GeomConicalSurface {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// `Copy()` — a new surface equal to this one.
    pub fn copy(&self) -> GeomConicalSurface {
        *self
    }
}
