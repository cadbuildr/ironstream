//! `Geom2d_Parabola` -- analytic 2D parabola, faithfully reproducing
//! OpenCascade's `Geom2d_Parabola` (a `Geom2d_Conic` in package TKG2d).
//!
//! A parabola in the plane is positioned by a local coordinate system
//! (`gp_Ax22d`, here [`Ax22d2`]): its **origin** is the **vertex** of the
//! parabola; its "X Direction" points toward the focus (the axis of symmetry);
//! its "Y Direction" is tangent to the parabola at the vertex and perpendicular
//! to the axis.
//!
//! The parabola is parameterised by a real `U` in `]-∞, +∞[`:
//!
//! ```text
//! P(U) = O + (U² / (2*p)) * XDir + U * YDir
//! ```
//!
//! where `O` is the vertex, `p = 2 * FocalLength` is the **parameter** of the
//! parabola (the semi-latus rectum / ordinate at the focus), `XDir` and `YDir`
//! are the two planar directions of the local coordinate system.
//!
//! This formulation is identical to OCCT's
//! `src/ModelingData/TKG2d/Geom2d/Geom2d_Parabola.cxx`.
//!
//! Builds on the existing `gp2d` / `geom2d_circle` API (zero third-party deps).

use crate::geom2d_circle::{Ax22d2, Dir2d};
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d, Vec2d};
use crate::precision::INFINITE;

// ─────────────────────────────────────────────────────────────────────────────
// Helper: apply the linear (non-translating) part of a Trsf2d to a vector.
// ─────────────────────────────────────────────────────────────────────────────

#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// The uniform scale factor of a 2D transform: square root of |det(linear part)|.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

// ─────────────────────────────────────────────────────────────────────────────
// gp_Parab2d  (elementary, non-parameterised value type)
// ─────────────────────────────────────────────────────────────────────────────

/// `gp_Parab2d` — an elementary, non-parameterised 2D parabola: a local frame
/// (`gp_Ax22d`) whose origin is the vertex, plus a focal length.
///
/// The axis of symmetry runs along the "X Direction", the tangent at the vertex
/// along the "Y Direction".
#[derive(Clone, Copy, Debug)]
// occt: gp_Parab2d
pub struct Parab2d {
    position: Ax22d2,
    focal: f64,
}

impl Parab2d {
    /// `gp_Parab2d(const gp_Ax2d& MirrorAxis, Standard_Real Focal, Standard_Boolean Sense)`.
    ///
    /// The "X Direction" of the local frame is the axis of symmetry;
    /// the frame is direct if `sense` is true.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if `focal < 0`.
    pub fn new(mirror_axis: Ax2d, focal: f64, sense: bool) -> Self {
        assert!(focal >= 0.0, "gp_Parab2d: focal must be >= 0");
        Self {
            position: Ax22d2::from_x_axis(mirror_axis.location, mirror_axis.direction, sense),
            focal,
        }
    }

    /// `gp_Parab2d(const gp_Ax22d& A, Standard_Real Focal)` — parabola from a
    /// full local frame. `A`'s origin is the vertex.
    ///
    /// # Panics
    /// Raised if `focal < 0`.
    pub fn from_axis(position: Ax22d2, focal: f64) -> Self {
        assert!(focal >= 0.0, "gp_Parab2d: focal must be >= 0");
        Self { position, focal }
    }

    /// The focal length (distance from the vertex to the focus).
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// The vertex (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.position.location()
    }

    /// The local coordinate system (`gp_Ax22d`).
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.position
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2d_Parabola
// ─────────────────────────────────────────────────────────────────────────────

