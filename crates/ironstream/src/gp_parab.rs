//! `gp_Parab` -- analytic 3D parabola primitive, mirroring OpenCascade's
//! `gp_Parab` (package `TKMath`).
//!
//! A parabola is positioned in 3D space by a right-handed local coordinate
//! system (`gp_Ax2`, aliased here as [`crate::gp::Ax3`]): the origin is the
//! **vertex** of the parabola, the "X Direction" points toward the focus
//! (axis of symmetry), and the "Y Direction" is tangent to the parabola at
//! the vertex.  The normal to the parabola plane is the "Z Direction".
//!
//! # Geometry
//! ```text
//!   p  = 2 * focal           -- the parabola parameter (semi-latus rectum)
//!   Focus = Vertex + Focal * XDir
//!   Directrix: line through (Vertex - Focal * XDir), parallel to YDir
//! ```
//!
//! # Invariants (mirroring OCCT `Standard_ConstructionError`)
//! * `focal >= 0`

use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use crate::gp_prim::Lin;

// occt: gp_Parab
/// An analytic 3D parabola: a placement (`gp_Ax2`) plus a focal length.
///
/// The placement's origin is the parabola vertex; `XDir` points along the
/// axis of symmetry toward the focus; `YDir` is tangent to the parabola at
/// the vertex; `ZDir` is the plane normal.
///
/// The **parameter** of the parabola is `p = 2 * focal`.
#[derive(Clone, Copy, Debug)]
pub struct Parab {
    /// Local coordinate system — OCCT calls this `pos` (a `gp_Ax2`).
    pos: Ax3,
    /// Distance from the vertex to the focus.  Must be `>= 0`.
    focal: f64,
}

impl Parab {
    // ---------------------------------------------------------------- constructor

    /// `gp_Parab(const gp_Ax2& A2, Standard_Real Focal)`
    ///
    /// Constructs a parabola with vertex at the origin of `a2`, axis of
    /// symmetry along the "X Direction" of `a2`, and parabola parameter
    /// `p = 2 * focal`.
    ///
    /// # Panics
    /// Panics (`Standard_ConstructionError` in OCCT) if `focal < 0`.
    pub fn new(a2: Ax3, focal: f64) -> Self {
        assert!(
            focal >= 0.0,
            "gp_Parab: Focal must be >= 0"
        );
        Self { pos: a2, focal }
    }

    // ------------------------------------------------------------------ setters

    /// `SetFocal(Standard_Real Focal)` — change the focal length.
    ///
    /// # Panics
    /// Panics if `focal < 0`.
    pub fn set_focal(&mut self, focal: f64) {
        assert!(focal >= 0.0, "gp_Parab::SetFocal: Focal must be >= 0");
        self.focal = focal;
    }

    /// `SetLocation(gp_Pnt)` — move the vertex.
    #[inline]
    pub fn set_location(&mut self, p: Pnt) {
        self.pos.location = p;
    }

    /// `SetPosition(gp_Ax2)` — replace the whole local frame.
    #[inline]
    pub fn set_position(&mut self, a2: Ax3) {
        self.pos = a2;
    }

    // ------------------------------------------------------------------ getters

    /// `Focal()` — the focal length (vertex-to-focus distance).
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// `Parameter()` — the parabola parameter `p = 2 * Focal`.
    ///
    /// This equals the half-latus-rectum doubled, i.e. the length of the
    /// chord through the focus perpendicular to the axis.
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// `Focus()` — the focus: `Vertex + Focal * XDir`.
    #[inline]
    pub fn focus(&self) -> Pnt {
        self.pos.location + self.pos.x_dir * self.focal
    }

