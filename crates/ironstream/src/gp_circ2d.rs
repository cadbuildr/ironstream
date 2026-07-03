// FILE: gp_circ2d.rs
//! `gp_Circ2d` — analytic 2D circle primitive, mirroring OpenCascade's
//! `gp_Circ2d` (package `TKMath/gp`).
//!
//! A 2D circle is positioned in the plane by a local coordinate system
//! (`gp_Ax22d`): an origin (the circle centre) plus an "X Direction" and a
//! "Y Direction". The normal to the circle plane is the Z direction (always
//! `+Z` for a 2D circle).
//!
//! # Invariant
//! `radius >= 0`  (mirrors `Standard_ConstructionError` if violated).
//!
//! # Geometry
//! ```text
//! Area       = PI * R²
//! Length     = 2 * PI * R
//! Point(U)   = Centre + R * cos(U) * XDir + R * sin(U) * YDir
//! ```

use crate::geom2d_circle::{Ax22d2, Dir2d};
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d};

use std::f64::consts::PI;

// occt: gp_Circ2d
/// An analytic 2D circle: a local coordinate system ([`Ax22d2`]) whose origin
/// is the **centre**, plus a non-negative **radius**.
///
/// Mirrors the full OCCT `gp_Circ2d` API including geometric queries, derived
/// quantities and all rigid-body transformations.
#[derive(Clone, Copy, Debug)]
pub struct Circ2d {
    /// Local coordinate system — OCCT calls this `pos` (a `gp_Ax22d`).
    pos: Ax22d2,
    /// The radius of the circle. Always `>= 0`.
    radius: f64,
}

impl Circ2d {
    // ---------------------------------------------------------------- constructors

    /// `gp_Circ2d(const gp_Ax2d& XAxis, Standard_Real Radius,
    ///             Standard_Boolean Sense = Standard_True)`
    ///
    /// Constructs a circle from the "X Axis" (which provides the centre
    /// location and the "X Direction") and a radius. The frame is **direct**
    /// (counter-clockwise) when `sense` is `true`, otherwise **indirect**
    /// (clockwise).
    ///
    /// # Panics
    /// (`Standard_ConstructionError` in OCCT) if `radius < 0`.
    pub fn new(x_axis: Ax2d, radius: f64) -> Self {
        assert!(radius >= 0.0, "gp_Circ2d: Radius must be >= 0");
        Self {
            pos: Ax22d2::from_x_axis(x_axis.location, x_axis.direction, true),
            radius,
        }
    }

    /// `gp_Circ2d(const gp_Ax2d& XAxis, Standard_Real Radius,
    ///             Standard_Boolean Sense)`
    ///
    /// As [`Self::new`] but with an explicit orientation sense.
    ///
    /// # Panics
    /// (`Standard_ConstructionError` in OCCT) if `radius < 0`.
    pub fn new_with_sense(x_axis: Ax2d, radius: f64, sense: bool) -> Self {
        assert!(radius >= 0.0, "gp_Circ2d: Radius must be >= 0");
        Self {
            pos: Ax22d2::from_x_axis(x_axis.location, x_axis.direction, sense),
            radius,
        }
    }

    /// `gp_Circ2d(const gp_Ax22d& A, Standard_Real Radius)`
    ///
    /// Constructs a circle from a full 2D local coordinate system.
    /// The origin of `A` is the centre; `XDir` is the "X Direction".
    ///
    /// # Panics
    /// (`Standard_ConstructionError` in OCCT) if `radius < 0`.
    pub fn from_axis(position: Ax22d2, radius: f64) -> Self {
        assert!(radius >= 0.0, "gp_Circ2d: Radius must be >= 0");
        Self {
            pos: position,
            radius,
        }
    }

    // ----------------------------------------------------------------- setters

    /// `SetRadius(Standard_Real R)` — update the radius.
    ///
    /// # Panics
    /// If `radius < 0`.
    pub fn set_radius(&mut self, radius: f64) {
        assert!(radius >= 0.0, "gp_Circ2d::SetRadius: Radius must be >= 0");
        self.radius = radius;
    }

