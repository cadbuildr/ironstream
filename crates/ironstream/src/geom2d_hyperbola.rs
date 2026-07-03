//! `Geom2d_Hyperbola` -- analytic 2D hyperbola, faithfully reproducing
//! OpenCascade's `Geom2d_Hyperbola` (a `Geom2d_Conic` in package TKG2d).
//!
//! A hyperbola in the plane is positioned by a local coordinate system
//! (`gp_Ax22d`, here [`Ax22d2`]): its **origin** is the **center**; its "X
//! Direction" points along the **major axis** (transverse axis); its "Y
//! Direction" is perpendicular to it in the local plane.
//!
//! The main branch of the hyperbola is parameterised by a real `U` in
//! `]-∞, +∞[`:
//!
//! ```text
//! P(U) = O + a·cosh(U)·XDir + b·sinh(U)·YDir
//! ```
//!
//! where `O` is the center, `a = MajorRadius`, `b = MinorRadius`, and
//! `XDir`, `YDir` are the two planar directions of the local coordinate
//! system.
//!
//! Key relations:
//! - `c = sqrt(a² + b²)` (focal distance from center)
//! - Eccentricity `e = c / a = sqrt(1 + (b/a)²)` (always > 1 for a proper
//!   hyperbola)
//! - Foci: `F1 = O + c·XDir`, `F2 = O − c·XDir`
//! - Semi-latus rectum (parameter): `p = b² / a`
//! - Derivatives cycle every 2 steps (`cosh` ↔ `sinh`):
//!   - `D'(U)  = a·sinh(U)·XDir + b·cosh(U)·YDir`  (odd orders)
//!   - `D''(U) = a·cosh(U)·XDir + b·sinh(U)·YDir`  (even orders)
//!
//! This is identical to OCCT's
//! `src/ModelingData/TKG2d/Geom2d/Geom2d_Hyperbola.cxx`.
//!
//! Builds on the existing `gp2d` / `geom2d_circle` API (zero third-party deps).

use crate::geom2d_circle::{Ax22d2, Dir2d};
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d, Vec2d};
use crate::precision::INFINITE;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers: apply the linear (non-translating) part of a Trsf2d to a vector.
// ─────────────────────────────────────────────────────────────────────────────

#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// The uniform scale factor of a 2D transform: `sqrt(|det(linear part)|)`.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

// ─────────────────────────────────────────────────────────────────────────────
// gp_Hypr2d  (elementary, non-parameterised value type)
// ─────────────────────────────────────────────────────────────────────────────

/// `gp_Hypr2d` — an elementary, non-parameterised 2D hyperbola: a local frame
/// (`gp_Ax22d`) whose origin is the center, plus major and minor radii.
///
/// Both radii must be non-negative; the major radius may be zero (degenerate)
/// but the minor radius must be `>= 0`.
#[derive(Clone, Copy, Debug)]
// occt: gp_Hypr2d
pub struct Hypr2d {
    position: Ax22d2,
    major_radius: f64,
    minor_radius: f64,
}

impl Hypr2d {
    /// `gp_Hypr2d(const gp_Ax2d& MajorAxis, MajorRadius, MinorRadius, Sense)`.
    ///
    /// The "X Direction" of the local frame is the major axis; the frame is
    /// direct if `sense` is true.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if `major_radius < 0` or
    /// `minor_radius < 0`.
    pub fn new(major_axis: Ax2d, major_radius: f64, minor_radius: f64, sense: bool) -> Self {
        assert!(
            major_radius >= 0.0,
            "gp_Hypr2d: MajorRadius must be >= 0"
        );
        assert!(
            minor_radius >= 0.0,
            "gp_Hypr2d: MinorRadius must be >= 0"
        );
        Self {
            position: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, sense),
            major_radius,
            minor_radius,
        }
    }

    /// `gp_Hypr2d(const gp_Ax22d& A, MajorRadius, MinorRadius)` — from a full
    /// local frame.
    ///
    /// # Panics
    /// Raised if `major_radius < 0` or `minor_radius < 0`.
    pub fn from_axis(position: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            major_radius >= 0.0,
            "gp_Hypr2d: MajorRadius must be >= 0"
        );
        assert!(
            minor_radius >= 0.0,
            "gp_Hypr2d: MinorRadius must be >= 0"
        );
        Self {
            position,
            major_radius,
            minor_radius,
        }
    }

    /// The major radius.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// The minor radius.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// The center (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.position.location()
    }

    /// The local coordinate system.
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.position
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2d_Hyperbola
// ─────────────────────────────────────────────────────────────────────────────

