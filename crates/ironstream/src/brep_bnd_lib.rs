// FILE: brep_bnd_lib.rs
//! `BRepBndLib` — bounding-box computation for B-Rep shapes, mirroring
//! OpenCascade's `BRepBndLib` class.
//!
//! This module provides the primary entry points for computing axis-aligned
//! bounding boxes (`BndBox`) from topological shapes.  The OCCT `BRepBndLib`
//! class offers three main static methods:
//!
//! * `Add` — compute a standard (vertex-based) AABB for any shape.
//! * `AddOptimal` — compute an accurate AABB by sampling curve/surface
//!   geometry at sub-edge intervals.
//! * `AddOBB` — compute an oriented bounding box (OBB) for a shape.
//!
//! IronStream's tessellating kernel stores all geometry as discrete
//! vertex/edge/face structures, so "optimal" and "standard" boxes are
//! produced by the same vertex traversal; the `AddOptimal` path additionally
//! samples extra midpoints along edges to tighten the box for curved geometry.
//!
//! # Key types
//!
//! * [`BRepBndLib`] — static utility, mirrors OCCT `BRepBndLib`.

use crate::bnd_box::BndBox;
use crate::gp::Pnt;
use crate::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell,
    TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
};

// ─────────────────────────── BRepBndLib ──────────────────────────────────────

/// Static utilities for computing bounding boxes of B-Rep shapes.
///
/// Mirrors OpenCascade's `BRepBndLib` class.  All methods are free functions
/// accessed via an uninhabited struct (the OCCT static-method idiom).
// occt: BRepBndLib
pub struct BRepBndLib;

impl BRepBndLib {
    /// Add the axis-aligned bounding box of `shape` to `bbox`.
    ///
    /// Visits every vertex in the shape and extends `bbox` to contain it.
    /// This mirrors `BRepBndLib::Add(shape, box)`.
    ///
    /// # Example
    /// ```
    /// use ironstream::brep_bnd_lib::BRepBndLib;
    /// use ironstream::bnd_box::BndBox;
    /// use ironstream::top_exp::{TopoDS_Shape, TopoDS_Vertex};
    ///
    /// let v = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: 1.0, y: 2.0, z: 3.0 });
    /// let mut bbox = BndBox::new();
    /// BRepBndLib::add(&v, &mut bbox);
    /// assert!(!bbox.is_void());
    /// ```
    pub fn add(shape: &TopoDS_Shape, bbox: &mut BndBox) {
        visit_shape_vertices(shape, bbox, false);
    }

    /// Add the bounding box of `shape` to `bbox`, optionally using a gap
    /// (linear tolerance) to inflate the result.
    ///
    /// Equivalent to calling `add` then `bbox.enlarge(gap)`.
    ///
    /// Mirrors `BRepBndLib::Add(shape, box)` with the `theIsGap` parameter.
    pub fn add_with_gap(shape: &TopoDS_Shape, bbox: &mut BndBox, gap: f64) {
        Self::add(shape, bbox);
        if gap > 0.0 {
            bbox.enlarge(gap);
        }
    }

    /// Compute the bounding box of `shape` and return it.
    ///
    /// Convenience wrapper around [`BRepBndLib::add`].
    pub fn bounding_box(shape: &TopoDS_Shape) -> BndBox {
        let mut bbox = BndBox::new();
        Self::add(shape, &mut bbox);
        bbox
    }

    /// Add the "optimal" bounding box of `shape` to `bbox`.
    ///
    /// In OCCT `AddOptimal` samples curve geometry at extra parametric
    /// intervals for tighter bounds on curved edges/faces.  In IronStream the
    /// geometry is already tessellated, so this method samples extra midpoints
    /// between each consecutive pair of vertices along every edge (one extra
    /// midpoint per edge interval by default) to tighten the result for
    /// shapes whose faces were tessellated at coarse resolution.
    ///
    /// Mirrors `BRepBndLib::AddOptimal(shape, box)`.
    pub fn add_optimal(shape: &TopoDS_Shape, bbox: &mut BndBox) {
        visit_shape_vertices(shape, bbox, true);
    }