    /// `SetLocation(const gp_Pnt2d& P)` — move the centre.
    pub fn set_location(&mut self, location: Pnt2d) {
        let sense = self.is_direct();
        self.pos = Ax22d2::from_x_axis(location, self.pos.x_direction(), sense);
    }

    /// `SetPosition(const gp_Ax22d& A)` — replace the local frame entirely.
    pub fn set_position(&mut self, position: Ax22d2) {
        self.pos = position;
    }

    /// `SetXAxis(const gp_Ax2d& A)` — redefine the "X Axis" (updates both
    /// location and "X Direction", re-building the frame while preserving the
    /// current handedness).
    pub fn set_x_axis(&mut self, axis: Ax2d) {
        let sense = self.is_direct();
        self.pos = Ax22d2::from_x_axis(axis.location, axis.direction, sense);
    }

    /// `SetYAxis(const gp_Ax2d& A)` — redefine the "Y Axis" (moves the centre
    /// to `axis.location` and makes the "Y Direction" the axis direction, while
    /// rebuilding X = Y rotated -90° to keep the frame orthonormal).
    ///
    /// Note: OCCT rebuilds the whole frame from the supplied Y axis, keeping the
    /// same sense. Here we derive X as the Y direction rotated by -90° (for a
    /// direct frame) so the stored X direction is consistent.
    pub fn set_y_axis(&mut self, axis: Ax2d) {
        let sense = self.is_direct();
        // Derive X from Y: for a direct frame X = Y rotated -90° = (y, -x).
        // For indirect frame X = Y rotated +90° = (-y, x).
        let d = axis.direction.normalized();
        let x_dir = if sense {
            Pnt2d::new(d.y, -d.x)
        } else {
            Pnt2d::new(-d.y, d.x)
        };
        self.pos = Ax22d2::from_x_axis(axis.location, x_dir, sense);
    }

    // ----------------------------------------------------------------- getters

