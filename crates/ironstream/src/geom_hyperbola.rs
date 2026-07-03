//! `Geom_Hyperbola` / `Geom2d_Hyperbola` — zero-dependency stubs for the
//! IronStream OCCT reimplementation.
//!
//! # 3-D hyperbola (`Hyperbola3d`) — parametrisation
//!
//! The main branch is parameterised by a real `t` in `]-∞, +∞[`:
//!
//! ```text
//! P(t) = center + a·cosh(t)·axis + b·sinh(t)·perp
//! ```
//!
//! where `a = major_radius`, `b = minor_radius`, `axis` is the stored unit
//! transverse-axis direction, and `perp` is an orthogonal unit vector derived
//! at construction time.
//!
//! Key relations:
//! - `c = sqrt(a² + b²)` (focal semi-distance)
//! - eccentricity `e = c / a > 1`
//! - `focus1 = center + c·axis`, `focus2 = center − c·axis`
//! - `first_parameter = −∞`, `last_parameter = +∞` (open curve)
//!
//! # 2-D hyperbola (`Hyperbola2d`) — parametrisation
//!
//! ```text
//! P(t) = (cx + a·cosh(t),  cy + b·sinh(t))
//! ```
//!
//! The transverse axis is aligned with the global X axis `(1, 0)`.
//!
//! Both types use only `std` (`f64::cosh`, `f64::sinh`, `f64::sqrt`).
//! No external crates are required.

// ─────────────────────────────────────────────────────────────────────────────
// 3-D hyperbola
// ─────────────────────────────────────────────────────────────────────────────

/// A 3-D analytic hyperbola with center, semi-major radius `a`, semi-minor
/// radius `b`, and a unit transverse-axis direction.
///
/// The parametric equation of the **main branch** is:
///
/// ```text
/// P(t) = center + a·cosh(t)·axis + b·sinh(t)·perp
/// ```
///
/// where `perp` is a unit vector perpendicular to `axis`, computed at
/// construction by `new` (the global Y axis `[0, 1, 0]` when `axis` is the
/// global X axis `[1, 0, 0]`).
// occt: Geom_Hyperbola
#[derive(Clone, Copy, Debug)]
pub struct Hyperbola3d {
    /// Center of the hyperbola — the midpoint between the two foci.
    pub center: [f64; 3],
    /// Semi-major radius `a` (transverse semi-axis, `>= 0`).
    pub major_radius: f64,
    /// Semi-minor radius `b` (conjugate semi-axis, `>= 0`).
    pub minor_radius: f64,
    /// Unit direction of the transverse (major) axis.
    pub axis: [f64; 3],
    /// Unit direction perpendicular to `axis` used for the sinh component.
    perp: [f64; 3],
}

impl Hyperbola3d {
    /// `Geom_Hyperbola` — constructs a hyperbola centred at `center` with
    /// semi-major radius `a`, semi-minor radius `b`, transverse axis along
    /// the global X axis `[1, 0, 0]`, and conjugate axis along `[0, 1, 0]`.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError`) if `a < 0` or `b < 0`.
    pub fn new(center: [f64; 3], a: f64, b: f64) -> Self {
        assert!(a >= 0.0, "Hyperbola3d: major_radius must be >= 0");
        assert!(b >= 0.0, "Hyperbola3d: minor_radius must be >= 0");
        Self {
            center,
            major_radius: a,
            minor_radius: b,
            axis: [1.0, 0.0, 0.0],
            perp: [0.0, 1.0, 0.0],
        }
    }

