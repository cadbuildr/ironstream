//! `Geom2d_Circle` — an analytic 2D circle, faithfully reproducing
//! OpenCascade's `Geom2d_Circle` (a `Geom2d_Conic`).
//!
//! A circle is positioned in the plane by a local coordinate system
//! (`gp_Ax22d`): an origin (the circle centre) plus an "X Direction" and a
//! "Y Direction". The circle is parameterised by an angle `U`:
//!
//! ```text
//! P(U) = O + R*cos(U)*XDir + R*sin(U)*YDir
//! ```
//!
//! where `O`, `XDir`, `YDir` are the origin and the two directions of the
//! local coordinate system, and `R` is the radius. The "X Axis" defines the
//! parameter origin. The curve is closed and periodic with period `2*PI` and a
//! parameter range of `[0, 2*PI[`.
//!
//! This module is self-contained: it builds on `gp2d::{Pnt2d, Vec2d, Trsf2d}`
//! and `precision`, and introduces the local-frame (`Ax22d2`) and elementary
//! 2D circle (`Circ2d`) value types that OCCT spells `gp_Ax22d` / `gp_Circ2d`.

use crate::gp2d::{Pnt2d, Trsf2d, Vec2d};
use std::f64::consts::PI;

/// `2*PI` — the period and parameter span of a full circle.
const TWO_PI: f64 = 2.0 * PI;

/// A 2D unit direction (`gp_Dir2d`). Reuses [`Pnt2d`] as the underlying XY pair,
/// kept always normalised at construction.
pub type Dir2d = Pnt2d;

/// A 2D local coordinate system (`gp_Ax22d`): an origin plus a normalised
/// "X Direction" and "Y Direction".
///
/// The frame carries an explicit orientation (direct / indirect). When built
/// from a single X axis with `sense = true` the Y direction is the X direction
/// rotated `+90°` (a direct / counter-clockwise frame); with `sense = false`
/// it is rotated `-90°` (an indirect / clockwise frame).
#[derive(Clone, Copy, Debug)]
// occt-ref: gp_Ax22d
pub struct Ax22d2 {
    location: Pnt2d,
    x_direction: Dir2d,
    y_direction: Dir2d,
}

impl Ax22d2 {
    /// Builds a frame from an explicit origin and X/Y directions (both
    /// normalised). Mirrors `gp_Ax22d(P, Vx, Vy)`: the Y direction is forced
    /// to lie on the side of `vy` while staying orthogonal to the X direction,
    /// so the stored frame is orthonormal.
    pub fn new(location: Pnt2d, vx: Dir2d, vy: Dir2d) -> Self {
        let x_direction = vx.normalized();
        // Orientation (sense) is taken from the sign of the cross product of
        // the supplied X and Y directions; the stored Y direction is X rotated
        // by +/-90 so the frame is exactly orthonormal, as gp_Ax22d does.
        let sense = x_direction.cross(vy.normalized()) >= 0.0;
        let y_direction = if sense {
            x_direction.perp() // +90deg
        } else {
            -x_direction.perp() // -90deg
        };
        Self {
            location,
            x_direction,
            y_direction,
        }
    }

    /// Builds a frame from a single X axis (origin + X direction). Mirrors
    /// `gp_Ax22d(gp_Ax2d, Sense)`: with `sense = true` the frame is direct
    /// (Y = X rotated +90°), otherwise indirect (Y = X rotated -90°).
    pub fn from_x_axis(location: Pnt2d, vx: Dir2d, sense: bool) -> Self {
        let x_direction = vx.normalized();
        let y_direction = if sense {
            x_direction.perp()
        } else {
            -x_direction.perp()
        };
        Self {
            location,
            x_direction,
            y_direction,
        }
    }

    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.location
    }
    #[inline]
    pub fn x_direction(&self) -> Dir2d {
        self.x_direction
    }
    #[inline]
    pub fn y_direction(&self) -> Dir2d {
        self.y_direction
    }
}

/// An elementary, non-parameterised 2D circle (`gp_Circ2d`): a local frame
/// (`gp_Ax22d`) whose origin is the centre, plus a radius.
#[derive(Clone, Copy, Debug)]
// occt-ref: gp_Circ2d
pub struct Circ2d {
    position: Ax22d2,
    radius: f64,
}