/// `Geom2d_Hyperbola` — a parameterised analytic 2D hyperbola.
///
/// Stores its local coordinate system (`Ax22d2`, the OCCT `gp_Ax22d pos` of
/// `Geom2d_Conic`) and its major/minor radii. The parameterisation is:
///
/// ```text
/// P(U) = O + a·cosh(U)·XDir + b·sinh(U)·YDir
/// ```
///
/// where `a = MajorRadius`, `b = MinorRadius`.
///
/// The curve is **open** and **not periodic**: first parameter = `−INFINITE`,
/// last parameter = `+INFINITE`.
// occt: Geom2d_Hyperbola
#[derive(Clone, Copy, Debug)]
pub struct Geom2dHyperbola {
    /// Local coordinate system: origin = center, X = major axis, Y = minor axis.
    pos: Ax22d2,
    /// Semi-major radius (≥ 0).
    major_radius: f64,
    /// Semi-minor radius (≥ 0).
    minor_radius: f64,
}

impl Geom2dHyperbola {
    // ──────────────────────────────────────── Constructors ───────────────────

    /// `Geom2d_Hyperbola(const gp_Ax2d& MajorAxis, Standard_Real MajorRadius,
    /// Standard_Real MinorRadius, Standard_Boolean Sense = Standard_True)`.
    ///
    /// Constructs a hyperbola with:
    /// - center at the origin of `major_axis`,
    /// - major axis along the "X Direction" of `major_axis`,
    /// - direct (CCW) frame if `sense` is `true`.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if `major_radius < 0` or
    /// `minor_radius < 0`.
    pub fn new(major_axis: Ax2d, major_radius: f64, minor_radius: f64, sense: bool) -> Self {
        assert!(
            major_radius >= 0.0,
            "Geom2d_Hyperbola: MajorRadius must be >= 0"
        );
        assert!(
            minor_radius >= 0.0,
            "Geom2d_Hyperbola: MinorRadius must be >= 0"
        );
        Self {
            pos: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, sense),
            major_radius,
            minor_radius,
        }
    }

    /// `Geom2d_Hyperbola(const gp_Ax22d& Axis, Standard_Real MajorRadius,
    /// Standard_Real MinorRadius)` — from a full local frame.
    ///
    /// # Panics
    /// Raised if `major_radius < 0` or `minor_radius < 0`.
    pub fn from_axis(axis: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            major_radius >= 0.0,
            "Geom2d_Hyperbola: MajorRadius must be >= 0"
        );
        assert!(
            minor_radius >= 0.0,
            "Geom2d_Hyperbola: MinorRadius must be >= 0"
        );
        Self {
            pos: axis,
            major_radius,
            minor_radius,
        }
    }

    /// `Geom2d_Hyperbola(const gp_Hypr2d& H)` — by conversion of an elementary
    /// hyperbola.
    pub fn from_hypr2d(h: Hypr2d) -> Self {
        Self {
            pos: h.position(),
            major_radius: h.major_radius(),
            minor_radius: h.minor_radius(),
        }
    }

    // ──────────────────────────────────────── Setters ────────────────────────

    /// `void SetMajorRadius(Standard_Real MajorRadius)`.
    ///
    /// # Panics
    /// Raised if `r < 0`.
    pub fn set_major_radius(&mut self, r: f64) {
        assert!(
            r >= 0.0,
            "Geom2d_Hyperbola::SetMajorRadius: MajorRadius must be >= 0"
        );
        self.major_radius = r;
    }

    /// `void SetMinorRadius(Standard_Real MinorRadius)`.
    ///
    /// # Panics
    /// Raised if `r < 0`.
    pub fn set_minor_radius(&mut self, r: f64) {
        assert!(
            r >= 0.0,
            "Geom2d_Hyperbola::SetMinorRadius: MinorRadius must be >= 0"
        );
        self.minor_radius = r;
    }

    /// `void SetHypr2d(const gp_Hypr2d& H)` — convert an elementary hyperbola
    /// into this one.
    pub fn set_hypr2d(&mut self, h: Hypr2d) {
        self.pos = h.position();
        self.major_radius = h.major_radius();
        self.minor_radius = h.minor_radius();
    }

    // ──────────────────────────────────────── Getters ────────────────────────

    /// `Standard_Real MajorRadius() const`.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// `Standard_Real MinorRadius() const`.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// `gp_Ax22d Position() const` — the local coordinate system.
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// `gp_Pnt2d Location() const` — the center (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// `gp_Ax2d XAxis() const` — the major axis: center + X direction.
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.x_direction())
    }

    /// `gp_Ax2d YAxis() const` — the minor axis: center + Y direction.
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.y_direction())
    }

    /// The "X Direction" of the local frame (major axis direction).
    #[inline]
    pub fn x_direction(&self) -> Dir2d {
        self.pos.x_direction()
    }

    /// The "Y Direction" of the local frame (minor axis direction).
    #[inline]
    pub fn y_direction(&self) -> Dir2d {
        self.pos.y_direction()
    }

    /// `gp_Hypr2d Hypr2d() const` — the equivalent non-persistent hyperbola.
    pub fn hypr2d(&self) -> Hypr2d {
        Hypr2d::from_axis(self.pos, self.major_radius, self.minor_radius)
    }

    // ──────────────────────────────────────── Conic properties ───────────────

    /// `Standard_Real Eccentricity() const`.
    ///
    /// Returns `sqrt(1 + (b/a)²)` where `a` = major radius, `b` = minor radius.
    /// For a non-degenerate hyperbola this is always ≥ 1.
    ///
    /// # Panics
    /// Raises (`Standard_DomainError`) if `major_radius == 0`.
    pub fn eccentricity(&self) -> f64 {
        assert!(
            self.major_radius > 0.0,
            "Geom2d_Hyperbola::Eccentricity: MajorRadius is zero"
        );
        let ratio = self.minor_radius / self.major_radius;
        (1.0 + ratio * ratio).sqrt()
    }

    /// `Standard_Real Focal() const`.
    ///
    /// Returns the distance between the two foci: `2·c` where
    /// `c = sqrt(a² + b²)`.
    pub fn focal(&self) -> f64 {
        2.0 * (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt()
    }

    /// `gp_Pnt2d Focus1() const` — the first focus, at distance `c` along +X.
    ///
    /// `F1 = center + c·XDir` where `c = sqrt(a² + b²)`.
    pub fn focus1(&self) -> Pnt2d {
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt();
        self.pos.location() + self.pos.x_direction() * c
    }

    /// `gp_Pnt2d Focus2() const` — the second focus, at distance `c` along −X.
    ///
    /// `F2 = center − c·XDir`.
    pub fn focus2(&self) -> Pnt2d {
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt();
        self.pos.location() - self.pos.x_direction() * c
    }

    /// `Standard_Real Parameter() const` — the semi-latus rectum `p = b² / a`.
    ///
    /// # Panics
    /// Raises if `major_radius == 0`.
    pub fn parameter(&self) -> f64 {
        assert!(
            self.major_radius > 0.0,
            "Geom2d_Hyperbola::Parameter: MajorRadius is zero"
        );
        self.minor_radius * self.minor_radius / self.major_radius
    }

    /// `gp_Ax2d Asymptote1() const`.
    ///
    /// The first asymptote passes through the center with direction
    /// `a·XDir + b·YDir` (normalised). Returns the line as an [`Ax2d`].
    ///
    /// # Panics
    /// Raises if `major_radius == 0`.
    pub fn asymptote1(&self) -> Ax2d {
        assert!(
            self.major_radius > 0.0,
            "Geom2d_Hyperbola::Asymptote1: MajorRadius is zero"
        );
        let dir = (self.pos.x_direction() * self.major_radius
            + self.pos.y_direction() * self.minor_radius)
            .normalized();
        Ax2d::new(self.pos.location(), dir)
    }

    /// `gp_Ax2d Asymptote2() const`.
    ///
    /// The second asymptote direction is `a·XDir − b·YDir` (normalised).
    ///
    /// # Panics
    /// Raises if `major_radius == 0`.
    pub fn asymptote2(&self) -> Ax2d {
        assert!(
            self.major_radius > 0.0,
            "Geom2d_Hyperbola::Asymptote2: MajorRadius is zero"
        );
        let dir = (self.pos.x_direction() * self.major_radius
            - self.pos.y_direction() * self.minor_radius)
            .normalized();
        Ax2d::new(self.pos.location(), dir)
    }

    /// `gp_Ax2d Directrix1() const`.
    ///
    /// The first directrix is the line perpendicular to the major axis at
    /// distance `a / e = a² / c` from the center, on the `+X` side.
    ///
    /// # Panics
    /// Raises if `major_radius == 0` (undefined eccentricity).
    pub fn directrix1(&self) -> Ax2d {
        assert!(
            self.major_radius > 0.0,
            "Geom2d_Hyperbola::Directrix1: MajorRadius is zero"
        );
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt();
        // d = a / e = a * a / c
        let d = self.major_radius * self.major_radius / c;
        let loc = self.pos.location() + self.pos.x_direction() * d;
        Ax2d::new(loc, self.pos.y_direction())
    }

    /// `gp_Ax2d Directrix2() const`.
    ///
    /// Symmetric of `Directrix1` about the center (on the `−X` side).
    ///
    /// # Panics
    /// Raises if `major_radius == 0`.
    pub fn directrix2(&self) -> Ax2d {
        assert!(
            self.major_radius > 0.0,
            "Geom2d_Hyperbola::Directrix2: MajorRadius is zero"
        );
        let c = (self.major_radius * self.major_radius
            + self.minor_radius * self.minor_radius)
            .sqrt();
        let d = self.major_radius * self.major_radius / c;
        let loc = self.pos.location() - self.pos.x_direction() * d;
        Ax2d::new(loc, self.pos.y_direction())
    }

    /// `Geom2d_Hyperbola ConjugateBranch1()` — the conjugate hyperbola (swaps
    /// major / minor axes and radii): `b·cosh(U)·YDir + a·sinh(U)·XDir`.
    pub fn conjugate(&self) -> Geom2dHyperbola {
        // Swap X↔Y, radii swap: new X = old Y, new Y = −old X.
        let new_x = self.pos.y_direction();
        let new_y_hint = -(self.pos.x_direction());
        let conj_pos = Ax22d2::new(self.pos.location(), new_x, new_y_hint);
        Geom2dHyperbola {
            pos: conj_pos,
            major_radius: self.minor_radius,
            minor_radius: self.major_radius,
        }
    }

    // ──────────────────────────────────────── Curve interface ────────────────

    /// `Standard_Real FirstParameter() const` — `−Precision::Infinite()`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        -INFINITE
    }

    /// `Standard_Real LastParameter() const` — `+Precision::Infinite()`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        INFINITE
    }

    /// `Standard_Boolean IsClosed() const` — always `false`.
    #[inline]
    pub fn is_closed(&self) -> bool {
        false
    }

    /// `Standard_Boolean IsPeriodic() const` — always `false`.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// `Standard_Real ReversedParameter(Standard_Real U) const` — returns `−U`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

    // ──────────────────────────────────────── Evaluation ─────────────────────

    /// `D0(U, P)` — the point at parameter `U`:
    ///
    /// ```text
    /// P = O + a·cosh(U)·XDir + b·sinh(U)·YDir
    /// ```
    pub fn d0(&self, u: f64) -> Pnt2d {
        self.pos.location()
            + self.pos.x_direction() * (self.major_radius * u.cosh())
            + self.pos.y_direction() * (self.minor_radius * u.sinh())
    }

    /// `Value(U)` — alias for [`d0`](Self::d0).
    #[inline]
    pub fn value(&self, u: f64) -> Pnt2d {
        self.d0(u)
    }

    /// `D1(U, P, V1)` — the point and first derivative.
    ///
    /// ```text
    /// V1 = a·sinh(U)·XDir + b·cosh(U)·YDir
    /// ```
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let p = self.d0(u);
        let v1 = self.pos.x_direction() * (self.major_radius * u.sinh())
            + self.pos.y_direction() * (self.minor_radius * u.cosh());
        (p, v1)
    }

    /// `D2(U, P, V1, V2)` — the point and first two derivatives.
    ///
    /// ```text
    /// V2 = a·cosh(U)·XDir + b·sinh(U)·YDir
    /// ```
    pub fn d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let (p, v1) = self.d1(u);
        let v2 = self.pos.x_direction() * (self.major_radius * u.cosh())
            + self.pos.y_direction() * (self.minor_radius * u.sinh());
        (p, v1, v2)
    }

    /// `D3(U, P, V1, V2, V3)` — the point and first three derivatives.
    ///
    /// ```text
    /// V3 = a·sinh(U)·XDir + b·cosh(U)·YDir   (same as V1)
    /// ```
    pub fn d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let (p, v1, v2) = self.d2(u);
        let v3 = self.pos.x_direction() * (self.major_radius * u.sinh())
            + self.pos.y_direction() * (self.minor_radius * u.cosh());
        (p, v1, v2, v3)
    }

    /// `DN(U, N)` — the N-th derivative vector (`N ≥ 1`).
    ///
    /// Differentiation of `(a·cosh(U), b·sinh(U))` cycles every 2 steps:
    /// - **even** `N`: `a·cosh(U)·XDir + b·sinh(U)·YDir`
    /// - **odd**  `N`: `a·sinh(U)·XDir + b·cosh(U)·YDir`
    ///
    /// # Panics
    /// Raised (`Standard_RangeError`) if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_Hyperbola::DN: N must be >= 1");
        if n % 2 == 0 {
            // even: same as the displacement from center
            self.pos.x_direction() * (self.major_radius * u.cosh())
                + self.pos.y_direction() * (self.minor_radius * u.sinh())
        } else {
            // odd: tangent-like
            self.pos.x_direction() * (self.major_radius * u.sinh())
                + self.pos.y_direction() * (self.minor_radius * u.cosh())
        }
    }

    // ──────────────────────────────────────── Orientation ────────────────────

    /// `Reverse()` — reverses the orientation of this hyperbola in place.
    ///
    /// Reversing the parameterisation direction negates the "Y Direction" (the
    /// minor-axis direction). The shape and the "X Direction" are unchanged.
    /// The local frame handedness changes accordingly.
    pub fn reverse(&mut self) {
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let current_direct = xd.cross(yd) >= 0.0;
        self.pos = Ax22d2::from_x_axis(self.pos.location(), xd, !current_direct);
    }

    /// `Reversed()` — returns a copy with the orientation reversed.
    pub fn reversed(&self) -> Geom2dHyperbola {
        let mut c = *self;
        c.reverse();
        c
    }

    // ──────────────────────────────────────── Transform ──────────────────────

    /// `Transform(const gp_Trsf2d& T)` — applies the transformation `T` in
    /// place.
    ///
    /// The center and directions are mapped through `T`; the radii are scaled
    /// by `|ScaleFactor(T)|`.
    pub fn transform(&mut self, t: &Trsf2d) {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        let new_x = apply_vector_2d(t, self.pos.x_direction());
        let new_y_hint = apply_vector_2d(t, self.pos.y_direction());
        self.pos = Ax22d2::new(new_loc, new_x, new_y_hint);
        self.major_radius *= scale;
        self.minor_radius *= scale;
    }

    /// `Transformed(T)` — returns a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Geom2dHyperbola {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// `Copy()` — a deep, independent copy.
    #[inline]
    pub fn copy(&self) -> Geom2dHyperbola {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::CONFUSION;
    use std::f64::consts::PI;

    fn make_hyperbola() -> Geom2dHyperbola {
        // a=3, b=4 => c=5, e=5/3
        Geom2dHyperbola::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            3.0,
            4.0,
            true,
        )
    }

    #[test]
    fn vertex_at_u_zero() {
        let h = make_hyperbola();
        let p = h.value(0.0);
        // cosh(0)=1, sinh(0)=0 => P = (a, 0) = (3, 0)
        assert!((p.x - 3.0).abs() < CONFUSION && p.y.abs() < CONFUSION, "{p:?}");
    }

    #[test]
    fn eccentricity() {
        let h = make_hyperbola();
        // e = sqrt(1 + (4/3)^2) = sqrt(1 + 16/9) = sqrt(25/9) = 5/3
        let e = h.eccentricity();
        assert!((e - 5.0 / 3.0).abs() < CONFUSION, "e={e}");
    }

    #[test]
    fn foci_positions() {
        let h = make_hyperbola();
        // c = sqrt(9+16) = 5
        let f1 = h.focus1();
        let f2 = h.focus2();
        assert!((f1.x - 5.0).abs() < CONFUSION && f1.y.abs() < CONFUSION, "{f1:?}");
        assert!((f2.x + 5.0).abs() < CONFUSION && f2.y.abs() < CONFUSION, "{f2:?}");
    }

    #[test]
    fn focal_distance() {
        let h = make_hyperbola();
        // 2*c = 10
        assert!((h.focal() - 10.0).abs() < CONFUSION);
    }

    #[test]
    fn not_closed_or_periodic() {
        let h = make_hyperbola();
        assert!(!h.is_closed());
        assert!(!h.is_periodic());
    }

    #[test]
    fn d1_at_u0() {
        let h = make_hyperbola();
        let (p, v1) = h.d1(0.0);
        // P = (3,0); V1 = a*sinh(0)*X + b*cosh(0)*Y = (0, b) = (0,4)
        assert!((p.x - 3.0).abs() < CONFUSION && p.y.abs() < CONFUSION);
        assert!(v1.x.abs() < CONFUSION && (v1.y - 4.0).abs() < CONFUSION, "{v1:?}");
    }

    #[test]
    fn dn_cycling() {
        let h = make_hyperbola();
        let u = 1.0_f64;
        let d1 = h.dn(u, 1);
        let d3 = h.dn(u, 3);
        // odd derivatives are the same
        assert!((d1.x - d3.x).abs() < CONFUSION && (d1.y - d3.y).abs() < CONFUSION);
        let d2 = h.dn(u, 2);
        let d4 = h.dn(u, 4);
        // even derivatives are the same
        assert!((d2.x - d4.x).abs() < CONFUSION && (d2.y - d4.y).abs() < CONFUSION);
    }

    #[test]
    fn reversed_parameter() {
        let h = make_hyperbola();
        let u = 1.5;
        let pt_fwd = h.value(u);
        let rev = h.reversed();
        let pt_rev = rev.value(h.reversed_parameter(u));
        // The point on the reversed curve at -u should match original at u
        // reversed_parameter(-u) => -(-u) = u
        // value on reversed curve at u = original d0 at u with y flipped
        // Actually the shape is preserved, just orientation of Y flips.
        // P(U) = a*cosh(U)*X + b*sinh(U)*Y_orig; reversed: Y_new = -Y_orig
        // P_rev(-U) = a*cosh(-U)*X + b*sinh(-U)*Y_new
        //           = a*cosh(U)*X - b*sinh(U)*(-Y_orig) = a*cosh(U)*X + b*sinh(U)*Y_orig
        // So the x coordinates match; let's just check x equals
        assert!((pt_fwd.x - pt_rev.x).abs() < CONFUSION, "x: {} vs {}", pt_fwd.x, pt_rev.x);
    }

    #[test]
    fn transform_translation() {
        let mut h = make_hyperbola();
        h.transform(&Trsf2d::translation(Pnt2d::new(10.0, 20.0)));
        assert!((h.location().x - 10.0).abs() < CONFUSION);
        assert!((h.location().y - 20.0).abs() < CONFUSION);
        // radii unchanged by pure translation
        assert!((h.major_radius() - 3.0).abs() < CONFUSION);
        assert!((h.minor_radius() - 4.0).abs() < CONFUSION);
    }

    #[test]
    fn parameter_semi_latus_rectum() {
        let h = make_hyperbola();
        // p = b^2 / a = 16/3
        assert!((h.parameter() - 16.0 / 3.0).abs() < CONFUSION);
    }

    #[test]
    fn asymptotes() {
        let h = make_hyperbola();
        // Asym1 dir: (a, b)/c = (3, 4)/5 = (0.6, 0.8)
        let a1 = h.asymptote1();
        assert!((a1.direction.x - 0.6).abs() < CONFUSION, "{:?}", a1.direction);
        assert!((a1.direction.y - 0.8).abs() < CONFUSION, "{:?}", a1.direction);
        // Asym2 dir: (a, -b)/c = (3, -4)/5 = (0.6, -0.8)
        let a2 = h.asymptote2();
        assert!((a2.direction.x - 0.6).abs() < CONFUSION, "{:?}", a2.direction);
        assert!((a2.direction.y + 0.8).abs() < CONFUSION, "{:?}", a2.direction);
    }

    #[test]
    fn d2_equals_position_offset() {
        // V2(U) = a*cosh(U)*XDir + b*sinh(U)*YDir = P(U) - O
        let h = make_hyperbola();
        let u = 0.7_f64;
        let (_p, _v1, v2) = h.d2(u);
        let offset = h.value(u) - h.location();
        assert!((v2.x - offset.x).abs() < CONFUSION);
        assert!((v2.y - offset.y).abs() < CONFUSION);
    }

    #[test]
    fn hyperbola_equation_satisfied() {
        // On the main branch: (x/a)^2 - (y/b)^2 = 1
        let h = make_hyperbola();
        let a = h.major_radius();
        let b = h.minor_radius();
        for u in [-2.0_f64, -1.0, 0.0, 1.0, 2.0] {
            let p = h.value(u);
            let lhs = (p.x / a).powi(2) - (p.y / b).powi(2);
            assert!((lhs - 1.0).abs() < 1e-10, "u={u}, lhs={lhs}");
        }
    }

    // suppress unused import warning in the unused PI import
    #[allow(unused)]
    const _: f64 = PI;
}