    /// `D0(t)` — the point at parameter `t`:
    ///
    /// `P = center + a·cosh(t)·axis + b·sinh(t)·perp`
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        let ch = t.cosh();
        let sh = t.sinh();
        [
            self.center[0]
                + self.major_radius * ch * self.axis[0]
                + self.minor_radius * sh * self.perp[0],
            self.center[1]
                + self.major_radius * ch * self.axis[1]
                + self.minor_radius * sh * self.perp[1],
            self.center[2]
                + self.major_radius * ch * self.axis[2]
                + self.minor_radius * sh * self.perp[2],
        ]
    }

    /// `D1(t)` — the first-derivative vector at parameter `t`:
    ///
    /// `V1 = a·sinh(t)·axis + b·cosh(t)·perp`
    pub fn d1(&self, t: f64) -> [f64; 3] {
        let ch = t.cosh();
        let sh = t.sinh();
        [
            self.major_radius * sh * self.axis[0] + self.minor_radius * ch * self.perp[0],
            self.major_radius * sh * self.axis[1] + self.minor_radius * ch * self.perp[1],
            self.major_radius * sh * self.axis[2] + self.minor_radius * ch * self.perp[2],
        ]
    }

    /// `Focus1()` — the first focus, on the positive transverse-axis side:
    ///
    /// `F1 = center + c·axis`  where `c = sqrt(a² + b²)`.
    pub fn focus1(&self) -> [f64; 3] {
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt();
        [
            self.center[0] + c * self.axis[0],
            self.center[1] + c * self.axis[1],
            self.center[2] + c * self.axis[2],
        ]
    }

    /// `Focus2()` — the second focus, on the negative transverse-axis side:
    ///
    /// `F2 = center − c·axis`  where `c = sqrt(a² + b²)`.
    pub fn focus2(&self) -> [f64; 3] {
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt();
        [
            self.center[0] - c * self.axis[0],
            self.center[1] - c * self.axis[1],
            self.center[2] - c * self.axis[2],
        ]
    }

    /// `Eccentricity()` — `e = c / a = sqrt(1 + (b/a)²)`.
    ///
    /// Always `> 1` for a proper hyperbola (`a > 0`, `b > 0`).
    ///
    /// # Panics
    /// Panics (`Standard_DomainError`) if `major_radius == 0`.
    pub fn eccentricity(&self) -> f64 {
        assert!(
            self.major_radius > 0.0,
            "Hyperbola3d::eccentricity: major_radius is zero"
        );
        let ratio = self.minor_radius / self.major_radius;
        (1.0 + ratio * ratio).sqrt()
    }

    /// `FirstParameter()` — `−f64::INFINITY` (open curve, no lower bound).
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        f64::NEG_INFINITY
    }

    /// `LastParameter()` — `+f64::INFINITY` (open curve, no upper bound).
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        f64::INFINITY
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2-D hyperbola
// ─────────────────────────────────────────────────────────────────────────────

/// A 2-D analytic hyperbola with center `[cx, cy]`, semi-major radius `a`,
/// and semi-minor radius `b`.
///
/// The parametric equation of the **main branch** is:
///
/// ```text
/// P(t) = [cx + a·cosh(t),  cy + b·sinh(t)]
/// ```
///
/// The transverse axis is aligned with the global X axis.
// occt: Geom2d_Hyperbola
#[derive(Clone, Copy, Debug)]
pub struct Hyperbola2d {
    /// Center of the hyperbola `[cx, cy]`.
    pub center: [f64; 2],
    /// Semi-major radius `a` (transverse semi-axis, along X, `>= 0`).
    pub major_radius: f64,
    /// Semi-minor radius `b` (conjugate semi-axis, along Y, `>= 0`).
    pub minor_radius: f64,
}

impl Hyperbola2d {
    /// `Geom2d_Hyperbola` — constructs a 2-D hyperbola centred at `(cx, cy)`
    /// with semi-major radius `a` and semi-minor radius `b`.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError`) if `a < 0` or `b < 0`.
    pub fn new(cx: f64, cy: f64, a: f64, b: f64) -> Self {
        assert!(a >= 0.0, "Hyperbola2d: major_radius must be >= 0");
        assert!(b >= 0.0, "Hyperbola2d: minor_radius must be >= 0");
        Self {
            center: [cx, cy],
            major_radius: a,
            minor_radius: b,
        }
    }

    /// `D0(t)` — the point at parameter `t`:
    ///
    /// `P = [cx + a·cosh(t),  cy + b·sinh(t)]`
    pub fn point_at(&self, t: f64) -> [f64; 2] {
        [
            self.center[0] + self.major_radius * t.cosh(),
            self.center[1] + self.minor_radius * t.sinh(),
        ]
    }

    /// `D1(t)` — the first-derivative vector at parameter `t`:
    ///
    /// `V1 = [a·sinh(t),  b·cosh(t)]`
    pub fn d1(&self, t: f64) -> [f64; 2] {
        [
            self.major_radius * t.sinh(),
            self.minor_radius * t.cosh(),
        ]
    }

