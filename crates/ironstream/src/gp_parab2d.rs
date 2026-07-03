//! `gp_Parab2d` -- analytic 2D parabola primitive, mirroring OpenCascade's
//! `gp_Parab2d` (package `TKMath`).
//!
//! A 2D parabola is positioned in the plane by a local coordinate system
//! ([`Ax22d2`]): its origin is the **vertex** of the parabola; its "X Direction"
//! points toward the focus; its "Y Direction" is tangent to the parabola at the
//! vertex.
//!
//! # Parameterisation
//!
//! ```text
//! P(U) = O + (U² / (2·p)) · XDir + U · YDir
//! ```
//!
//! where `p = 2 · FocalLength` is the **parameter** (semi-latus rectum) and `O`
//! is the vertex.
//!
//! # Derived geometry
//! ```text
//! Focus     = O + focal · XDir
//! Directrix = line through (O − focal · XDir) in the YDir direction
//! Parameter = 2 · focal
//! ```
//!
//! # Invariant
//! `focal >= 0`  (mirrors `Standard_ConstructionError` if violated).

use crate::geom2d_circle::{Ax22d2, Dir2d};
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d};

// ─────────────────────────────────────────────────────── type ─────────────

/// An analytic 2D parabola: a placement ([`Ax22d2`]) plus a focal length.
///
/// The placement's origin is the parabola **vertex**; `XDir` points from the
/// vertex toward the **focus**; `YDir` is the tangent at the vertex.
// occt: gp_Parab2d
#[derive(Clone, Copy, Debug)]
pub struct Parab2d {
    /// Local coordinate system (OCCT `pos`, type `gp_Ax22d`).
    pos: Ax22d2,
    /// Focal length: distance from vertex to focus. Must be `>= 0`.
    focal: f64,
}

// ─────────────────────────────────────────────────── constructors ─────────

impl Parab2d {
    /// `gp_Parab2d(const gp_Ax2d& MirrorAxis, Standard_Real Focal,
    ///             Standard_Boolean Sense = Standard_True)`
    ///
    /// Constructs a parabola from its symmetry axis (providing the vertex
    /// location and the "X Direction") and a focal length. The frame is direct
    /// (counter-clockwise) when `sense` is `true`.
    ///
    /// # Panics
    /// (`Standard_ConstructionError` in OCCT) if `focal < 0`.
    pub fn new(axis: Ax2d, focal: f64) -> Self {
        assert!(
            focal >= 0.0,
            "gp_Parab2d: Focal must be >= 0"
        );
        Self {
            pos: Ax22d2::from_x_axis(axis.location, axis.direction, true),
            focal,
        }
    }

    /// `gp_Parab2d(const gp_Ax2d& MirrorAxis, Standard_Real Focal,
    ///             Standard_Boolean Sense)`
    ///
    /// As [`Self::new`] but with an explicit orientation sense.
    pub fn new_with_sense(axis: Ax2d, focal: f64, sense: bool) -> Self {
        assert!(
            focal >= 0.0,
            "gp_Parab2d: Focal must be >= 0"
        );
        Self {
            pos: Ax22d2::from_x_axis(axis.location, axis.direction, sense),
            focal,
        }
    }

    /// `gp_Parab2d(const gp_Ax22d& A, Standard_Real Focal)`
    ///
    /// Constructs a parabola from a full 2D local coordinate system.
    ///
    /// # Panics
    /// (`Standard_ConstructionError` in OCCT) if `focal < 0`.
    pub fn from_axis(position: Ax22d2, focal: f64) -> Self {
        assert!(
            focal >= 0.0,
            "gp_Parab2d: Focal must be >= 0"
        );
        Self {
            pos: position,
            focal,
        }
    }

    // ─────────────────────────────────────────────────────── setters ──────────

    /// `SetFocal(Standard_Real Focal)` — update the focal length.
    ///
    /// # Panics
    /// If `focal < 0`.
    pub fn set_focal(&mut self, focal: f64) {
        assert!(focal >= 0.0, "gp_Parab2d::SetFocal: Focal must be >= 0");
        self.focal = focal;
    }

