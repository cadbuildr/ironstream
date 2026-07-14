//! `Geom_Line` — an infinite, parameterized 3D line, mirroring OpenCascade's
//! `Geom_Line` (`src/ModelingData/TKG3d/Geom/Geom_Line.hxx`).
//!
//! A line is defined and positioned in space by a positioning axis
//! (`gp_Ax1` — here [`Ax1`]) which gives it an origin and a unit direction.
//! It is parameterized as `P(U) = O + U * Dir`, where `O` is the origin and
//! `Dir` the unit vector of its positioning axis. The parameter range is
//! `]-infinite, +infinite[`, and the orientation is given by `Dir`.
//!
//! This is the `Geom`-level descriptor (a transient curve with mutable state
//! and a positioning axis), distinct from the immutable `gp_Lin`
//! ([`crate::gp_prim::Lin`]) it can be built from / converted to.
//!
//! The zero-dependency array-based [`GeomLine3d`] type is also provided here
//! for use in contexts that do not depend on the internal `gp` types.

use crate::gp::{Ax1, Pnt, Trsf};
use crate::gp_prim::Lin;
use crate::precision::INFINITE;

/// Global continuity class of a curve, mirroring OCCT's `GeomAbs_Shape`.
/// A line is `CN` (infinitely continuously differentiable).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// occt-ref: GeomAbs_Shape
pub enum GeomAbsShape {
    C0,
    G1,
    C1,
    G2,
    C2,
    C3,
    CN,
}

/// `Geom_Line` — an infinite line described by a positioning axis `pos`
/// (`gp_Ax1`): its `location` is the origin, its (unit) `direction` the line
/// direction. Parameterized `P(U) = O + U * Dir`.
// occt-ref: Geom_Line
#[derive(Clone, Copy, Debug)]
pub struct GeomLine {
    /// The positioning axis: origin (`Location`) plus unit direction.
    pos: Ax1,
}

impl GeomLine {
    // ----------------------------- constructors ----------------------------

    /// Creates a line located in 3D space with the axis placement `a1`.
    /// The `Location` of `a1` is the origin of the line.
    pub fn new(a1: Ax1) -> Self {
        Self {
            pos: Ax1::new(a1.location, a1.direction),
        }
    }

    /// Creates a line from a non-transient `gp_Lin` line.
    pub fn from_lin(l: Lin) -> Self {
        Self {
            pos: Ax1::new(l.location(), l.direction()),
        }
    }

    /// Constructs a line passing through point `p` and parallel to vector `v`
    /// (`p` and `v` are respectively the origin and the unit vector of the
    /// positioning axis of the line).
    pub fn from_point_dir(p: Pnt, v: Pnt) -> Self {
        Self {
            pos: Ax1::new(p, v),
        }
    }

    // ------------------------------- setters -------------------------------

    /// Sets `self` so that it has the same geometric properties as `l`.
    pub fn set_lin(&mut self, l: Lin) {
        self.pos = Ax1::new(l.location(), l.direction());
    }

    /// Changes the direction of the line.
    pub fn set_direction(&mut self, v: Pnt) {
        self.pos = Ax1::new(self.pos.location, v);
    }

    /// Changes the `Location` point (origin) of the line.
    pub fn set_location(&mut self, p: Pnt) {
        self.pos = Ax1::new(p, self.pos.direction);
    }

    /// Changes the `Location` and the `Direction` of `self`.
    pub fn set_position(&mut self, a1: Ax1) {
        self.pos = Ax1::new(a1.location, a1.direction);
    }

    // ------------------------------- getters -------------------------------

    /// Returns the non-transient `gp_Lin` with the same geometric properties.
    pub fn lin(&self) -> Lin {
        Lin::new(self.pos)
    }

    /// Returns the positioning axis of this line; this is also its local
    /// coordinate system.
    pub fn position(&self) -> Ax1 {
        self.pos
    }

    // ----------------------------- orientation -----------------------------

    /// Changes the orientation of this line. As a result, the unit vector of
    /// the positioning axis of this line is reversed.
    pub fn reverse(&mut self) {
        self.pos = Ax1::new(self.pos.location, -self.pos.direction);
    }

    /// Returns a copy of this line with the direction reversed (`Geom_Curve`'s
    /// `Reversed`).
    pub fn reversed(&self) -> GeomLine {
        let mut c = *self;
        c.reverse();
        c
    }

    /// Computes the parameter on the reversed line for the point of parameter
    /// `u` on this line. For a line the returned value is `-u`.
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

    // ------------------------------ parameters -----------------------------

    /// First parameter of this line: `-Precision::Infinite()`.
    pub fn first_parameter(&self) -> f64 {
        -INFINITE
    }