    /// `Location()` — the vertex (origin of the local coordinate system).
    #[inline]
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// `Position()` — the local coordinate system (`gp_Ax2`).
    #[inline]
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `Axis()` — the normal axis: a `gp_Ax1` through the vertex along `ZDir`.
    #[inline]
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// `XAxis()` — the symmetry axis: a `gp_Ax1` through the vertex along
    /// `XDir` (pointing toward the focus).
    #[inline]
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    /// `YAxis()` — the tangent axis at the vertex: a `gp_Ax1` through the
    /// vertex along `YDir`.
    #[inline]
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    /// `DirectrixAxis()` — the directrix of the parabola as a `gp_Ax1`.
    ///
    /// The directrix passes through `Vertex - Focal * XDir` and is directed
    /// along `YDir` (perpendicular to the axis of symmetry, in the parabola
    /// plane).
    #[inline]
    pub fn directrix_axis(&self) -> Ax1 {
        let loc = self.pos.location - self.pos.x_dir * self.focal;
        Ax1::new(loc, self.pos.y_dir)
    }

    /// Convenience: the directrix as a `Lin` (infinite line).
    #[inline]
    pub fn directrix(&self) -> Lin {
        Lin::new(self.directrix_axis())
    }

    // ---------------------------------------------------------------- transforms

    /// `Translate(const gp_Vec& V)` — translate by vector `v`.
    pub fn translated(&self, v: Pnt) -> Parab {
        Parab {
            pos: self.pos.transformed(&Trsf::translation(v)),
            focal: self.focal,
        }
    }

    /// `Rotate(const gp_Ax1& A1, Standard_Real Ang)` — rotate about `axis`
    /// by `angle` radians.
    pub fn rotated(&self, axis: Ax1, angle: f64) -> Parab {
        Parab {
            pos: self.pos.transformed(&Trsf::rotation(axis, angle)),
            focal: self.focal,
        }
    }

    /// `Mirror(const gp_Pnt& P)` — point symmetry through `center`.
    ///
    /// The vertex is mirrored; both `XDir` and `YDir` are negated (the shape
    /// is preserved — two negations restore the right-hand rule for `ZDir`).
    pub fn mirrored_point(&self, center: Pnt) -> Parab {
        let mut pos = self.pos;
        pos.location = pos.location.mirrored_point(center);
        pos.x_dir = -pos.x_dir;
        pos.y_dir = -pos.y_dir;
        // z_dir = x_dir × y_dir: negating both x and y keeps z unchanged.
        Parab {
            pos,
            focal: self.focal,
        }
    }

    /// `Mirror(const gp_Ax1& A1)` — reflection across the line `axis`.
    ///
    /// The vertex and all direction vectors are reflected; the focal length
    /// is unchanged.
    pub fn mirrored_axis1(&self, axis: Ax1) -> Parab {
        let a = axis.direction;
        let new_loc = self.pos.location.mirrored_axis(axis);
        let mirror_dir = |d: Pnt| (a * (2.0 * d.dot(a)) - d).normalized();
        let x = mirror_dir(self.pos.x_dir);
        let y = mirror_dir(self.pos.y_dir);
        let z = x.cross(y).normalized();
        Parab {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
            focal: self.focal,
        }
    }

    /// `Mirror(const gp_Ax2& A2)` — reflection across the plane defined by
    /// `mirror` (the XY plane of the given `Ax3`).
    pub fn mirrored_plane(&self, mirror: Ax3) -> Parab {
        let n = mirror.z_dir;
        let mirror_pt = |p: Pnt| {
            let dist = (p - mirror.location).dot(n);
            p - n * (2.0 * dist)
        };
        let mirror_dir = |d: Pnt| (d - n * (2.0 * d.dot(n))).normalized();
        let new_loc = mirror_pt(self.pos.location);
        let x = mirror_dir(self.pos.x_dir);
        let y = mirror_dir(self.pos.y_dir);
        let z = x.cross(y).normalized();
        Parab {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
            focal: self.focal,
        }
    }

    /// `Scale(const gp_Pnt& P, Standard_Real S)` — uniform scaling about
    /// `center` by `factor`.
    ///
    /// The focal length scales by the absolute value of `factor`; the vertex
    /// moves by the scale.  A negative factor inverts X and Y directions
    /// (OCCT behaviour).
    pub fn scaled(&self, center: Pnt, factor: f64) -> Parab {
        let mut t = Trsf::identity();
        t.set_scale(center, factor);
        self.transformed(&t)
    }

