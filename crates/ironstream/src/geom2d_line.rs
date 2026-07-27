//! `Geom2d_Line` — an infinite, parameterized line in the plane (2D space),
//! a faithful Rust port of OpenCascade's `Geom2d_Line` (TKG2d).
//!
//! A line is defined and positioned in the plane with an axis (`gp_Ax2d`) which
//! gives it an origin and a unit vector. It is parameterized as:
//!
//! ```text
//! P(U) = O + U * Dir
//! ```
//!
//! where `O` is the origin (`Location`) and `Dir` the unit vector of its
//! positioning axis. The parameter range is `]-infinite, +infinite[`. The
//! orientation of the line is given by the unit vector of its positioning axis.
//!
//! This module is self-contained and builds only on the existing public API
//! (`gp2d::{Pnt2d, Vec2d, Ax2d, Trsf2d}` and `precision`). To avoid clashing
//! with the in-tree `gp_prim::Lin` (which is 3D), the local 2D line value type
//! is named [`Lin2d`] (OCCT's `gp_Lin2d`).

use crate::gp2d::{Ax2d, Pnt2d, Trsf2d, Vec2d};

/// Global continuity classification, mirroring OCCT's `GeomAbs_Shape`. A line
/// is infinitely differentiable, so its continuity is [`GeomAbsShape::CN`].
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

/// `gp_Lin2d` — a non-parameterized infinite 2D line: a location plus a unit
/// direction. Provided locally because the in-tree `gp_prim::Lin` is 3D.
#[derive(Clone, Copy, Debug)]
// occt-ref: gp_Lin2d
pub struct Lin2d {
    loc: Pnt2d,
    dir: Pnt2d,
}

impl Lin2d {
    /// Constructs a 2D line through point `p` parallel to the unit vector `v`.
    pub fn new(p: Pnt2d, v: Pnt2d) -> Self {
        Self {
            loc: p,
            dir: v.normalized(),
        }
    }

    /// Constructs a 2D line from a positioning axis (`gp_Ax2d`).
    pub fn from_axis(a: Ax2d) -> Self {
        Self {
            loc: a.location,
            dir: a.direction.normalized(),
        }
    }

    /// The location point (origin) of the line.
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.loc
    }

    /// The unit direction of the line.
    #[inline]
    pub fn direction(&self) -> Pnt2d {
        self.dir
    }

    /// The positioning axis (`gp_Ax2d`) of the line.
    #[inline]
    pub fn position(&self) -> Ax2d {
        Ax2d {
            location: self.loc,
            direction: self.dir,
        }
    }

    /// Computes the distance between the line and the point `p`.
    pub fn distance(&self, p: Pnt2d) -> f64 {
        // |(P - O) x Dir|, with Dir a unit vector.
        let v = p - self.loc;
        v.cross(self.dir).abs()
    }
}

/// Describes an infinite line in the plane (2D space).
///
/// Faithful port of OCCT's `Geom2d_Line`. The line is positioned with a 2D axis
/// (`gp_Ax2d`): the location is the origin, and the unit direction gives the
/// orientation and parameterization `P(U) = O + U*Dir`.
#[derive(Clone, Copy, Debug)]
// occt-ref: Geom2d_Line
pub struct Geom2dLine {
    pos: Ax2d,
}

/// Result of [`Geom2dLine::eval_d1`]: point and first derivative.
#[derive(Clone, Copy, Debug)]
pub struct ResD1 {
    pub p: Pnt2d,
    pub v1: Vec2d,
}

/// Result of [`Geom2dLine::eval_d2`]: point, first and second derivatives.
#[derive(Clone, Copy, Debug)]
pub struct ResD2 {
    pub p: Pnt2d,
    pub v1: Vec2d,
    pub v2: Vec2d,
}

/// Result of [`Geom2dLine::eval_d3`]: point and first three derivatives.
#[derive(Clone, Copy, Debug)]
pub struct ResD3 {
    pub p: Pnt2d,
    pub v1: Vec2d,
    pub v2: Vec2d,
    pub v3: Vec2d,
}