    /// `SetLocation(const gp_Pnt2d& P)` — move the vertex.
    pub fn set_location(&mut self, location: Pnt2d) {
        let sense = self.is_direct();
        self.pos = Ax22d2::from_x_axis(location, self.pos.x_direction(), sense);
    }

    /// `SetPosition(const gp_Ax22d& A)` — replace the local frame.
    pub fn set_position(&mut self, position: Ax22d2) {
        self.pos = position;
    }

    // ─────────────────────────────────────────────────────── getters ──────────

    /// `Focal()` — the focal length (distance from vertex to focus).
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// `Parameter()` — the parabola parameter `p = 2 * Focal` (semi-latus rectum).
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// `Location()` — the vertex (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// `Position()` — the full local coordinate system.
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// `IsDirect()` — `true` when the frame is direct (counter-clockwise).
    ///
    /// The frame is direct when the cross product of X and Y directions is
    /// positive (i.e. Y = X rotated +90°).
    #[inline]
    pub fn is_direct(&self) -> bool {
        self.pos.x_direction().cross(self.pos.y_direction()) > 0.0
    }

    /// `XAxis()` — the axis of symmetry through the vertex in the "X Direction".
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.x_direction())
    }

    /// `YAxis()` — the tangent axis at the vertex: through the vertex in "Y Direction".
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.y_direction())
    }

    // ─────────────────────────────────────────────────── derived geometry ──────

    /// `Focus()` — the focus of the parabola: `Vertex + Focal · XDir`.
    pub fn focus(&self) -> Pnt2d {
        self.pos.location() + self.pos.x_direction() * self.focal
    }

    /// `Directrix()` — the directrix of the parabola.
    ///
    /// The directrix is the line through `Vertex − Focal · XDir` in the
    /// "Y Direction" (i.e. perpendicular to the axis of symmetry).
    ///
    /// Every point on the parabola is equidistant from the focus and the
    /// directrix.
    pub fn directrix(&self) -> Ax2d {
        let loc = self.pos.location() - self.pos.x_direction() * self.focal;
        Ax2d::new(loc, self.pos.y_direction())
    }

    // ─────────────────────────────────────────────────────── transforms ───────

    /// `Translate(const gp_Vec2d& V)` — translate by a 2D vector.
    pub fn translated(&self, v: Pnt2d) -> Parab2d {
        let t = Trsf2d::translation(v);
        self.transformed(&t)
    }

    /// `Rotate(const gp_Pnt2d& P, Standard_Real Ang)` — rotate about a 2D point.
    pub fn rotated(&self, center: Pnt2d, angle: f64) -> Parab2d {
        let t = Trsf2d::rotation(center, angle);
        self.transformed(&t)
    }

    /// `Mirror(const gp_Pnt2d& P)` — point symmetry through `center`.
    ///
    /// The vertex is reflected; the X and Y directions are negated (which
    /// reverses the frame handedness, matching OCCT behaviour).
    pub fn mirrored_point(&self, center: Pnt2d) -> Parab2d {
        let new_loc = center * 2.0 - self.pos.location();
        let new_x = -self.pos.x_direction();
        let new_y = -self.pos.y_direction();
        Parab2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            focal: self.focal,
        }
    }

    /// `Mirror(const gp_Ax2d& A)` — reflection across a 2D axis (a line).
    ///
    /// Both the vertex and the frame directions are reflected across `axis`.
    pub fn mirrored_axis(&self, axis: Ax2d) -> Parab2d {
        let d = axis.direction.normalized();
        let mirror_pt = |p: Pnt2d| {
            let rel = p - axis.location;
            let proj = d * (rel.dot(d) * 2.0) - rel;
            axis.location + proj
        };
        let mirror_dir = |v: Dir2d| {
            let proj = d * (v.dot(d) * 2.0) - v;
            proj.normalized()
        };
        let new_loc = mirror_pt(self.pos.location());
        let new_x = mirror_dir(self.pos.x_direction());
        let new_y = mirror_dir(self.pos.y_direction());
        Parab2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            focal: self.focal,
        }
    }

    /// `Scale(const gp_Pnt2d& P, Standard_Real S)` — uniform scale about `center`.
    ///
    /// A negative factor reverses the directions, matching OCCT semantics.
    pub fn scaled(&self, center: Pnt2d, factor: f64) -> Parab2d {
        let new_loc = center + (self.pos.location() - center) * factor;
        let abs_f = factor.abs();
        let (new_x, new_y) = if factor >= 0.0 {
            (self.pos.x_direction(), self.pos.y_direction())
        } else {
            (-self.pos.x_direction(), -self.pos.y_direction())
        };
        Parab2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            focal: self.focal * abs_f,
        }
    }

    /// `Transform(const gp_Trsf2d& T)` — apply a 2D affine transform.
    ///
    /// The vertex and frame directions are mapped through `T`; the focal length
    /// scales by `|ScaleFactor(T)|`.
    pub fn transformed(&self, t: &Trsf2d) -> Parab2d {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        let new_x = apply_vector_2d(t, self.pos.x_direction());
        let new_y = apply_vector_2d(t, self.pos.y_direction());
        Parab2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            focal: self.focal * scale.abs(),
        }
    }
}