    /// Last parameter of this line: `Precision::Infinite()`.
    pub fn last_parameter(&self) -> f64 {
        INFINITE
    }

    /// Returns `false`: a line is never closed.
    pub fn is_closed(&self) -> bool {
        false
    }

    /// Returns `false`: a line is never periodic.
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// Returns `GeomAbs_CN`, the global continuity of any line.
    pub fn continuity(&self) -> GeomAbsShape {
        GeomAbsShape::CN
    }

    /// Returns `true` for any `n >= 0` (a line is `CN`).
    ///
    /// # Panics
    /// Panics if `n < 0`, mirroring OCCT's `Standard_RangeError`.
    pub fn is_cn(&self, n: i32) -> bool {
        assert!(n >= 0, "Geom_Line::IsCN: N must be >= 0");
        true
    }

    // ------------------------------ evaluation -----------------------------

    /// `D0` — the point of parameter `u`: `P(U) = O + U * Dir`.
    pub fn d0(&self, u: f64) -> Pnt {
        self.pos.location + self.pos.direction * u
    }

    /// `D1` — the point of parameter `u` and the first derivative (the unit
    /// direction, constant for a line).
    pub fn d1(&self, u: f64) -> (Pnt, Pnt) {
        (self.d0(u), self.pos.direction)
    }

    /// `D2` — the point and the first and second derivatives. The second
    /// derivative is the null vector for a line.
    pub fn d2(&self, u: f64) -> (Pnt, Pnt, Pnt) {
        (self.d0(u), self.pos.direction, Pnt::origin())
    }

    /// `D3` — the point and the first, second and third derivatives. The
    /// second and third derivatives are null vectors for a line.
    pub fn d3(&self, u: f64) -> (Pnt, Pnt, Pnt, Pnt) {
        (self.d0(u), self.pos.direction, Pnt::origin(), Pnt::origin())
    }

    /// `DN` — the derivative of order `n`. For `n == 1` this is the unit
    /// direction; for `n >= 2` it is the null vector.
    ///
    /// # Panics
    /// Panics if `n < 1`, mirroring OCCT's `Standard_RangeError`.
    pub fn dn(&self, _u: f64, n: i32) -> Pnt {
        assert!(n >= 1, "Geom_Line::DN: N must be >= 1");
        if n == 1 {
            self.pos.direction
        } else {
            Pnt::origin()
        }
    }

    // ------------------------------ transforms -----------------------------

    /// Applies the transformation `t` to this line.
    pub fn transform(&mut self, t: &Trsf) {
        let loc = t.apply_point(self.pos.location);
        let dir = t.apply_vector(self.pos.direction);
        self.pos = Ax1::new(loc, dir);
    }

    /// Returns a transformed copy of this line.
    pub fn transformed(&self, t: &Trsf) -> GeomLine {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// Returns the parameter on the transformed curve for the transform of the
    /// point of parameter `u`: `u * T.ScaleFactor()`.
    pub fn transformed_parameter(&self, u: f64, t: &Trsf) -> f64 {
        u * t.scale_factor()
    }

    /// Returns the coefficient to compute the parameter on the transformed
    /// curve: `T.ScaleFactor()`.
    pub fn parametric_transformation(&self, t: &Trsf) -> f64 {
        t.scale_factor()
    }

    /// Creates a new object which is a copy of this line.
    pub fn copy(&self) -> GeomLine {
        *self
    }
}

// =============================================================================
// Zero-dependency array-based stub — mirrors Geom_Line using plain [f64; 3]
// arrays so that callers with no dependency on the internal `gp` types can
// work with 3D infinite lines.
// =============================================================================

/// An infinite 3D line defined by an origin point and a unit direction vector,
/// parameterized as `P(t) = origin + t * direction`.
///
/// All coordinates are stored as plain `[f64; 3]` arrays; no external crates
/// are required.
// occt-ref: Geom_Line
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomLine3d {
    /// The origin (a point on the line).
    pub origin: [f64; 3],
    /// The unit direction vector of the line.
    pub direction: [f64; 3],
}

// ---------------------------------------------------------------------------
// Internal helpers — kept private; use only std.
// ---------------------------------------------------------------------------

#[inline]
fn norm3(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[inline]
fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let n = norm3(v);
    if n == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / n, v[1] / n, v[2] / n]
    }
}

#[inline]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

// ---------------------------------------------------------------------------

impl GeomLine3d {
    /// Creates a new `GeomLine3d` from an origin point and a direction vector.
    ///
    /// The direction is normalized internally so that all parametric
    /// evaluations follow the standard `P(t) = origin + t * unit_direction`
    /// convention.
    pub fn new(origin: [f64; 3], dir: [f64; 3]) -> Self {
        Self {
            origin,
            direction: normalize3(dir),
        }
    }