impl Circ2d {
    /// `gp_Circ2d(gp_Ax2d, Radius, Sense)` — circle from an X axis and radius.
    pub fn new(x_axis_loc: Pnt2d, x_axis_dir: Dir2d, radius: f64, sense: bool) -> Self {
        assert!(radius >= 0.0, "gp_Circ2d: radius must be non-negative");
        Self {
            position: Ax22d2::from_x_axis(x_axis_loc, x_axis_dir, sense),
            radius,
        }
    }

    /// `gp_Circ2d(gp_Ax22d, Radius)` — circle from a full local frame.
    pub fn from_axis(position: Ax22d2, radius: f64) -> Self {
        assert!(radius >= 0.0, "gp_Circ2d: radius must be non-negative");
        Self { position, radius }
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.position.location()
    }
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.position
    }
}

/// `Geom2d_Circle` — a parameterised analytic 2D circle.
///
/// Stores its local coordinate system (`Ax22d2`, the OCCT `gp_Ax22d pos` of
/// `Geom2d_Conic`) and its radius. The parametrisation is
/// `P(U) = O + R*cos(U)*XDir + R*sin(U)*YDir`.
// occt-ref: Geom2d_Circle
#[derive(Clone, Copy, Debug)]
pub struct Geom2dCircle {
    pos: Ax22d2,
    radius: f64,
}

impl Geom2dCircle {
    /// `Geom2d_Circle(const gp_Circ2d& C)` — by conversion of an elementary
    /// circle.
    pub fn from_circ2d(c: Circ2d) -> Self {
        Self {
            pos: c.position(),
            radius: c.radius(),
        }
    }

    /// `Geom2d_Circle(const gp_Ax2d& A, Radius, Sense)` — `A` is the "X Axis"
    /// of the local frame; the frame is direct if `sense` is true.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if `radius` is
    /// negative.
    pub fn from_x_axis(loc: Pnt2d, x_dir: Dir2d, radius: f64, sense: bool) -> Self {
        assert!(radius >= 0.0, "Geom2d_Circle: radius must be non-negative");
        Self {
            pos: Ax22d2::from_x_axis(loc, x_dir, sense),
            radius,
        }
    }

    /// `Geom2d_Circle(const gp_Ax22d& A, Radius)` — `A` locates and orients the
    /// circle; its origin is the centre.
    ///
    /// Panics if `radius` is negative.
    pub fn from_axis(a: Ax22d2, radius: f64) -> Self {
        assert!(radius >= 0.0, "Geom2d_Circle: radius must be non-negative");
        Self { pos: a, radius }
    }

    /// `SetCirc2d(const gp_Circ2d& C)` — convert an elementary circle into this
    /// one.
    pub fn set_circ2d(&mut self, c: Circ2d) {
        self.pos = c.position();
        self.radius = c.radius();
    }

    /// `SetRadius(const Standard_Real R)`.
    ///
    /// Panics if `r` is negative (OCCT raises `Standard_ConstructionError`).
    pub fn set_radius(&mut self, r: f64) {
        assert!(
            r >= 0.0,
            "Geom2d_Circle::SetRadius: radius must be non-negative"
        );
        self.radius = r;
    }

    /// `Circ2d()` — the equivalent non-persistent `gp_Circ2d`.
    pub fn circ2d(&self) -> Circ2d {
        Circ2d::from_axis(self.pos, self.radius)
    }

    /// `Radius()`.
    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// `Location()` — the centre of the circle (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// The local coordinate system (`gp_Ax22d Position()`).
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// The "X Direction" of the local frame.
    #[inline]
    pub fn x_direction(&self) -> Dir2d {
        self.pos.x_direction()
    }

    /// The "Y Direction" of the local frame.
    #[inline]
    pub fn y_direction(&self) -> Dir2d {
        self.pos.y_direction()
    }