impl Geom2dLine {
    /// Creates a line located in 2D space with the axis placement `a`. The
    /// location of `a` is the origin of the line. (`Geom2d_Line(const gp_Ax2d&)`)
    pub fn new(a: Ax2d) -> Self {
        Self {
            pos: Ax2d {
                location: a.location,
                direction: a.direction.normalized(),
            },
        }
    }

    /// Creates a line by conversion of the `gp_Lin2d` line `l`.
    /// (`Geom2d_Line(const gp_Lin2d&)`)
    pub fn from_lin2d(l: Lin2d) -> Self {
        Self { pos: l.position() }
    }

    /// Constructs a line passing through point `p` and parallel to vector `v`
    /// (`p` and `v` are, respectively, the origin and the unit vector of the
    /// positioning axis of the line). (`Geom2d_Line(const gp_Pnt2d&, const gp_Dir2d&)`)
    pub fn from_point_dir(p: Pnt2d, v: Pnt2d) -> Self {
        Self {
            pos: Ax2d {
                location: p,
                direction: v.normalized(),
            },
        }
    }

    /// Sets `self` so that it has the same geometric properties as `l`.
    pub fn set_lin2d(&mut self, l: Lin2d) {
        self.pos = l.position();
    }

    /// Changes the direction of the line.
    pub fn set_direction(&mut self, v: Pnt2d) {
        self.pos.direction = v.normalized();
    }

    /// Returns the direction of the line (a unit vector).
    #[inline]
    pub fn direction(&self) -> Pnt2d {
        self.pos.direction
    }

    /// Changes the "Location" point (origin) of the line.
    pub fn set_location(&mut self, p: Pnt2d) {
        self.pos.location = p;
    }