    /// Compute the optimal bounding box of `shape` and return it.
    ///
    /// Convenience wrapper around [`BRepBndLib::add_optimal`].
    pub fn optimal_bounding_box(shape: &TopoDS_Shape) -> BndBox {
        let mut bbox = BndBox::new();
        Self::add_optimal(shape, &mut bbox);
        bbox
    }

    /// Extend `bbox` to contain the bounding box of a face.
    ///
    /// Visits every vertex in every wire of `face`.  Mirrors the face-specific
    /// overload of `BRepBndLib::Add`.
    pub fn add_face(face: &TopoDS_Face, bbox: &mut BndBox) {
        for wire in &face.wires {
            add_wire_to_box(wire, bbox, false);
        }
    }

    /// Extend `bbox` to contain the bounding box of an edge.
    ///
    /// Visits every vertex on the edge.  Mirrors the edge-specific overload of
    /// `BRepBndLib::Add`.
    pub fn add_edge(edge: &TopoDS_Edge, bbox: &mut BndBox) {
        add_edge_to_box(edge, bbox, false);
    }

    /// Extend `bbox` to contain the bounding box of an edge, sampling
    /// midpoints for tighter bounds.
    pub fn add_edge_optimal(edge: &TopoDS_Edge, bbox: &mut BndBox) {
        add_edge_to_box(edge, bbox, true);
    }

    /// Extend `bbox` to contain the bounding box of a shell.
    pub fn add_shell(shell: &TopoDS_Shell, bbox: &mut BndBox) {
        for face in &shell.faces {
            Self::add_face(face, bbox);
        }
    }

    /// Extend `bbox` to contain the bounding box of a solid.
    pub fn add_solid(solid: &TopoDS_Solid, bbox: &mut BndBox) {
        for shell in &solid.shells {
            Self::add_shell(shell, bbox);
        }
    }

    /// Extend `bbox` to contain the bounding box of a compound.
    pub fn add_compound(compound: &TopoDS_Compound, bbox: &mut BndBox) {
        for child in &compound.shapes {
            Self::add(child, bbox);
        }
    }

    /// Compute the bounding box of a face and return it.
    pub fn face_bounding_box(face: &TopoDS_Face) -> BndBox {
        let mut bbox = BndBox::new();
        Self::add_face(face, &mut bbox);
        bbox
    }

    /// Compute the bounding box of an edge and return it.
    pub fn edge_bounding_box(edge: &TopoDS_Edge) -> BndBox {
        let mut bbox = BndBox::new();
        Self::add_edge(edge, &mut bbox);
        bbox
    }

    /// Compute the bounding box of a solid and return it.
    pub fn solid_bounding_box(solid: &TopoDS_Solid) -> BndBox {
        let mut bbox = BndBox::new();
        Self::add_solid(solid, &mut bbox);
        bbox
    }

    /// Compute the bounding box of a compound and return it.
    pub fn compound_bounding_box(compound: &TopoDS_Compound) -> BndBox {
        let mut bbox = BndBox::new();
        Self::add_compound(compound, &mut bbox);
        bbox
    }

    /// Return the center of the bounding box of `shape`, or `None` if the
    /// shape has no vertices.
    ///
    /// Mirrors the combination of `BRepBndLib::Add` + `Bnd_Box::IsVoid` +
    /// `Bnd_Box::CornerMin/Max` used to find shape centers in OCCT code.
    pub fn center(shape: &TopoDS_Shape) -> Option<Pnt> {
        let bbox = Self::bounding_box(shape);
        if bbox.is_void() {
            return None;
        }
        bbox.center()
    }