// ─────────────────────────────────────────────────────── helpers ──────────

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

// ─────────────────────────────────────────────────────── unit tests ───────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::FRAC_PI_2;

    fn unit_parab() -> Parab2d {
        Parab2d::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            2.0,
        )
    }

    #[test]
    fn constructor_stores_focal() {
        let p = unit_parab();
        assert!((p.focal() - 2.0).abs() < CONFUSION);
        assert!((p.parameter() - 4.0).abs() < CONFUSION);
    }

    #[test]
    fn focus_position() {
        let p = unit_parab();
        let f = p.focus();
        assert!((f.x - 2.0).abs() < CONFUSION && f.y.abs() < CONFUSION);
    }

    #[test]
    fn directrix_position() {
        let p = unit_parab();
        let d = p.directrix();
        // Directrix at x = -focal = -2
        assert!((d.location.x + 2.0).abs() < CONFUSION);
        assert!(d.location.y.abs() < CONFUSION);
        // Directrix direction is Y-like (perpendicular to X axis)
        assert!(d.direction.x.abs() < ANGULAR);
        assert!((d.direction.y.abs() - 1.0).abs() < ANGULAR);
    }

    #[test]
    fn translate_moves_vertex() {
        let p = unit_parab();
        let t = p.translated(Pnt2d::new(5.0, 3.0));
        assert!((t.location().x - 5.0).abs() < CONFUSION);
        assert!((t.location().y - 3.0).abs() < CONFUSION);
        assert!((t.focal() - 2.0).abs() < CONFUSION);
    }

    #[test]
    fn rotate_moves_vertex() {
        // Parab at (1,0); rotate 90° CCW about origin → vertex at (0,1).
        let p = Parab2d::new(
            Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
            3.0,
        );
        let r = p.rotated(Pnt2d::origin(), FRAC_PI_2);
        assert!(r.location().x.abs() < CONFUSION);
        assert!((r.location().y - 1.0).abs() < CONFUSION);
        assert!((r.focal() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn scale_adjusts_focal_and_location() {
        let p = Parab2d::new(
            Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
            2.0,
        );
        let s = p.scaled(Pnt2d::origin(), 3.0);
        assert!((s.focal() - 6.0).abs() < CONFUSION);
        assert!((s.location().x - 3.0).abs() < CONFUSION);
        assert!(s.location().y.abs() < CONFUSION);
    }

    #[test]
    fn is_direct_default_frame() {
        let p = unit_parab();
        assert!(p.is_direct());
    }

    #[test]
    fn is_direct_indirect_frame() {
        let p = Parab2d::new_with_sense(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            2.0,
            false,
        );
        assert!(!p.is_direct());
    }
}