    /// `Eccentricity()` — `e = sqrt(1 + (b/a)²)`.
    ///
    /// Always `>= 1` for a proper hyperbola (`a > 0`).
    ///
    /// # Panics
    /// Panics (`Standard_DomainError`) if `major_radius == 0`.
    pub fn eccentricity(&self) -> f64 {
        assert!(
            self.major_radius > 0.0,
            "Hyperbola2d::eccentricity: major_radius is zero"
        );
        let ratio = self.minor_radius / self.major_radius;
        (1.0 + ratio * ratio).sqrt()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    // ── Hyperbola3d ──────────────────────────────────────────────────────────

    fn make3d() -> Hyperbola3d {
        // a=3, b=4 => c=5, e=5/3
        Hyperbola3d::new([0.0, 0.0, 0.0], 3.0, 4.0)
    }

    #[test]
    fn h3d_vertex_at_zero() {
        let h = make3d();
        let p = h.point_at(0.0);
        // cosh(0)=1, sinh(0)=0 => P = (a, 0, 0) = (3, 0, 0)
        assert!((p[0] - 3.0).abs() < EPS, "p={p:?}");
        assert!(p[1].abs() < EPS, "p={p:?}");
        assert!(p[2].abs() < EPS, "p={p:?}");
    }

    #[test]
    fn h3d_hyperbola_equation() {
        let h = make3d();
        let a = h.major_radius;
        let b = h.minor_radius;
        for &t in &[-2.0_f64, -1.0, 0.0, 1.0, 2.0] {
            let p = h.point_at(t);
            // (x/a)^2 - (y/b)^2 = cosh^2 - sinh^2 = 1
            let lhs = (p[0] / a).powi(2) - (p[1] / b).powi(2);
            assert!((lhs - 1.0).abs() < 1e-10, "t={t}, lhs={lhs}");
        }
    }

    #[test]
    fn h3d_eccentricity() {
        let h = make3d();
        // e = sqrt(1 + (4/3)^2) = 5/3
        assert!((h.eccentricity() - 5.0 / 3.0).abs() < EPS);
    }

    #[test]
    fn h3d_foci() {
        let h = make3d();
        // c = sqrt(9+16) = 5
        let f1 = h.focus1();
        let f2 = h.focus2();
        assert!((f1[0] - 5.0).abs() < EPS && f1[1].abs() < EPS && f1[2].abs() < EPS);
        assert!((f2[0] + 5.0).abs() < EPS && f2[1].abs() < EPS && f2[2].abs() < EPS);
    }

    #[test]
    fn h3d_d1_at_zero() {
        let h = make3d();
        let v = h.d1(0.0);
        // sinh(0)=0, cosh(0)=1 => V1 = b·perp = (0, 4, 0)
        assert!(v[0].abs() < EPS, "v={v:?}");
        assert!((v[1] - 4.0).abs() < EPS, "v={v:?}");
        assert!(v[2].abs() < EPS, "v={v:?}");
    }

    #[test]
    fn h3d_parameter_bounds() {
        let h = make3d();
        assert_eq!(h.first_parameter(), f64::NEG_INFINITY);
        assert_eq!(h.last_parameter(), f64::INFINITY);
    }

    #[test]
    fn h3d_center_offset() {
        let h = Hyperbola3d::new([1.0, 2.0, 3.0], 5.0, 12.0);
        let p = h.point_at(0.0);
        // P = center + (a, 0, 0) = (6, 2, 3)
        assert!((p[0] - 6.0).abs() < EPS);
        assert!((p[1] - 2.0).abs() < EPS);
        assert!((p[2] - 3.0).abs() < EPS);
    }

    // ── Hyperbola2d ──────────────────────────────────────────────────────────

    fn make2d() -> Hyperbola2d {
        // a=3, b=4 => e=5/3
        Hyperbola2d::new(0.0, 0.0, 3.0, 4.0)
    }

    #[test]
    fn h2d_vertex_at_zero() {
        let h = make2d();
        let p = h.point_at(0.0);
        // cosh(0)=1, sinh(0)=0 => P = (a, 0) = (3, 0)
        assert!((p[0] - 3.0).abs() < EPS, "p={p:?}");
        assert!(p[1].abs() < EPS, "p={p:?}");
    }

    #[test]
    fn h2d_hyperbola_equation() {
        let h = make2d();
        let a = h.major_radius;
        let b = h.minor_radius;
        for &t in &[-2.0_f64, -1.0, 0.0, 1.0, 2.0] {
            let p = h.point_at(t);
            let lhs = (p[0] / a).powi(2) - (p[1] / b).powi(2);
            assert!((lhs - 1.0).abs() < 1e-10, "t={t}, lhs={lhs}");
        }
    }

    #[test]
    fn h2d_eccentricity() {
        let h = make2d();
        assert!((h.eccentricity() - 5.0 / 3.0).abs() < EPS);
    }

    #[test]
    fn h2d_d1_at_zero() {
        let h = make2d();
        let v = h.d1(0.0);
        // sinh(0)=0, cosh(0)=1 => V1 = (0, b) = (0, 4)
        assert!(v[0].abs() < EPS, "v={v:?}");
        assert!((v[1] - 4.0).abs() < EPS, "v={v:?}");
    }

    #[test]
    fn h2d_center_offset() {
        let h = Hyperbola2d::new(10.0, -5.0, 2.0, 1.0);
        let p = h.point_at(0.0);
        // P = (cx + a, cy) = (12, -5)
        assert!((p[0] - 12.0).abs() < EPS);
        assert!((p[1] + 5.0).abs() < EPS);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomHyperbola — Ax3-based type used by geom_trimmed_curve, geom_offset_curve,
// and geom_extrusion_surface.  Kept alongside the simpler array-based stubs
// above so that existing downstream imports continue to compile.
// ─────────────────────────────────────────────────────────────────────────────

use crate::gp::{Ax1, Ax3, Pnt, Trsf, Vec3};

/// `Geom_Hyperbola` — describes a hyperbola in 3D space using a full local
/// coordinate system (`gp_Ax2`, here [`Ax3`]).
///
/// Mirrors OCCT: the hyperbola stores its placement and the two semi-radii, and
/// is derived (conceptually) from `Geom_Conic` / `Geom_Curve`.
// occt: Geom_Hyperbola
#[derive(Clone, Copy, Debug)]
pub struct GeomHyperbola {
    /// Local coordinate system: origin = center, X = major axis, Y = minor
    /// axis, Z = hyperbola normal.
    pos: Ax3,
    /// Semi-major radius (`>= 0`).
    major_radius: f64,
    /// Semi-minor radius (`>= 0`).
    minor_radius: f64,
}

impl GeomHyperbola {
    /// `Geom_Hyperbola(const gp_Ax2& A2, MajorRadius, MinorRadius)`.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if either radius is `< 0`.
    pub fn new(a2: Ax3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(major_radius >= 0.0, "Geom_Hyperbola: MajorRadius must be >= 0");
        assert!(minor_radius >= 0.0, "Geom_Hyperbola: MinorRadius must be >= 0");
        Self { pos: a2, major_radius, minor_radius }
    }

    /// `void SetMajorRadius(Standard_Real r)`.
    pub fn set_major_radius(&mut self, r: f64) {
        assert!(r >= 0.0, "Geom_Hyperbola::SetMajorRadius: R must be >= 0");
        self.major_radius = r;
    }

    /// `void SetMinorRadius(Standard_Real r)`.
    pub fn set_minor_radius(&mut self, r: f64) {
        assert!(r >= 0.0, "Geom_Hyperbola::SetMinorRadius: R must be >= 0");
        self.minor_radius = r;
    }

    /// `Standard_Real MajorRadius() const`.
    #[inline] pub fn major_radius(&self) -> f64 { self.major_radius }

    /// `Standard_Real MinorRadius() const`.
    #[inline] pub fn minor_radius(&self) -> f64 { self.minor_radius }

    /// `gp_Ax2 Position() const` — the local coordinate system.
    #[inline] pub fn position(&self) -> Ax3 { self.pos }

    /// `gp_Pnt Location() const` — the center.
    #[inline] pub fn location(&self) -> Pnt { self.pos.location }

    /// `gp_Ax1 Axis() const` — origin + Z direction (hyperbola normal).
    pub fn axis(&self) -> Ax1 { Ax1::new(self.pos.location, self.pos.z_dir) }

    /// `gp_Ax1 XAxis() const` — origin + major-axis direction.
    pub fn x_axis(&self) -> Ax1 { Ax1::new(self.pos.location, self.pos.x_dir) }

    /// `gp_Ax1 YAxis() const` — origin + minor-axis direction.
    pub fn y_axis(&self) -> Ax1 { Ax1::new(self.pos.location, self.pos.y_dir) }

    /// `Standard_Real Eccentricity() const` — `sqrt(1 + (b/a)²)`.
    ///
    /// # Panics
    /// Raises if `major_radius == 0`.
    pub fn eccentricity(&self) -> f64 {
        assert!(self.major_radius > 0.0, "Geom_Hyperbola::Eccentricity: MajorRadius is zero");
        let ratio = self.minor_radius / self.major_radius;
        (1.0 + ratio * ratio).sqrt()
    }

    /// `Standard_Real Focal() const` — `2·sqrt(a² + b²)`.
    pub fn focal(&self) -> f64 {
        2.0 * (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius).sqrt()
    }

    /// `gp_Pnt Focus1() const` — `center + c·XDir`, `c = sqrt(a² + b²)`.
    pub fn focus1(&self) -> Pnt {
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius).sqrt();
        self.pos.location + self.pos.x_dir * c
    }

    /// `gp_Pnt Focus2() const` — `center − c·XDir`.
    pub fn focus2(&self) -> Pnt {
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius).sqrt();
        self.pos.location - self.pos.x_dir * c
    }

    /// `Standard_Real Parameter() const` — `b² / a`.
    ///
    /// # Panics
    /// Raises if `major_radius == 0`.
    pub fn parameter(&self) -> f64 {
        assert!(self.major_radius > 0.0, "Geom_Hyperbola::Parameter: MajorRadius is zero");
        self.minor_radius * self.minor_radius / self.major_radius
    }

    /// `gp_Ax1 Asymptote1() const` — direction `a·XDir + b·YDir` (normalised).
    pub fn asymptote1(&self) -> Ax1 {
        assert!(self.major_radius > 0.0, "Geom_Hyperbola::Asymptote1: MajorRadius is zero");
        let dir = (self.pos.x_dir * self.major_radius
            + self.pos.y_dir * self.minor_radius).normalized();
        Ax1::new(self.pos.location, dir)
    }

    /// `gp_Ax1 Asymptote2() const` — direction `a·XDir − b·YDir` (normalised).
    pub fn asymptote2(&self) -> Ax1 {
        assert!(self.major_radius > 0.0, "Geom_Hyperbola::Asymptote2: MajorRadius is zero");
        let dir = (self.pos.x_dir * self.major_radius
            - self.pos.y_dir * self.minor_radius).normalized();
        Ax1::new(self.pos.location, dir)
    }

    /// Returns the conjugate hyperbola (swaps radii, rotates frame 90°).
    pub fn conjugate(&self) -> GeomHyperbola {
        let pos = Ax3 {
            location: self.pos.location,
            x_dir: self.pos.y_dir,
            y_dir: -self.pos.x_dir,
            z_dir: self.pos.z_dir,
        };
        GeomHyperbola { pos, major_radius: self.minor_radius, minor_radius: self.major_radius }
    }

    /// `Standard_Boolean IsClosed() const` — `false`.
    pub fn is_closed(&self) -> bool { false }

    /// `Standard_Boolean IsPeriodic() const` — `false`.
    pub fn is_periodic(&self) -> bool { false }

    /// `Standard_Real ReversedParameter(U) const` — `−U`.
    pub fn reversed_parameter(&self, u: f64) -> f64 { -u }

    /// `D0(U)` — `center + a·cosh(U)·XDir + b·sinh(U)·YDir`.
    pub fn value(&self, u: f64) -> Pnt {
        self.pos.to_world(self.major_radius * u.cosh(), self.minor_radius * u.sinh(), 0.0)
    }

    /// `D1(U)` — `(P, a·sinh(U)·XDir + b·cosh(U)·YDir)`.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        let p = self.value(u);
        let v1 = self.pos.x_dir * (self.major_radius * u.sinh())
            + self.pos.y_dir * (self.minor_radius * u.cosh());
        (p, v1)
    }