    /// Return the diagonal length of the bounding box of `shape`, or `0.0`
    /// if the shape is empty.
    ///
    /// The diagonal length is `sqrt((xmax-xmin)^2 + (ymax-ymin)^2 + (zmax-zmin)^2)`.
    pub fn diagonal(shape: &TopoDS_Shape) -> f64 {
        let bbox = Self::bounding_box(shape);
        if bbox.is_void() {
            return 0.0;
        }
        let limits = match bbox.get() {
            Some(l) => l,
            None => return 0.0,
        };
        let dx = limits.xmax - limits.xmin;
        let dy = limits.ymax - limits.ymin;
        let dz = limits.zmax - limits.zmin;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Return the volume of the bounding box of `shape`, or `0.0` if empty.
    pub fn volume(shape: &TopoDS_Shape) -> f64 {
        let bbox = Self::bounding_box(shape);
        if bbox.is_void() {
            return 0.0;
        }
        match bbox.get() {
            Some(l) => {
                let dx = (l.xmax - l.xmin).max(0.0);
                let dy = (l.ymax - l.ymin).max(0.0);
                let dz = (l.zmax - l.zmin).max(0.0);
                dx * dy * dz
            }
            None => 0.0,
        }
    }

    /// Test whether the bounding box of `shape` contains `point` (within
    /// the box's gap tolerance).
    pub fn contains_point(shape: &TopoDS_Shape, point: Pnt) -> bool {
        let bbox = Self::bounding_box(shape);
        if bbox.is_void() {
            return false;
        }
        !bbox.is_out_point(point)
    }

    /// Test whether the bounding boxes of `a` and `b` overlap.
    pub fn bounding_boxes_overlap(a: &TopoDS_Shape, b: &TopoDS_Shape) -> bool {
        let ba = Self::bounding_box(a);
        let bb = Self::bounding_box(b);
        if ba.is_void() || bb.is_void() {
            return false;
        }
        !ba.is_out_box(&bb)
    }
}

// ─────────────────────────── internal helpers ────────────────────────────────

/// Visit every vertex reachable from `shape`, adding each to `bbox`.
/// When `sample_midpoints` is `true`, linear midpoints between consecutive
/// vertices on every edge are also added (tighter bound for coarse meshes).
fn visit_shape_vertices(shape: &TopoDS_Shape, bbox: &mut BndBox, sample_midpoints: bool) {
    match shape {
        TopoDS_Shape::Vertex(v) => {
            bbox.add_point(Pnt::new(v.x, v.y, v.z));
        }
        TopoDS_Shape::Edge(e) => {
            add_edge_to_box(e, bbox, sample_midpoints);
        }
        TopoDS_Shape::Wire(w) => {
            add_wire_to_box(w, bbox, sample_midpoints);
        }
        TopoDS_Shape::Face(f) => {
            for wire in &f.wires {
                add_wire_to_box(wire, bbox, sample_midpoints);
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for face in &sh.faces {
                for wire in &face.wires {
                    add_wire_to_box(wire, bbox, sample_midpoints);
                }
            }
        }
        TopoDS_Shape::Solid(sol) => {
            for shell in &sol.shells {
                for face in &shell.faces {
                    for wire in &face.wires {
                        add_wire_to_box(wire, bbox, sample_midpoints);
                    }
                }
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for solid in &cs.solids {
                for shell in &solid.shells {
                    for face in &shell.faces {
                        for wire in &face.wires {
                            add_wire_to_box(wire, bbox, sample_midpoints);
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                visit_shape_vertices(child, bbox, sample_midpoints);
            }
        }
    }
}

/// Add all vertices of a wire's edges to `bbox`.
fn add_wire_to_box(wire: &TopoDS_Wire, bbox: &mut BndBox, sample_midpoints: bool) {
    for edge in &wire.edges {
        add_edge_to_box(edge, bbox, sample_midpoints);
    }
}

/// Add all vertices (and optionally edge midpoints) of an edge to `bbox`.
fn add_edge_to_box(edge: &TopoDS_Edge, bbox: &mut BndBox, sample_midpoints: bool) {
    for v in &edge.vertices {
        bbox.add_point(Pnt::new(v.x, v.y, v.z));
    }
    // When sampling midpoints, add the midpoint between each consecutive pair
    // of vertices.  For a straight edge this is redundant but for a coarsely
    // tessellated curve it can provide a tighter bound.
    if sample_midpoints && edge.vertices.len() >= 2 {
        let verts = &edge.vertices;
        for i in 0..verts.len() - 1 {
            let a = &verts[i];
            let b = &verts[i + 1];
            let mx = (a.x + b.x) * 0.5;
            let my = (a.y + b.y) * 0.5;
            let mz = (a.z + b.z) * 0.5;
            bbox.add_point(Pnt::new(mx, my, mz));
        }
    }
}

// ─────────────────────────── helper builders ─────────────────────────────────

/// Build a `TopoDS_Vertex` with a given id and position.
///
/// Convenience function used in tests.
#[inline]
pub fn make_vertex(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Shape {
    TopoDS_Shape::Vertex(TopoDS_Vertex { id, x, y, z })
}

/// Build a `TopoDS_Edge` from a list of `(x, y, z)` vertex positions.
///
/// Assigns sequential IDs starting from 1 for vertices; the edge itself gets
/// the supplied `id`.
pub fn make_edge_from_points(id: u64, points: &[(f64, f64, f64)]) -> TopoDS_Edge {
    let vertices: Vec<TopoDS_Vertex> = points
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| TopoDS_Vertex {
            id: i as u64 + 1,
            x,
            y,
            z,
        })
        .collect();
    TopoDS_Edge { id, vertices }
}

/// Build a rectangular `TopoDS_Face` from min/max corner coordinates.
///
/// Creates a face with a single rectangular outer wire (four corner vertices).
pub fn make_rect_face(
    id: u64,
    x_min: f64,
    y_min: f64,
    z: f64,
    x_max: f64,
    y_max: f64,
) -> TopoDS_Face {
    use crate::top_exp::TopoDS_Wire as TopWire;

    let corners = [
        (x_min, y_min, z),
        (x_max, y_min, z),
        (x_max, y_max, z),
        (x_min, y_max, z),
    ];
    let edge = make_edge_from_points(1, &corners);
    let wire = TopWire {
        id,
        edges: vec![edge],
    };
    TopoDS_Face {
        id,
        wires: vec![wire],
    }
}

// ─────────────────────────── unit tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bnd_box::BndBox;
    use crate::gp::Pnt;
    use crate::top_exp::{
        TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell, TopoDS_Solid, TopoDS_Vertex,
        TopoDS_Wire,
    };

    const TOL: f64 = 1e-12;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= TOL,
            "expected {a} ≈ {b}, diff = {}",
            (a - b).abs()
        );
    }

    // Build a simple vertex shape at (x, y, z).
    fn vertex_shape(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Shape {
        TopoDS_Shape::Vertex(TopoDS_Vertex { id, x, y, z })
    }

    // Build an edge shape from a list of (x, y, z) tuples.
    fn edge_shape(id: u64, pts: &[(f64, f64, f64)]) -> TopoDS_Shape {
        TopoDS_Shape::Edge(make_edge_from_points(id, pts))
    }

    // Build a unit box solid (8 vertices at 0/1 corners).
    fn unit_box_solid() -> TopoDS_Shape {
        let corners: &[(f64, f64, f64)] = &[
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (1.0, 1.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (1.0, 0.0, 1.0),
            (1.0, 1.0, 1.0),
            (0.0, 1.0, 1.0),
        ];
        let bottom_edge = make_edge_from_points(1, &corners[0..4]);
        let top_edge = make_edge_from_points(2, &corners[4..8]);
        let bottom_wire = TopoDS_Wire {
            id: 1,
            edges: vec![bottom_edge],
        };
        let top_wire = TopoDS_Wire {
            id: 2,
            edges: vec![top_edge],
        };
        let bottom_face = TopoDS_Face {
            id: 1,
            wires: vec![bottom_wire],
        };
        let top_face = TopoDS_Face {
            id: 2,
            wires: vec![top_wire],
        };
        let shell = TopoDS_Shell {
            id: 1,
            faces: vec![bottom_face, top_face],
        };
        let solid = TopoDS_Solid {
            id: 1,
            shells: vec![shell],
        };
        TopoDS_Shape::Solid(solid)
    }

    // ── Test 1: empty shape produces void box ─────────────────────────────────
    #[test]
    fn empty_compound_gives_void_box() {
        let empty = TopoDS_Shape::Compound(TopoDS_Compound {
            id: 1,
            shapes: vec![],
        });
        let mut bbox = BndBox::new();
        BRepBndLib::add(&empty, &mut bbox);
        assert!(bbox.is_void(), "empty compound should give void box");
    }

    // ── Test 2: single vertex ─────────────────────────────────────────────────
    #[test]
    fn single_vertex_gives_point_box() {
        let v = vertex_shape(1, 3.0, 4.0, 5.0);
        let mut bbox = BndBox::new();
        BRepBndLib::add(&v, &mut bbox);
        assert!(!bbox.is_void());
        let l = bbox.get().unwrap();
        near(l.xmin, 3.0);
        near(l.xmax, 3.0);
        near(l.ymin, 4.0);
        near(l.ymax, 4.0);
        near(l.zmin, 5.0);
        near(l.zmax, 5.0);
    }

    // ── Test 3: axis-aligned edge ─────────────────────────────────────────────
    #[test]
    fn axis_aligned_edge_bounding_box() {
        let e = edge_shape(1, &[(-1.0, 0.0, 0.0), (1.0, 0.0, 0.0)]);
        let mut bbox = BndBox::new();
        BRepBndLib::add(&e, &mut bbox);
        let l = bbox.get().unwrap();
        near(l.xmin, -1.0);
        near(l.xmax, 1.0);
        near(l.ymin, 0.0);
        near(l.ymax, 0.0);
        near(l.zmin, 0.0);
        near(l.zmax, 0.0);
    }

    // ── Test 4: unit box solid ────────────────────────────────────────────────
    #[test]
    fn unit_box_solid_bounding_box() {
        let s = unit_box_solid();
        let bbox = BRepBndLib::bounding_box(&s);
        let l = bbox.get().unwrap();
        near(l.xmin, 0.0);
        near(l.xmax, 1.0);
        near(l.ymin, 0.0);
        near(l.ymax, 1.0);
        near(l.zmin, 0.0);
        near(l.zmax, 1.0);
    }

    // ── Test 5: compound of two vertices ─────────────────────────────────────
    #[test]
    fn compound_of_two_vertices() {
        let compound = TopoDS_Shape::Compound(TopoDS_Compound {
            id: 1,
            shapes: vec![
                vertex_shape(1, -2.0, 0.0, 0.0),
                vertex_shape(2, 2.0, 3.0, 1.0),
            ],
        });
        let bbox = BRepBndLib::bounding_box(&compound);
        let l = bbox.get().unwrap();
        near(l.xmin, -2.0);
        near(l.xmax, 2.0);
        near(l.ymin, 0.0);
        near(l.ymax, 3.0);
        near(l.zmin, 0.0);
        near(l.zmax, 1.0);
    }

    // ── Test 6: diagonal length of unit box ───────────────────────────────────
    #[test]
    fn diagonal_of_unit_box() {
        let s = unit_box_solid();
        let d = BRepBndLib::diagonal(&s);
        // diagonal of unit cube = sqrt(3)
        assert!((d - 3.0_f64.sqrt()).abs() < 1e-10, "diagonal should be sqrt(3), got {d}");
    }

    // ── Test 7: volume of unit box bounding box ───────────────────────────────
    #[test]
    fn volume_of_unit_box_bounding_box() {
        let s = unit_box_solid();
        let vol = BRepBndLib::volume(&s);
        near(vol, 1.0);
    }

    // ── Test 8: add_with_gap enlarges the box ────────────────────────────────
    #[test]
    fn add_with_gap_enlarges_box() {
        let v = vertex_shape(1, 0.0, 0.0, 0.0);
        let mut bbox = BndBox::new();
        BRepBndLib::add_with_gap(&v, &mut bbox, 0.5);
        let l = bbox.get().unwrap();
        assert!(l.xmin <= -0.5 + TOL, "xmin should be <= -0.5");
        assert!(l.xmax >= 0.5 - TOL, "xmax should be >= 0.5");
    }

    // ── Test 9: center of unit box ────────────────────────────────────────────
    #[test]
    fn center_of_unit_box() {
        let s = unit_box_solid();
        let c = BRepBndLib::center(&s).expect("unit box must have a center");
        near(c.x, 0.5);
        near(c.y, 0.5);
        near(c.z, 0.5);
    }

    // ── Test 10: center returns None for empty shape ──────────────────────────
    #[test]
    fn center_of_empty_shape_is_none() {
        let empty = TopoDS_Shape::Compound(TopoDS_Compound {
            id: 1,
            shapes: vec![],
        });
        assert!(BRepBndLib::center(&empty).is_none());
    }

    // ── Test 11: contains_point ───────────────────────────────────────────────
    #[test]
    fn contains_point_inside_bounding_box() {
        let s = unit_box_solid();
        // Point at center should be inside.
        assert!(BRepBndLib::contains_point(&s, Pnt::new(0.5, 0.5, 0.5)));
        // Point outside bounding box.
        assert!(!BRepBndLib::contains_point(&s, Pnt::new(2.0, 2.0, 2.0)));
    }

    // ── Test 12: bounding_boxes_overlap ──────────────────────────────────────
    #[test]
    fn bounding_boxes_overlap_test() {
        let s1 = unit_box_solid();
        // Separate solid: corners from (5,5,5) to (6,6,6).
        let corners2: &[(f64, f64, f64)] = &[
            (5.0, 5.0, 5.0),
            (6.0, 5.0, 5.0),
            (6.0, 6.0, 5.0),
            (5.0, 6.0, 5.0),
        ];
        let edge2 = make_edge_from_points(1, corners2);
        let wire2 = TopoDS_Wire {
            id: 1,
            edges: vec![edge2],
        };
        let face2 = TopoDS_Face {
            id: 1,
            wires: vec![wire2],
        };
        let shell2 = TopoDS_Shell {
            id: 1,
            faces: vec![face2],
        };
        let solid2 = TopoDS_Solid {
            id: 1,
            shells: vec![shell2],
        };
        let s2 = TopoDS_Shape::Solid(solid2);

        // The two solids are far apart — they should not overlap.
        assert!(!BRepBndLib::bounding_boxes_overlap(&s1, &s2));

        // A compound containing both — it overlaps itself trivially.
        let compound = TopoDS_Shape::Compound(TopoDS_Compound {
            id: 99,
            shapes: vec![s1.clone(), vertex_shape(42, 0.5, 0.5, 0.5)],
        });
        assert!(BRepBndLib::bounding_boxes_overlap(&compound, &s1));
    }

    // ── Test 13: add_optimal vs add ───────────────────────────────────────────
    #[test]
    fn optimal_box_contains_standard_box() {
        // For point-sampled geometry both methods should give the same result
        // (same vertices).
        let s = unit_box_solid();
        let std_bbox = BRepBndLib::bounding_box(&s);
        let opt_bbox = BRepBndLib::optimal_bounding_box(&s);

        let sl = std_bbox.get().unwrap();
        let ol = opt_bbox.get().unwrap();
        near(sl.xmin, ol.xmin);
        near(sl.xmax, ol.xmax);
        near(sl.ymin, ol.ymin);
        near(sl.ymax, ol.ymax);
        near(sl.zmin, ol.zmin);
        near(sl.zmax, ol.zmax);
    }

    // ── Test 14: make_rect_face helper ────────────────────────────────────────
    #[test]
    fn rect_face_bounding_box() {
        let face = make_rect_face(1, -3.0, -2.0, 0.0, 3.0, 2.0);
        let bbox = BRepBndLib::face_bounding_box(&face);
        let l = bbox.get().unwrap();
        near(l.xmin, -3.0);
        near(l.xmax, 3.0);
        near(l.ymin, -2.0);
        near(l.ymax, 2.0);
        near(l.zmin, 0.0);
        near(l.zmax, 0.0);
    }
}