    /// Returns the point on the line at parameter `t`:
    /// `P(t) = origin + t * direction`.
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        [
            self.origin[0] + t * self.direction[0],
            self.origin[1] + t * self.direction[1],
            self.origin[2] + t * self.direction[2],
        ]
    }

    /// First derivative with respect to `t` — the (constant) unit direction
    /// vector of the line.  The parameter `_t` is accepted for API symmetry
    /// but is unused.
    pub fn d1(&self, _t: f64) -> [f64; 3] {
        self.direction
    }

    /// Second derivative with respect to `t` — always the zero vector for a
    /// line (constant first derivative).  The parameter `_t` is accepted for
    /// API symmetry but is unused.
    pub fn d2(&self, _t: f64) -> [f64; 3] {
        [0.0, 0.0, 0.0]
    }

    /// Returns the first parameter of this curve.
    ///
    /// A line is infinite in the negative direction, so this is
    /// `f64::NEG_INFINITY`, mirroring `Geom_Line::FirstParameter`.
    pub fn first_parameter() -> f64 {
        f64::NEG_INFINITY
    }

    /// Returns the last parameter of this curve.
    ///
    /// A line is infinite in the positive direction, so this is
    /// `f64::INFINITY`, mirroring `Geom_Line::LastParameter`.
    pub fn last_parameter() -> f64 {
        f64::INFINITY
    }

    /// Projects the point `pt` onto the line and returns the corresponding
    /// parameter `t` such that `point_at(t)` is the closest point on the line
    /// to `pt`.
    ///
    /// Formula: `t = dot(pt - origin, direction)`.
    pub fn project(&self, pt: [f64; 3]) -> f64 {
        dot3(sub3(pt, self.origin), self.direction)
    }

    /// Returns the Euclidean distance from the point `pt` to the nearest point
    /// on the line.
    pub fn distance_to(&self, pt: [f64; 3]) -> f64 {
        // distance = ||(pt - origin) x direction||
        let v = sub3(pt, self.origin);
        norm3(cross3(v, self.direction))
    }

    /// Reverses the orientation of the line by negating its direction vector.
    ///
    /// After reversal the parametrization satisfies `reversed_param(t) == -t`.
    pub fn reverse(&mut self) {
        self.direction = [
            -self.direction[0],
            -self.direction[1],
            -self.direction[2],
        ];
    }

    /// Returns the parameter value on the reversed line that corresponds to
    /// parameter `t` on the original line.
    ///
    /// For a line `reversed_param(t) == -t`.
    pub fn reversed_param(&self, t: f64) -> f64 {
        -t
    }
}

// ---------------------------------------------------------------------------
// Free-function constructor
// ---------------------------------------------------------------------------

/// Constructs a [`GeomLine3d`] passing through two distinct points `p1` and
/// `p2`.  The origin of the resulting line is `p1`; the direction is the unit
/// vector from `p1` to `p2`.
///
/// # Panics
/// Does not panic — if `p1 == p2` the stored direction will be the zero
/// vector, which is the only sensible degenerate result.
pub fn line_from_2pts(p1: [f64; 3], p2: [f64; 3]) -> GeomLine3d {
    GeomLine3d::new(p1, sub3(p2, p1))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at_origin() {
        let l = GeomLine3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(l.point_at(0.0), [0.0, 0.0, 0.0]);
        assert_eq!(l.point_at(3.0), [3.0, 0.0, 0.0]);
        assert_eq!(l.point_at(-2.0), [-2.0, 0.0, 0.0]);
    }

    #[test]
    fn test_d1_is_unit_direction() {
        let l = GeomLine3d::new([1.0, 2.0, 3.0], [3.0, 4.0, 0.0]);
        let d = l.d1(99.0);
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_d2_is_zero() {
        let l = GeomLine3d::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(l.d2(5.0), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_parameter_bounds() {
        assert_eq!(GeomLine3d::first_parameter(), f64::NEG_INFINITY);
        assert_eq!(GeomLine3d::last_parameter(), f64::INFINITY);
    }

    #[test]
    fn test_project() {
        let l = GeomLine3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!((l.project([5.0, 3.0, 0.0]) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_distance_to() {
        let l = GeomLine3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        // Point [0, 3, 4] is distance 5 from the x-axis.
        assert!((l.distance_to([0.0, 3.0, 4.0]) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_reverse_and_reversed_param() {
        let mut l = GeomLine3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let t = 7.0_f64;
        assert_eq!(l.reversed_param(t), -t);
        l.reverse();
        assert_eq!(l.direction, [-1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_line_from_2pts() {
        let l = line_from_2pts([1.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
        assert_eq!(l.origin, [1.0, 0.0, 0.0]);
        assert_eq!(l.direction, [1.0, 0.0, 0.0]);
    }
}