    /// Returns the "Location" point (origin) of the line.
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location
    }

    /// Changes the "Location" and the "Direction" of the line.
    pub fn set_position(&mut self, a: Ax2d) {
        self.pos = Ax2d {
            location: a.location,
            direction: a.direction.normalized(),
        };
    }

    /// Returns the positioning axis (`gp_Ax2d`) of the line.
    #[inline]
    pub fn position(&self) -> Ax2d {
        self.pos
    }

    /// Returns a non-persistent `gp_Lin2d` line with the same geometric
    /// properties as `self`.
    pub fn lin2d(&self) -> Lin2d {
        Lin2d::from_axis(self.pos)
    }

    /// Changes the orientation of this line: the unit vector of the positioning
    /// axis is reversed.
    pub fn reverse(&mut self) {
        self.pos.direction = -self.pos.direction;
    }

    /// Returns a copy of this line with reversed orientation.
    pub fn reversed(&self) -> Self {
        let mut c = *self;
        c.reverse();
        c
    }

    /// Computes the parameter on the reversed line for the point of parameter
    /// `u` on this line. For a line, the returned value is `-u`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

    /// Returns `RealFirst` from `Standard` (the smallest representable double).
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        // OCCT returns Standard_RealFirst == -DBL_MAX (≈ -1.7977e308).
        f64::MIN
    }

    /// Returns `RealLast` from `Standard` (the largest representable double).
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        // OCCT returns Standard_RealLast == DBL_MAX (≈ 1.7977e308).
        f64::MAX
    }

    /// Returns `false` — a line is never closed.
    #[inline]
    pub fn is_closed(&self) -> bool {
        false
    }

    /// Returns `false` — a line is never periodic.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// Returns `GeomAbs_CN`, the global continuity of any line.
    #[inline]
    pub fn continuity(&self) -> GeomAbsShape {
        GeomAbsShape::CN
    }

    /// Returns `true` for any order `n` — a line is `CN`.
    #[inline]
    pub fn is_cn(&self, _n: i32) -> bool {
        true
    }

    /// Computes the distance between this line and the point `p`.
    pub fn distance(&self, p: Pnt2d) -> f64 {
        self.lin2d().distance(p)
    }

    /// Returns the point of parameter `u`: `P(U) = O + U * Dir`.
    pub fn eval_d0(&self, u: f64) -> Pnt2d {
        self.pos.location + self.pos.direction * u
    }

    /// Returns the point `p` of parameter `u` and the first derivative `v1`.
    pub fn eval_d1(&self, u: f64) -> ResD1 {
        ResD1 {
            p: self.eval_d0(u),
            v1: self.pos.direction,
        }
    }

    /// Returns the point `p` of parameter `u`, the first and second
    /// derivatives. `v2` has null magnitude for a line.
    pub fn eval_d2(&self, u: f64) -> ResD2 {
        ResD2 {
            p: self.eval_d0(u),
            v1: self.pos.direction,
            v2: Vec2d::new(0.0, 0.0),
        }
    }

    /// Returns the point `p` of parameter `u` and the first three derivatives.
    /// `v2` and `v3` have null magnitude for a line.
    pub fn eval_d3(&self, u: f64) -> ResD3 {
        ResD3 {
            p: self.eval_d0(u),
            v1: self.pos.direction,
            v2: Vec2d::new(0.0, 0.0),
            v3: Vec2d::new(0.0, 0.0),
        }
    }

    /// For the point of parameter `u`, computes the vector corresponding to the
    /// `n`-th derivative. For `n == 1` this is `Dir`; for `n >= 2` it is the
    /// null vector.
    ///
    /// # Panics
    /// Panics if `n < 1` (mirrors OCCT's `Standard_RangeError`).
    pub fn eval_dn(&self, _u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_Line::EvalDN: N must be >= 1");
        if n == 1 {
            self.pos.direction
        } else {
            Vec2d::new(0.0, 0.0)
        }
    }

    /// Applies the transformation `t` to this line.
    pub fn transform(&mut self, t: &Trsf2d) {
        let new_loc = t.apply(self.pos.location);
        // Transform the direction as a free vector (no translation), then
        // re-normalize so the positioning axis keeps a unit direction.
        let d = self.pos.direction;
        let td = Pnt2d::new(
            t.m[0][0] * d.x + t.m[0][1] * d.y,
            t.m[1][0] * d.x + t.m[1][1] * d.y,
        );
        self.pos = Ax2d {
            location: new_loc,
            direction: td.normalized(),
        };
    }

    /// Returns a copy of this line transformed by `t`.
    pub fn transformed(&self, t: &Trsf2d) -> Self {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// Computes the parameter on the line transformed by `t` for the point of
    /// parameter `u` on this line. For a line, the returned value is `u`
    /// multiplied by the scale factor of `t`.
    pub fn transformed_parameter(&self, u: f64, t: &Trsf2d) -> f64 {
        u * self.parametric_transformation(t)
    }

    /// Returns the coefficient required to compute the parametric
    /// transformation of this line when `t` is applied. For a line, this is the
    /// scale factor of `t` — recovered as the magnitude of the transformed unit
    /// direction (OCCT's `gp_Trsf2d::ScaleFactor`).
    pub fn parametric_transformation(&self, t: &Trsf2d) -> f64 {
        let d = self.pos.direction;
        let td = Pnt2d::new(
            t.m[0][0] * d.x + t.m[0][1] * d.y,
            t.m[1][0] * d.x + t.m[1][1] * d.y,
        );
        td.norm()
    }

    /// Creates a new object which is a copy of this line.
    pub fn copy(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_and_distance() {
        let l = Geom2dLine::from_point_dir(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
        let p = l.eval_d0(5.0);
        assert!((p.x - 5.0).abs() < 1e-12 && p.y.abs() < 1e-12);
        assert!((l.distance(Pnt2d::new(5.0, 3.0)) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn reverse_negates_param() {
        let l = Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        assert_eq!(l.reversed_parameter(5.0), -5.0);
    }
}
