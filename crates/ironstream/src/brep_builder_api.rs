//! `BRepBuilderAPI` — wire/face construction and transforms, mirroring OCCT's
//! `BRepBuilderAPI_MakePolygon` / `MakeFace` / `BRepBuilderAPI_Transform`.

use crate::gp::{Pnt, Trsf};
use crate::topods::{Face, Solid, Wire};

/// `BRepBuilderAPI_MakePolygon` — a closed polygonal wire from ordered points.
// occt: BRepBuilderAPI_MakePolygon
pub fn make_polygon(points: &[Pnt]) -> Wire {
    Wire::new(points.to_vec())
}

/// `BRepBuilderAPI_MakeFace` — a planar face from an outer wire.
// occt: BRepBuilderAPI_MakeFace
pub fn make_face(outer: Wire) -> Face {
    Face::new(outer)
}

/// A planar face with interior holes.
pub fn make_face_with_holes(outer: Wire, holes: Vec<Wire>) -> Face {
    Face::with_holes(outer, holes)
}

/// `BRepBuilderAPI_Transform` — apply an affine transform to a solid.
// occt: BRepBuilderAPI_Transform
pub fn transform(solid: &Solid, t: &Trsf) -> Solid {
    solid.transformed(t)
}