/// `Geom2d_Parabola` — a parameterised analytic 2D parabola.
///
/// Stores its local coordinate system (`Ax22d2`, the OCCT `gp_Ax22d pos` of
/// `Geom2d_Conic`) and its `focal` length. The parameterisation is:
///
/// ```text
/// P(U) = Vertex + (U² / (2*p)) * XDir + U * YDir
/// ```
///
/// where `p = 2 * focal` is the parabola **parameter** (semi-latus rectum).
///
/// The curve is **open** and **not periodic**: first parameter = `−INFINITE`,
/// last parameter = `+INFINITE`.
// occt: Geom2d_Parabola
#[derive(Clone, Copy, Debug)]
pub struct Geom2dParabola {
    /// Local coordinate system: origin = vertex, X = axis toward focus,
    /// Y = tangent at vertex.
    pos: Ax22d2,
    /// Focal length (distance from vertex to focus). Must be non-negative.
    focal: f64,
}

impl Geom2dParabola {
    // ──────────────────────────────────────── Constructors ───────────────────

    /// `Geom2d_Parabola(const gp_Ax2d& MirrorAxis, Standard_Real Focal,
    /// Standard_Boolean Sense = Standard_True)`.
    ///
    /// Constructs a parabola with:
    /// - vertex at the origin of `mirror_axis`,
    /// - axis of symmetry along the "X Direction" of `mirror_axis`,
    /// - direct (CCW) frame if `sense` is `true`.
    ///
    /// # Panics
    /// Raised (`Standard_ConstructionError`) if `focal < 0`.
    pub fn new(mirror_axis: Ax2d, focal: f64, sense: bool) -> Self {
        assert!(
            focal >= 0.0,
            "Geom2d_Parabola: Focal must be >= 0"
        );
        Self {
            pos: Ax22d2::from_x_axis(mirror_axis.location, mirror_axis.direction, sense),
            focal,
        }
    }

    /// `Geom2d_Parabola(const gp_Ax22d& Axis, Standard_Real Focal)` — from a
    /// full local frame. `Axis`'s origin is the vertex.
    ///
    /// # Panics
    /// Raised if `focal < 0`.
    pub fn from_axis(axis: Ax22d2, focal: f64) -> Self {
        assert!(
            focal >= 0.0,
            "Geom2d_Parabola: Focal must be >= 0"
        );
        Self { pos: axis, focal }
    }

    /// `Geom2d_Parabola(const gp_Parab2d& Prb)` — by conversion of an
    /// elementary parabola.
    pub fn from_parab2d(p: Parab2d) -> Self {
        Self {
            pos: p.position(),
            focal: p.focal(),
        }
    }

    // ──────────────────────────────────────── Setters ────────────────────────

    /// `SetFocal(Standard_Real Focal)` — change the focal length.
    ///
    /// # Panics
    /// Raised if `focal < 0`.
    pub fn set_focal(&mut self, focal: f64) {
        assert!(
            focal >= 0.0,
            "Geom2d_Parabola::SetFocal: Focal must be >= 0"
        );
        self.focal = focal;
    }

    /// `SetLocation(const gp_Pnt2d& P)` — move the vertex.
    pub fn set_location(&mut self, p: Pnt2d) {
        self.pos = Ax22d2::from_x_axis(p, self.pos.x_direction(), true);
    }

    // ──────────────────────────────────────── Getters ────────────────────────

    /// `Standard_Real Focal() const` — the focal length.
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// `Standard_Real Parameter() const` — the parabola parameter
    /// `p = 2 * Focal` (semi-latus rectum).
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// `gp_Pnt2d Focus() const` — the focus of the parabola:
    /// `Vertex + Focal * XDir`.
    pub fn focus(&self) -> Pnt2d {
        self.pos.location() + self.pos.x_direction() * self.focal
    }