    /// `Radius()` — the radius of the circle.
    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// `Location()` — the centre of the circle (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// `Position()` — the full local coordinate system (`gp_Ax22d`).
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// `XAxis()` — the "X Axis" of the local frame: a line through the centre
    /// in the "X Direction".
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.x_direction())
    }

    /// `YAxis()` — the "Y Axis" of the local frame: a line through the centre
    /// in the "Y Direction".
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.y_direction())
    }

    /// `IsDirect()` — `true` when the local frame is direct (counter-clockwise).
    ///
    /// The frame is direct when the cross product of X and Y directions is
    /// positive (i.e. Y = X rotated +90°).
    #[inline]
    pub fn is_direct(&self) -> bool {
        self.pos.x_direction().cross(self.pos.y_direction()) > 0.0
    }

    // ----------------------------------------------------------- derived geometry

    /// `Area()` — the area enclosed by the circle: `PI * R²`.
    #[inline]
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// `Length()` — the circumference: `2 * PI * R`.
    #[inline]
    pub fn length(&self) -> f64 {
        2.0 * PI * self.radius
    }

    /// `Distance(const gp_Pnt2d& P)` — the absolute distance from `P` to the
    /// circle (i.e. `|distance(centre, P) - R|`).
    ///
    /// Returns 0 when `P` lies exactly on the circle.
    pub fn distance(&self, p: Pnt2d) -> f64 {
        (self.pos.location().distance(p) - self.radius).abs()
    }

    /// `SquareDistance(const gp_Pnt2d& P)` — squared distance from `P` to the
    /// nearest point on the circle.
    ///
    /// This is `(|CP| - R)²` where `|CP|` is the distance from the centre to `P`.
    pub fn square_distance(&self, p: Pnt2d) -> f64 {
        let d = self.distance(p);
        d * d
    }

    /// `Contains(const gp_Pnt2d& P, Standard_Real LinearTolerance)` — `true`
    /// when the signed distance from `P` to the circle is within
    /// `linear_tolerance`.
    pub fn contains(&self, p: Pnt2d, linear_tolerance: f64) -> bool {
        self.distance(p) <= linear_tolerance
    }

    // ---------------------------------------------------------------- transforms

    /// `Translate(const gp_Vec2d& V)` — translate by a 2D vector, returning a
    /// new circle.
    pub fn translated(&self, v: Pnt2d) -> Circ2d {
        let t = Trsf2d::translation(v);
        self.transformed(&t)
    }

    /// `Rotate(const gp_Pnt2d& P, Standard_Real Ang)` — rotate about a 2D
    /// point by `angle` radians (counter-clockwise), returning a new circle.
    pub fn rotated(&self, center: Pnt2d, angle: f64) -> Circ2d {
        let t = Trsf2d::rotation(center, angle);
        self.transformed(&t)
    }

    /// `Mirror(const gp_Pnt2d& P)` — point symmetry through `center`, returning
    /// a new circle.
    ///
    /// The centre is reflected; the X and Y directions are negated (which
    /// reverses the frame handedness), matching OCCT behaviour.
    pub fn mirrored_point(&self, center: Pnt2d) -> Circ2d {
        let new_loc = center * 2.0 - self.pos.location();
        let new_x = -self.pos.x_direction();
        let new_y = -self.pos.y_direction();
        Circ2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            radius: self.radius,
        }
    }

    /// `Mirror(const gp_Ax2d& A)` — reflection across a 2D axis (a line),
    /// returning a new circle.
    ///
    /// Both the centre and the frame directions are reflected across `axis`.
    pub fn mirrored_axis(&self, axis: Ax2d) -> Circ2d {
        let d = axis.direction.normalized();
        // Reflect a 2D point across the line through `axis.location` with
        // direction `d`.
        let mirror_pt = |p: Pnt2d| {
            let rel = p - axis.location;
            let proj = d * (rel.dot(d) * 2.0) - rel;
            axis.location + proj
        };
        // Reflect a 2D direction vector across the line.
        let mirror_dir = |v: Dir2d| {
            let proj = d * (v.dot(d) * 2.0) - v;
            proj.normalized()
        };

        let new_loc = mirror_pt(self.pos.location());
        let new_x = mirror_dir(self.pos.x_direction());
        let new_y = mirror_dir(self.pos.y_direction());
        Circ2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            radius: self.radius,
        }
    }

    /// `Scale(const gp_Pnt2d& P, Standard_Real S)` — uniform scale about
    /// `center`, returning a new circle.
    ///
    /// A negative factor reverses the frame directions (mirror-like), matching
    /// OCCT semantics.
    pub fn scaled(&self, center: Pnt2d, factor: f64) -> Circ2d {
        let new_loc = center + (self.pos.location() - center) * factor;
        let abs_f = factor.abs();
        // A negative scale flips the handedness: negate only the X direction
        // so that (-X).cross(Y) == -(X.cross(Y)), reversing the sense flag
        // inside Ax22d2::new. Negating both X and Y leaves the cross product
        // sign unchanged and would erroneously preserve the frame sense.
        let (new_x, new_y) = if factor >= 0.0 {
            (self.pos.x_direction(), self.pos.y_direction())
        } else {
            (-self.pos.x_direction(), self.pos.y_direction())
        };
        Circ2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            radius: self.radius * abs_f,
        }
    }

    /// `Transform(const gp_Trsf2d& T)` — apply a 2D affine transform, returning
    /// a new circle.
    ///
    /// The centre and directions are mapped through `T`; the radius scales by
    /// `|ScaleFactor(T)|`.
    pub fn transformed(&self, t: &Trsf2d) -> Circ2d {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        let new_x = apply_vector_2d(t, self.pos.x_direction());
        let new_y = apply_vector_2d(t, self.pos.y_direction());
        Circ2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            radius: self.radius * scale.abs(),
        }
    }
}

// ------------------------------------------------------------------ helpers

/// Applies the linear part of a `Trsf2d` to a 2D vector (ignores translation).
#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// Uniform scale factor of a `Trsf2d`: `sqrt(|det(linear part)|)`.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