    /// `ReversedParameter(U)` — for a circle, `2*PI - U`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        TWO_PI - u
    }

    /// `Eccentricity()` — `0.0` for any circle.
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        0.0
    }

    /// `FirstParameter()` — `0.0`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// `LastParameter()` — `2*PI`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        TWO_PI
    }

    /// `IsClosed()` — always true.
    #[inline]
    pub fn is_closed(&self) -> bool {
        true
    }

    /// `IsPeriodic()` — always true; the period is `2*PI`.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        true
    }

    /// The period, `2*PI`.
    #[inline]
    pub fn period(&self) -> f64 {
        TWO_PI
    }

    /// `EvalD0(U)` — the point of parameter `U`:
    /// `P = C + R*cos(U)*XDir + R*sin(U)*YDir`.
    pub fn eval_d0(&self, u: f64) -> Pnt2d {
        let (s, c) = u.sin_cos();
        self.pos.location()
            + self.pos.x_direction() * (self.radius * c)
            + self.pos.y_direction() * (self.radius * s)
    }

    /// `EvalD1(U)` — point and first derivative.
    ///
    /// `V1 = R*(-sin(U)*XDir + cos(U)*YDir)`.
    pub fn eval_d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let (s, c) = u.sin_cos();
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let o = self.pos.location();
        let p = o + xd * (self.radius * c) + yd * (self.radius * s);
        let v1 = xd * (-self.radius * s) + yd * (self.radius * c);
        (p, v1)
    }

    /// `EvalD2(U)` — point, first and second derivatives.
    ///
    /// `V2 = R*(-cos(U)*XDir - sin(U)*YDir) = -(P - C)`.
    pub fn eval_d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let (s, c) = u.sin_cos();
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let o = self.pos.location();
        let p = o + xd * (self.radius * c) + yd * (self.radius * s);
        let v1 = xd * (-self.radius * s) + yd * (self.radius * c);
        let v2 = xd * (-self.radius * c) + yd * (-self.radius * s);
        (p, v1, v2)
    }

    /// `EvalD3(U)` — point, first, second and third derivatives.
    ///
    /// `V3 = R*(sin(U)*XDir - cos(U)*YDir)`.
    pub fn eval_d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let (s, c) = u.sin_cos();
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let o = self.pos.location();
        let p = o + xd * (self.radius * c) + yd * (self.radius * s);
        let v1 = xd * (-self.radius * s) + yd * (self.radius * c);
        let v2 = xd * (-self.radius * c) + yd * (-self.radius * s);
        let v3 = xd * (self.radius * s) + yd * (-self.radius * c);
        (p, v1, v2, v3)
    }

    /// `EvalDN(U, N)` — the `N`-th derivative vector (`N >= 1`).
    ///
    /// Differentiating `R*cos(U)*XDir + R*sin(U)*YDir` `N` times rotates each
    /// trigonometric argument by `N*PI/2`:
    /// `DN = R*cos(U + N*PI/2)*XDir + R*sin(U + N*PI/2)*YDir`.
    ///
    /// Panics if `n < 1` (OCCT raises `Standard_RangeError`).
    pub fn eval_dn(&self, u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_Circle::EvalDN: N must be >= 1");
        let arg = u + (n as f64) * (PI / 2.0);
        let (s, c) = arg.sin_cos();
        self.pos.x_direction() * (self.radius * c) + self.pos.y_direction() * (self.radius * s)
    }

    /// `Transform(const gp_Trsf2d& T)` — applies a 2D transform.
    ///
    /// The centre and directions are mapped through `T`; the radius is scaled
    /// by `Abs(T.ScaleFactor())`. The local frame is re-orthonormalised, just
    /// as transforming a `gp_Ax22d` does.
    pub fn transform(&mut self, t: &Trsf2d) {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        // Transform the X direction as a vector (no translation), then rebuild
        // the orthonormal frame preserving the original handedness.
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let new_x = apply_vector_2d(t, xd);
        let new_y_hint = apply_vector_2d(t, yd);
        self.pos = Ax22d2::new(new_loc, new_x, new_y_hint);
        self.radius *= scale.abs();
    }

    /// `Transformed(T)` — non-mutating variant returning a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Geom2dCircle {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// `Copy()` — a deep, independent copy of this circle.
    pub fn copy(&self) -> Geom2dCircle {
        *self
    }
}

/// Applies the linear part of a 2D transform to a vector (ignoring the
/// translation), as `gp_Trsf2d` does for a `gp_Vec2d`.
#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// The uniform scale factor of a 2D transform (`gp_Trsf2d::ScaleFactor()`):
/// the square root of the absolute value of the 2x2 linear determinant.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_zero_is_on_x_axis() {
        let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 5.0, true);
        let p = c.eval_d0(0.0);
        assert!((p.x - 5.0).abs() < 1e-12 && p.y.abs() < 1e-12, "{p:?}");
    }

    #[test]
    fn periodicity_and_bounds() {
        let c = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 2.0, true);
        assert_eq!(c.first_parameter(), 0.0);
        assert!((c.last_parameter() - TWO_PI).abs() < 1e-15);
        assert!(c.is_closed() && c.is_periodic());
    }
}