    /// `Standard_Real Eccentricity() const` — for a parabola this is always
    /// `1.0` (the defining property of a parabola).
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        1.0
    }

    /// `gp_Ax2d Directrix() const` — the directrix of the parabola.
    ///
    /// The directrix is the line through `Vertex − Focal * XDir` (the point
    /// on the axis of symmetry equidistant from the vertex as the focus, but
    /// on the opposite side) perpendicular to the axis of symmetry —
    /// i.e. in the "Y Direction".
    pub fn directrix(&self) -> Ax2d {
        let loc = self.pos.location() - self.pos.x_direction() * self.focal;
        Ax2d::new(loc, self.pos.y_direction())
    }

    /// `gp_Ax22d Position() const` — the local coordinate system.
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// `gp_Pnt2d Location() const` — the vertex (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// `gp_Ax2d XAxis() const` — the axis of symmetry: vertex + X direction.
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.x_direction())
    }

    /// `gp_Ax2d YAxis() const` — the tangent axis at the vertex: vertex + Y direction.
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.y_direction())
    }

    /// The "X Direction" of the local frame (axis of symmetry direction).
    #[inline]
    pub fn x_direction(&self) -> Dir2d {
        self.pos.x_direction()
    }

    /// The "Y Direction" of the local frame (tangent at vertex direction).
    #[inline]
    pub fn y_direction(&self) -> Dir2d {
        self.pos.y_direction()
    }

    /// `gp_Parab2d Parab2d() const` — the equivalent non-persistent parabola.
    pub fn parab2d(&self) -> Parab2d {
        Parab2d::from_axis(self.pos, self.focal)
    }

    // ──────────────────────────────────────── Curve interface ────────────────

    /// `Standard_Real ReversedParameter(Standard_Real U) const`.
    ///
    /// For a parabola, the reversed parameter is `−U`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

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

    // ──────────────────────────────────────── Evaluation ─────────────────────

    /// `D0(U, P)` — the point at parameter `U`:
    ///
    /// ```text
    /// P = Vertex + (U² / (2*p)) * XDir + U * YDir
    /// ```
    ///
    /// where `p = 2 * Focal`, so `2*p = 4*Focal`.
    pub fn d0(&self, u: f64) -> Pnt2d {
        let p = 2.0 * self.focal; // parabola parameter p = 2F
        let x = u * u / (2.0 * p);
        let y = u;
        self.pos.location()
            + self.pos.x_direction() * x
            + self.pos.y_direction() * y
    }

    /// `Value(U)` — alias for [`d0`](Self::d0).
    #[inline]
    pub fn value(&self, u: f64) -> Pnt2d {
        self.d0(u)
    }

    /// `D1(U, P, V1)` — the point and first derivative.
    ///
    /// ```text
    /// P  = Vertex + (U² / (2*p)) * XDir + U * YDir
    /// V1 = (U / p) * XDir + YDir
    /// ```
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let p = 2.0 * self.focal;
        let point = self.d0(u);
        let v1 = self.pos.x_direction() * (u / p) + self.pos.y_direction();
        (point, v1)
    }

    /// `D2(U, P, V1, V2)` — the point and first two derivatives.
    ///
    /// ```text
    /// V2 = (1 / p) * XDir
    /// ```
    pub fn d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let p = 2.0 * self.focal;
        let (point, v1) = self.d1(u);
        let v2 = self.pos.x_direction() * (1.0 / p);
        (point, v1, v2)
    }

    /// `D3(U, P, V1, V2, V3)` — the point and first three derivatives.
    ///
    /// All derivatives of order ≥ 3 are zero for a parabola.
    pub fn d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let (point, v1, v2) = self.d2(u);
        (point, v1, v2, Vec2d::new(0.0, 0.0))
    }

    /// `DN(U, N)` — the `N`-th derivative vector (`N >= 1`).
    ///
    /// - `N = 1`: `(U/p) * XDir + YDir`
    /// - `N = 2`: `(1/p) * XDir`
    /// - `N >= 3`: zero vector
    ///
    /// # Panics
    /// Raised (`Standard_RangeError`) if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_Parabola::DN: N must be >= 1");
        let p = 2.0 * self.focal;
        match n {
            1 => self.pos.x_direction() * (u / p) + self.pos.y_direction(),
            2 => self.pos.x_direction() * (1.0 / p),
            _ => Vec2d::new(0.0, 0.0),
        }
    }

    // ──────────────────────────────────────── Orientation ────────────────────

    /// `Reverse()` — reverses the orientation of this parabola in place.
    ///
    /// Reversing the parameterisation direction negates the "Y Direction"
    /// (the tangent at the vertex). The shape and the "X Direction" (axis of
    /// symmetry) are unchanged. The local frame handedness changes accordingly:
    /// we rebuild from the current X direction with `sense = false` if the
    /// current frame was direct, `true` if it was indirect.
    pub fn reverse(&mut self) {
        // Negate the Y direction: rebuild the frame with flipped sense.
        // The cross product x_dir × y_dir determines the current sense.
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        // Current sense: positive cross product => direct (sense=true).
        let current_direct = xd.cross(yd) >= 0.0;
        self.pos = Ax22d2::from_x_axis(self.pos.location(), xd, !current_direct);
    }

    /// `Reversed()` — returns a copy with the orientation reversed.
    pub fn reversed(&self) -> Geom2dParabola {
        let mut c = *self;
        c.reverse();
        c
    }

    // ──────────────────────────────────────── Transform ──────────────────────

    /// `Transform(const gp_Trsf2d& T)` — applies the transformation `T` in
    /// place.
    ///
    /// The vertex (location) and directions are mapped through `T`; the focal
    /// length is scaled by `|ScaleFactor(T)|`.
    pub fn transform(&mut self, t: &Trsf2d) {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        let new_x = apply_vector_2d(t, self.pos.x_direction());
        let new_y_hint = apply_vector_2d(t, self.pos.y_direction());
        self.pos = Ax22d2::new(new_loc, new_x, new_y_hint);
        self.focal *= scale;
    }

    /// `Transformed(T)` — returns a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Geom2dParabola {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// `Copy()` — a deep, independent copy.
    #[inline]
    pub fn copy(&self) -> Geom2dParabola {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::CONFUSION;

    fn make_parabola() -> Geom2dParabola {
        Geom2dParabola::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            2.0,
            true,
        )
    }

    #[test]
    fn focal_and_parameter() {
        let p = make_parabola();
        assert!((p.focal() - 2.0).abs() < CONFUSION);
        assert!((p.parameter() - 4.0).abs() < CONFUSION);
    }

    #[test]
    fn focus_position() {
        let p = make_parabola();
        let f = p.focus();
        assert!((f.x - 2.0).abs() < CONFUSION && f.y.abs() < CONFUSION);
    }

    #[test]
    fn d0_at_vertex() {
        let p = make_parabola();
        let pt = p.d0(0.0);
        assert!(pt.x.abs() < CONFUSION && pt.y.abs() < CONFUSION);
    }

    #[test]
    fn d0_satisfies_parabola_equation() {
        let par = make_parabola(); // focal=2, p=4
        let p_param = par.parameter();
        for u in [-4.0_f64, -2.0, 0.0, 2.0, 4.0] {
            let pt = par.d0(u);
            assert!((pt.y * pt.y - 2.0 * p_param * pt.x).abs() < CONFUSION);
        }
    }

    #[test]
    fn is_not_closed() {
        let p = make_parabola();
        assert!(!p.is_closed());
        assert!(!p.is_periodic());
    }

    #[test]
    fn reverse_geometry_preserved() {
        let par = make_parabola();
        let u = 3.0;
        let pt_before = par.d0(u);
        let rev = par.reversed();
        let pt_after = rev.d0(par.reversed_parameter(u));
        assert!((pt_before.x - pt_after.x).abs() < CONFUSION);
        assert!((pt_before.y - pt_after.y).abs() < CONFUSION);
    }

    #[test]
    fn transform_translation() {
        let mut par = make_parabola();
        par.transform(&Trsf2d::translation(Pnt2d::new(5.0, 10.0)));
        assert!((par.location().x - 5.0).abs() < CONFUSION);
        assert!((par.location().y - 10.0).abs() < CONFUSION);
        assert!((par.focal() - 2.0).abs() < CONFUSION);
    }
}