    /// `Transform(const gp_Trsf& T)` — apply an affine transform.
    ///
    /// The position is mapped through `T`; the focal length is scaled by
    /// `|T.ScaleFactor()|`.
    pub fn transformed(&self, t: &Trsf) -> Parab {
        Parab {
            pos: self.pos.transformed(t),
            focal: self.focal * t.scale_factor(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::FRAC_PI_2;

    fn xy_plane_ax2() -> Ax3 {
        Ax3::from_origin_normal(
            Pnt::origin(),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        )
    }

    fn standard_parab() -> Parab {
        Parab::new(xy_plane_ax2(), 3.0)
    }

    #[test]
    fn constructor_stores_focal() {
        let p = standard_parab();
        assert!((p.focal() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn parameter_is_twice_focal() {
        let p = standard_parab();
        assert!((p.parameter() - 6.0).abs() < CONFUSION);
    }

    #[test]
    fn focus_position() {
        // Focus = Vertex + Focal * XDir = (3, 0, 0)
        let p = standard_parab();
        assert!(p.focus().is_equal(Pnt::new(3.0, 0.0, 0.0), CONFUSION));
    }

    #[test]
    fn directrix_location() {
        // Directrix passes through Vertex - Focal * XDir = (-3, 0, 0)
        let p = standard_parab();
        let d = p.directrix_axis();
        assert!(d.location.is_equal(Pnt::new(-3.0, 0.0, 0.0), CONFUSION));
        // and is directed along YDir = (0, 1, 0)
        assert!(d.direction.is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR));
    }

    #[test]
    fn axis_directions_identity_frame() {
        let p = Parab::new(Ax3::identity(), 2.0);
        assert!(p.x_axis().direction.is_equal_dir(Pnt::new(1.0, 0.0, 0.0), ANGULAR));
        assert!(p.y_axis().direction.is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR));
        assert!(p.axis().direction.is_equal_dir(Pnt::new(0.0, 0.0, 1.0), ANGULAR));
    }

    #[test]
    fn translate_moves_vertex() {
        let p = standard_parab();
        let v = Pnt::new(10.0, 20.0, 30.0);
        let t = p.translated(v);
        assert!(t.location().is_equal(v, CONFUSION));
        assert!((t.focal() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn rotate_moves_vertex_keeps_focal() {
        let ax2 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let p = Parab::new(ax2, 3.0);
        let rot = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let r = p.rotated(rot, FRAC_PI_2);
        assert!(r.location().is_equal(Pnt::new(0.0, 1.0, 0.0), CONFUSION));
        assert!((r.focal() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn scale_adjusts_focal_and_location() {
        let ax2 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let p = Parab::new(ax2, 3.0);
        let s = p.scaled(Pnt::origin(), 2.0);
        assert!((s.focal() - 6.0).abs() < CONFUSION);
        assert!(s.location().is_equal(Pnt::new(2.0, 0.0, 0.0), CONFUSION));
    }

    #[test]
    fn transform_with_translation() {
        let p = standard_parab();
        let mut t = Trsf::identity();
        t.set_translation(Pnt::new(0.0, 0.0, 5.0));
        let tp = p.transformed(&t);
        assert!((tp.location().z - 5.0).abs() < CONFUSION);
        assert!((tp.focal() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn mirror_point_negates_vertex() {
        let ax2 = Ax3::from_origin_normal(
            Pnt::new(2.0, 3.0, 4.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let p = Parab::new(ax2, 3.0);
        let m = p.mirrored_point(Pnt::origin());
        assert!(m.location().is_equal(Pnt::new(-2.0, -3.0, -4.0), CONFUSION));
        assert!((m.focal() - 3.0).abs() < CONFUSION);
    }
}