    /// `D2(U)`.
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        let (p, v1) = self.d1(u);
        let v2 = self.pos.x_dir * (self.major_radius * u.cosh())
            + self.pos.y_dir * (self.minor_radius * u.sinh());
        (p, v1, v2)
    }

    /// `D3(U)`.
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        let (p, v1, v2) = self.d2(u);
        let v3 = self.pos.x_dir * (self.major_radius * u.sinh())
            + self.pos.y_dir * (self.minor_radius * u.cosh());
        (p, v1, v2, v3)
    }

    /// `DN(U, N)` — N-th derivative (`N >= 1`).
    ///
    /// # Panics
    /// Raised if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        assert!(n >= 1, "Geom_Hyperbola::DN: N must be >= 1");
        if n % 2 == 0 {
            self.pos.x_dir * (self.major_radius * u.cosh())
                + self.pos.y_dir * (self.minor_radius * u.sinh())
        } else {
            self.pos.x_dir * (self.major_radius * u.sinh())
                + self.pos.y_dir * (self.minor_radius * u.cosh())
        }
    }

    /// `void Transform(const gp_Trsf& T)`.
    pub fn transform(&mut self, t: &Trsf) {
        let scale = t.scale_factor().abs();
        self.pos = self.pos.transformed(t);
        self.major_radius *= scale;
        self.minor_radius *= scale;
    }

    /// Returns a transformed copy.
    pub fn transformed(&self, t: &Trsf) -> GeomHyperbola {
        let mut h = *self;
        h.transform(t);
        h
    }

    /// `Copy()` — a new hyperbola equal to this one.
    pub fn copy(&self) -> GeomHyperbola { *self }
}