// ----------------------------------------------------------------- unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::FRAC_PI_2;

    fn unit_circle() -> Circ2d {
        Circ2d::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            5.0,
        )
    }

    #[test]
    fn constructor_stores_radius() {
        let c = unit_circle();
        assert!((c.radius() - 5.0).abs() < CONFUSION);
        assert!(c.location().x.abs() < CONFUSION && c.location().y.abs() < CONFUSION);
    }

    #[test]
    fn area_and_length() {
        let c = unit_circle();
        let expected_area = PI * 25.0;
        let expected_len = 2.0 * PI * 5.0;
        assert!((c.area() - expected_area).abs() < CONFUSION);
        assert!((c.length() - expected_len).abs() < CONFUSION);
    }

    #[test]
    fn distance_from_centre_equals_radius() {
        // A point at distance R from centre lies ON the circle; distance == 0.
        let c = unit_circle();
        let p = Pnt2d::new(5.0, 0.0); // on the circle
        assert!(c.distance(p) < CONFUSION);
    }

    #[test]
    fn distance_from_point_inside() {
        // Centre itself is R away from the circle.
        let c = unit_circle();
        assert!((c.distance(Pnt2d::origin()) - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn contains_on_circle() {
        let c = unit_circle();
        let p = Pnt2d::new(0.0, 5.0); // top of circle
        assert!(c.contains(p, 1e-6));
    }

    #[test]
    fn is_direct_default_frame() {
        let c = unit_circle();
        assert!(c.is_direct());
    }

    #[test]
    fn is_direct_indirect_frame() {
        let c = Circ2d::new_with_sense(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            3.0,
            false,
        );
        assert!(!c.is_direct());
    }

    #[test]
    fn translate_moves_centre() {
        let c = unit_circle();
        let t = c.translated(Pnt2d::new(10.0, -3.0));
        assert!((t.location().x - 10.0).abs() < CONFUSION);
        assert!((t.location().y + 3.0).abs() < CONFUSION);
        assert!((t.radius() - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn rotate_moves_centre() {
        // Circle at (1, 0); rotate 90° CCW about origin → (0, 1).
        let c = Circ2d::new(
            Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
            4.0,
        );
        let r = c.rotated(Pnt2d::origin(), FRAC_PI_2);
        assert!(r.location().x.abs() < CONFUSION);
        assert!((r.location().y - 1.0).abs() < CONFUSION);
        assert!((r.radius() - 4.0).abs() < CONFUSION);
    }

    #[test]
    fn scale_adjusts_radius_and_location() {
        let c = Circ2d::new(
            Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
            3.0,
        );
        let s = c.scaled(Pnt2d::origin(), 2.0);
        assert!((s.radius() - 6.0).abs() < CONFUSION);
        assert!((s.location().x - 2.0).abs() < CONFUSION);
        assert!(s.location().y.abs() < CONFUSION);
    }

    #[test]
    fn x_and_y_axes_directions() {
        let c = unit_circle();
        let xd = c.x_axis().direction.normalized();
        let yd = c.y_axis().direction.normalized();
        assert!((xd.x - 1.0).abs() < ANGULAR && xd.y.abs() < ANGULAR);
        assert!(yd.x.abs() < ANGULAR && (yd.y - 1.0).abs() < ANGULAR);
    }

    #[test]
    fn set_radius_updates() {
        let mut c = unit_circle();
        c.set_radius(10.0);
        assert!((c.radius() - 10.0).abs() < CONFUSION);
        assert!((c.area() - PI * 100.0).abs() < CONFUSION);
    }

    #[test]
    fn mirror_point_reflects_centre() {
        // Circle at (2, 3); mirror through origin → (-2, -3).
        let c = Circ2d::new(
            Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0)),
            5.0,
        );
        let m = c.mirrored_point(Pnt2d::origin());
        assert!((m.location().x + 2.0).abs() < CONFUSION);
        assert!((m.location().y + 3.0).abs() < CONFUSION);
        assert!((m.radius() - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn mirror_axis_reflects_centre() {
        // Circle at (1, 2); reflect across X axis → (1, -2).
        let c = Circ2d::new(
            Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0)),
            3.0,
        );
        let axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let m = c.mirrored_axis(axis);
        assert!((m.location().x - 1.0).abs() < CONFUSION);
        assert!((m.location().y + 2.0).abs() < CONFUSION);
        assert!((m.radius() - 3.0).abs() < CONFUSION);
    }
}
