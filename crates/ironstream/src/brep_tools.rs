// FILE: brep_tools.rs
//! `BRepTools` — shape utilities for boundary-representation geometry,
//! mirroring OpenCascade's `BRepTools` package and `BRepTools_WireExplorer`.
//!
//! # Key types
//!
//! * [`BRepTools`] — static utilities: bounding box, UV bounds, outer wire,
//!   shape comparison, wire ordering, and shape cleaning.
//! * [`WireExplorer`] — ordered traversal of a wire's edges in connected
//!   sequence (mirrors `BRepTools_WireExplorer`).

use crate::bnd_box::BndBox;
use crate::gp::{Pnt, EPS};
use crate::top_exp::{
    TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Vertex, TopoDS_Wire,
};

// ---------------------------------------------------------------------------
// BRepTools
// ---------------------------------------------------------------------------

/// Static utilities for working with boundary-representation shapes.
///
/// Mirrors the OCCT `BRepTools` class.
// occt: BRepTools
pub struct BRepTools;

impl BRepTools {
    /// Compute the 3-D axis-aligned bounding box of a shape by visiting every
    /// vertex it contains.
    ///
    /// Mirrors `BRepTools::Add(box, shape)` / `BRepBndLib::Add`.
    pub fn bounding_box(shape: &TopoDS_Shape) -> BndBox {
        let mut bbox = BndBox::new();
        collect_vertex_bounds(shape, &mut bbox);
        bbox
    }

    /// Compute the 3-D bounding box of a face (visits all vertices in all
    /// wires of that face).
    ///
    /// Mirrors `BRepTools::UVBounds` applied to 3-D geometry.
    pub fn face_bounding_box(face: &TopoDS_Face) -> BndBox {
        let mut bbox = BndBox::new();
        for wire in &face.wires {
            for edge in &wire.edges {
                for v in &edge.vertices {
                    bbox.add_point(Pnt::new(v.x, v.y, v.z));
                }
            }
        }
        bbox
    }

    /// Return the outer (first) wire of a face, if any.
    ///
    /// In OCCT a face can carry several wires; by convention the first wire is
    /// the outer boundary.  Mirrors `BRepTools::OuterWire(face)`.
    pub fn outer_wire(face: &TopoDS_Face) -> Option<&TopoDS_Wire> {
        face.wires.first()
    }

    /// Return the 2-D UV parameter bounds of a wire by projecting all vertices
    /// onto the `(u, v)` plane defined by the supplied `origin`, `x_dir`, and
    /// `y_dir` unit vectors.
    ///
    /// Returns `(u_min, u_max, v_min, v_max)`, or `None` if the wire has no
    /// vertices.
    ///
    /// Mirrors `BRepTools::UVBounds(face, surface, umin, umax, vmin, vmax)`.
    pub fn uv_bounds(
        wire: &TopoDS_Wire,
        origin: Pnt,
        x_dir: Pnt,
        y_dir: Pnt,
    ) -> Option<(f64, f64, f64, f64)> {
        let mut u_min = f64::MAX;
        let mut u_max = f64::MIN;
        let mut v_min = f64::MAX;
        let mut v_max = f64::MIN;
        let mut any = false;

        for edge in &wire.edges {
            for vtx in &edge.vertices {
                let rel = Pnt::new(vtx.x - origin.x, vtx.y - origin.y, vtx.z - origin.z);
                let u = rel.dot(x_dir);
                let v = rel.dot(y_dir);
                u_min = u_min.min(u);
                u_max = u_max.max(u);
                v_min = v_min.min(v);
                v_max = v_max.max(v);
                any = true;
            }
        }

        if any {
            Some((u_min, u_max, v_min, v_max))
        } else {
            None
        }
    }

    /// Check whether two shapes are geometrically equal by comparing all
    /// vertices pairwise within `tolerance`.
    ///
    /// Returns `true` only when the vertex sets of both shapes are identical in
    /// size and every vertex from one shape has a matching vertex in the other
    /// within `tolerance`.
    ///
    /// Mirrors `BRepTools::Compare`.
    pub fn compare(s1: &TopoDS_Shape, s2: &TopoDS_Shape, tolerance: f64) -> bool {
        let verts1 = collect_vertices(s1);
        let verts2 = collect_vertices(s2);
        if verts1.len() != verts2.len() {
            return false;
        }
        // Every vertex in s1 must find a match in s2.
        'outer: for v1 in &verts1 {
            for v2 in &verts2 {
                let dx = v1.x - v2.x;
                let dy = v1.y - v2.y;
                let dz = v1.z - v2.z;
                if (dx * dx + dy * dy + dz * dz).sqrt() <= tolerance {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }

    /// Return the number of edges in a wire.
    ///
    /// Mirrors `BRepTools::NbEdges(wire)`.
    pub fn nb_edges(wire: &TopoDS_Wire) -> usize {
        wire.edges.len()
    }

    /// Return the total number of distinct vertices across all edges of a wire.
    /// Shared vertices (same `id`) are counted once.
    ///
    /// Mirrors `BRepTools::NbVertices(wire)` (no such function exists verbatim
    /// in OCCT, but the concept maps to querying the wire topology).
    pub fn nb_vertices(wire: &TopoDS_Wire) -> usize {
        let mut ids: Vec<u64> = Vec::new();
        for edge in &wire.edges {
            for v in &edge.vertices {
                if !ids.contains(&v.id) {
                    ids.push(v.id);
                }
            }
        }
        ids.len()
    }

    /// Test whether a wire is closed: the last vertex of the last edge is
    /// within `tolerance` of the first vertex of the first edge.
    ///
    /// Mirrors `BRepTools::IsClosed(wire)`.
    pub fn is_closed(wire: &TopoDS_Wire, tolerance: f64) -> bool {
        if wire.edges.is_empty() {
            return false;
        }
        let first_edge = &wire.edges[0];
        let last_edge = &wire.edges[wire.edges.len() - 1];
        let v_first = match first_edge.vertices.first() {
            Some(v) => v,
            None => return false,
        };
        let v_last = match last_edge.vertices.last() {
            Some(v) => v,
            None => return false,
        };
        let dx = v_first.x - v_last.x;
        let dy = v_first.y - v_last.y;
        let dz = v_first.z - v_last.z;
        (dx * dx + dy * dy + dz * dz).sqrt() <= tolerance
    }

    /// Clean a wire by removing degenerate edges (edges whose two endpoint
    /// vertices coincide within `tolerance`).
    ///
    /// Mirrors `BRepTools::Clean(shape)` in spirit.
    pub fn clean_wire(wire: &TopoDS_Wire, tolerance: f64) -> TopoDS_Wire {
        let edges: Vec<TopoDS_Edge> = wire
            .edges
            .iter()
            .filter(|e| !is_degenerate_edge(e, tolerance))
            .cloned()
            .collect();
        TopoDS_Wire {
            id: wire.id,
            edges,
        }
    }

    /// Compute the approximate perimeter (sum of edge lengths) of a wire.
    ///
    /// Edge length is computed as the Euclidean distance between the first and
    /// last vertex of each edge (i.e. straight-line segments; polyline edges
    /// would require sampling but are not used here).
    ///
    /// Mirrors `BRepTools::SurfaceProperties` / GProp-based perimeter queries.
    pub fn wire_perimeter(wire: &TopoDS_Wire) -> f64 {
        wire.edges.iter().map(edge_chord_length).sum()
    }

    /// Compute the centroid of a wire by averaging all vertex positions.
    ///
    /// Mirrors OCCT's use of `GProp_GProps` for first-order properties.
    pub fn wire_centroid(wire: &TopoDS_Wire) -> Option<Pnt> {
        let mut sum = Pnt::new(0.0, 0.0, 0.0);
        let mut count = 0usize;
        for edge in &wire.edges {
            for v in &edge.vertices {
                sum.x += v.x;
                sum.y += v.y;
                sum.z += v.z;
                count += 1;
            }
        }
        if count == 0 {
            None
        } else {
            let n = count as f64;
            Some(Pnt::new(sum.x / n, sum.y / n, sum.z / n))
        }
    }

    /// Orient a wire's edges so that consecutive edges connect end-to-start.
    /// Returns the reordered edge list in a new wire, or the original wire if
    /// already connected.
    ///
    /// Mirrors `BRepTools::Orient(wire)` / `ShapeFix_Wire`.
    pub fn orient_wire(wire: &TopoDS_Wire) -> TopoDS_Wire {
        if wire.edges.is_empty() {
            return wire.clone();
        }
        let mut ordered: Vec<TopoDS_Edge> = Vec::with_capacity(wire.edges.len());
        let mut remaining: Vec<TopoDS_Edge> = wire.edges.clone();

        // Start with the first edge as-is.
        ordered.push(remaining.remove(0));

        while !remaining.is_empty() {
            let last = ordered.last().unwrap();
            let last_end = match last.vertices.last() {
                Some(v) => (v.x, v.y, v.z),
                None => break,
            };

            // Find an edge whose start or end connects to last_end.
            let found = remaining.iter().enumerate().find_map(|(i, e)| {
                let start = e.vertices.first().map(|v| (v.x, v.y, v.z));
                let end = e.vertices.last().map(|v| (v.x, v.y, v.z));
                if let Some(s) = start {
                    let d = dist3(s, last_end);
                    if d < EPS {
                        return Some((i, false));
                    }
                }
                if let Some(en) = end {
                    let d = dist3(en, last_end);
                    if d < EPS {
                        return Some((i, true));
                    }
                }
                None
            });

            match found {
                Some((idx, reversed)) => {
                    let mut edge = remaining.remove(idx);
                    if reversed {
                        edge.vertices.reverse();
                    }
                    ordered.push(edge);
                }
                None => {
                    // No connectable edge found; append remaining as-is.
                    ordered.extend(remaining.drain(..));
                    break;
                }
            }
        }

        TopoDS_Wire {
            id: wire.id,
            edges: ordered,
        }
    }
}

// ---------------------------------------------------------------------------
// BRepTools_WireExplorer
// ---------------------------------------------------------------------------

/// Ordered traversal of the edges of a wire in connected sequence.
///
/// Unlike `TopExp_Explorer` (which does an unordered DFS), `WireExplorer`
/// walks edges so that consecutive `current_edge()` calls yield geometrically
/// connected edges (end vertex of edge N = start vertex of edge N+1).
///
/// # Usage
///
/// ```ignore
/// let mut ex = WireExplorer::new();
/// ex.init(&wire_shape);
/// while ex.more() {
///     let edge = ex.current_edge();
///     // … use edge …
///     ex.next();
/// }
/// ```
///
/// Mirrors `BRepTools_WireExplorer`.
// occt: BRepTools_WireExplorer
pub struct WireExplorer {
    /// Ordered edge sequence built during `init`.
    edges: Vec<TopoDS_Edge>,
    /// Index into `edges` of the current position.
    index: usize,
}

impl WireExplorer {
    /// Create a new, uninitialized explorer.
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            index: 0,
        }
    }

    /// Initialize (or re-initialize) the explorer for `shape`.
    ///
    /// The shape must be (or contain) a `TopoDS_Wire`.  If no wire is found the
    /// explorer is left with no edges.  If multiple wires exist, all edges from
    /// all wires are chained in order.
    ///
    /// Mirrors `BRepTools_WireExplorer::Init(wire)`.
    pub fn init(&mut self, shape: &TopoDS_Shape) {
        self.edges.clear();
        self.index = 0;

        // Collect all wires from the shape.
        let wires = collect_wires(shape);
        for wire in wires {
            let oriented = BRepTools::orient_wire(&wire);
            self.edges.extend(oriented.edges);
        }
    }

    /// Initialize directly from a `TopoDS_Wire`.
    ///
    /// Mirrors `BRepTools_WireExplorer::Init(wire, face)`.
    pub fn init_wire(&mut self, wire: &TopoDS_Wire) {
        self.edges.clear();
        self.index = 0;
        let oriented = BRepTools::orient_wire(wire);
        self.edges = oriented.edges;
    }

    /// Returns `true` if there is a current edge to examine.
    ///
    /// Mirrors `BRepTools_WireExplorer::More()`.
    pub fn more(&self) -> bool {
        self.index < self.edges.len()
    }

    /// Returns a reference to the current edge.
    ///
    /// # Panics
    /// Panics if `more()` is false.
    ///
    /// Mirrors `BRepTools_WireExplorer::Current()`.
    pub fn current_edge(&self) -> &TopoDS_Edge {
        assert!(self.more(), "WireExplorer: no current edge (call more() first)");
        &self.edges[self.index]
    }

    /// Returns the current edge as a `TopoDS_Shape::Edge`.
    ///
    /// Mirrors `BRepTools_WireExplorer::Current()` returning `TopoDS_Edge`.
    pub fn current(&self) -> TopoDS_Shape {
        TopoDS_Shape::Edge(self.current_edge().clone())
    }

    /// Advance to the next edge.
    ///
    /// Mirrors `BRepTools_WireExplorer::Next()`.
    pub fn next(&mut self) {
        if self.index < self.edges.len() {
            self.index += 1;
        }
    }

    /// Return the current vertex (the start vertex of the current edge).
    ///
    /// Mirrors `BRepTools_WireExplorer::CurrentVertex()`.
    pub fn current_vertex(&self) -> Option<&TopoDS_Vertex> {
        self.current_edge().vertices.first()
    }

    /// Reset the explorer to the beginning without rebuilding the edge list.
    ///
    /// Mirrors the implicit reset that happens when `Init` is called again.
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Collect all remaining edges (from the current position onwards) into a
    /// `Vec`.  Advances the explorer to the end.
    pub fn collect_remaining(&mut self) -> Vec<TopoDS_Edge> {
        let result = self.edges[self.index..].to_vec();
        self.index = self.edges.len();
        result
    }

    /// Return the total number of edges in this wire (regardless of position).
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }
}

impl Default for WireExplorer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Recursively add bounding-box contributions of every vertex in `shape`.
fn collect_vertex_bounds(shape: &TopoDS_Shape, bbox: &mut BndBox) {
    match shape {
        TopoDS_Shape::Vertex(v) => {
            bbox.add_point(Pnt::new(v.x, v.y, v.z));
        }
        TopoDS_Shape::Edge(e) => {
            for v in &e.vertices {
                bbox.add_point(Pnt::new(v.x, v.y, v.z));
            }
        }
        TopoDS_Shape::Wire(w) => {
            for e in &w.edges {
                for v in &e.vertices {
                    bbox.add_point(Pnt::new(v.x, v.y, v.z));
                }
            }
        }
        TopoDS_Shape::Face(f) => {
            for w in &f.wires {
                for e in &w.edges {
                    for v in &e.vertices {
                        bbox.add_point(Pnt::new(v.x, v.y, v.z));
                    }
                }
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                for w in &f.wires {
                    for e in &w.edges {
                        for v in &e.vertices {
                            bbox.add_point(Pnt::new(v.x, v.y, v.z));
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Solid(s) => {
            for sh in &s.shells {
                for f in &sh.faces {
                    for w in &f.wires {
                        for e in &w.edges {
                            for v in &e.vertices {
                                bbox.add_point(Pnt::new(v.x, v.y, v.z));
                            }
                        }
                    }
                }
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for s in &cs.solids {
                let sub = TopoDS_Shape::Solid(s.clone());
                collect_vertex_bounds(&sub, bbox);
            }
        }
        TopoDS_Shape::Compound(c) => {
            for sub in &c.shapes {
                collect_vertex_bounds(sub, bbox);
            }
        }
    }
}

/// Collect all vertices reachable from `shape` into a flat list.
fn collect_vertices(shape: &TopoDS_Shape) -> Vec<TopoDS_Vertex> {
    let mut result = Vec::new();
    collect_vertices_rec(shape, &mut result);
    result
}

fn collect_vertices_rec(shape: &TopoDS_Shape, out: &mut Vec<TopoDS_Vertex>) {
    match shape {
        TopoDS_Shape::Vertex(v) => out.push(v.clone()),
        TopoDS_Shape::Edge(e) => out.extend(e.vertices.iter().cloned()),
        TopoDS_Shape::Wire(w) => {
            for e in &w.edges {
                out.extend(e.vertices.iter().cloned());
            }
        }
        TopoDS_Shape::Face(f) => {
            for w in &f.wires {
                for e in &w.edges {
                    out.extend(e.vertices.iter().cloned());
                }
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                let sub = TopoDS_Shape::Face(f.clone());
                collect_vertices_rec(&sub, out);
            }
        }
        TopoDS_Shape::Solid(s) => {
            for sh in &s.shells {
                let sub = TopoDS_Shape::Shell(sh.clone());
                collect_vertices_rec(&sub, out);
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for s in &cs.solids {
                let sub = TopoDS_Shape::Solid(s.clone());
                collect_vertices_rec(&sub, out);
            }
        }
        TopoDS_Shape::Compound(c) => {
            for sub in &c.shapes {
                collect_vertices_rec(sub, out);
            }
        }
    }
}

/// Collect all `TopoDS_Wire` objects reachable from `shape`.
fn collect_wires(shape: &TopoDS_Shape) -> Vec<TopoDS_Wire> {
    let mut out = Vec::new();
    collect_wires_rec(shape, &mut out);
    out
}

fn collect_wires_rec(shape: &TopoDS_Shape, out: &mut Vec<TopoDS_Wire>) {
    match shape {
        TopoDS_Shape::Wire(w) => out.push(w.clone()),
        TopoDS_Shape::Face(f) => out.extend(f.wires.iter().cloned()),
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                out.extend(f.wires.iter().cloned());
            }
        }
        TopoDS_Shape::Solid(s) => {
            for sh in &s.shells {
                for f in &sh.faces {
                    out.extend(f.wires.iter().cloned());
                }
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for s in &cs.solids {
                let sub = TopoDS_Shape::Solid(s.clone());
                collect_wires_rec(&sub, out);
            }
        }
        TopoDS_Shape::Compound(c) => {
            for sub in &c.shapes {
                collect_wires_rec(sub, out);
            }
        }
        _ => {}
    }
}

/// True if both endpoint vertices of an edge coincide within `tolerance`.
fn is_degenerate_edge(edge: &TopoDS_Edge, tolerance: f64) -> bool {
    match (edge.vertices.first(), edge.vertices.last()) {
        (Some(v0), Some(v1)) => {
            let dx = v0.x - v1.x;
            let dy = v0.y - v1.y;
            let dz = v0.z - v1.z;
            (dx * dx + dy * dy + dz * dz).sqrt() < tolerance
        }
        _ => true, // empty or single-vertex edge is degenerate
    }
}

/// Chord length of an edge (distance between first and last vertex).
fn edge_chord_length(edge: &TopoDS_Edge) -> f64 {
    match (edge.vertices.first(), edge.vertices.last()) {
        (Some(v0), Some(v1)) => {
            let dx = v0.x - v1.x;
            let dy = v0.y - v1.y;
            let dz = v0.z - v1.z;
            (dx * dx + dy * dy + dz * dz).sqrt()
        }
        _ => 0.0,
    }
}

/// Euclidean distance between two `(x,y,z)` tuples.
fn dist3(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::top_exp::{
        TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell, TopoDS_Solid,
        TopoDS_Vertex, TopoDS_Wire,
    };

    // ---- helpers ---------------------------------------------------------------

    fn vtx(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
        TopoDS_Vertex { id, x, y, z }
    }

    fn edge2(id: u64, v0: TopoDS_Vertex, v1: TopoDS_Vertex) -> TopoDS_Edge {
        TopoDS_Edge { id, vertices: vec![v0, v1] }
    }

    fn make_wire(id: u64, edges: Vec<TopoDS_Edge>) -> TopoDS_Wire {
        TopoDS_Wire { id, edges }
    }

    fn make_face(id: u64, wire: TopoDS_Wire) -> TopoDS_Face {
        TopoDS_Face { id, wires: vec![wire] }
    }

    fn make_shell(id: u64, faces: Vec<TopoDS_Face>) -> TopoDS_Shell {
        TopoDS_Shell { id, faces }
    }

    fn make_solid(id: u64, shell: TopoDS_Shell) -> TopoDS_Solid {
        TopoDS_Solid { id, shells: vec![shell] }
    }

    // ---- bounding box tests ----------------------------------------------------

    #[test]
    fn bounding_box_single_vertex() {
        let v = TopoDS_Shape::Vertex(vtx(1, 3.0, -2.0, 7.0));
        let bbox = BRepTools::bounding_box(&v);
        assert!(!bbox.is_void());
        let lim = bbox.get_limits();
        assert!((lim.xmin - 3.0).abs() < 1e-12);
        assert!((lim.ymin - (-2.0)).abs() < 1e-12);
        assert!((lim.zmax - 7.0).abs() < 1e-12);
    }

    #[test]
    fn bounding_box_edge_two_vertices() {
        let e = TopoDS_Shape::Edge(edge2(
            1,
            vtx(1, 0.0, 0.0, 0.0),
            vtx(2, 4.0, 3.0, 0.0),
        ));
        let bbox = BRepTools::bounding_box(&e);
        let lim = bbox.get_limits();
        assert!((lim.xmax - 4.0).abs() < 1e-12);
        assert!((lim.ymax - 3.0).abs() < 1e-12);
    }

    #[test]
    fn bounding_box_compound() {
        let e1 = TopoDS_Shape::Edge(edge2(1, vtx(1, -1.0, 0.0, 0.0), vtx(2, 0.0, 0.0, 0.0)));
        let e2 = TopoDS_Shape::Edge(edge2(2, vtx(3, 0.0, 0.0, 0.0), vtx(4, 5.0, 5.0, 5.0)));
        let c = TopoDS_Shape::Compound(TopoDS_Compound {
            id: 99,
            shapes: vec![e1, e2],
        });
        let bbox = BRepTools::bounding_box(&c);
        let lim = bbox.get_limits();
        assert!((lim.xmin - (-1.0)).abs() < 1e-12);
        assert!((lim.xmax - 5.0).abs() < 1e-12);
        assert!((lim.ymax - 5.0).abs() < 1e-12);
        assert!((lim.zmax - 5.0).abs() < 1e-12);
    }

    // ---- outer wire ------------------------------------------------------------

    #[test]
    fn outer_wire_returns_first() {
        let w1 = TopoDS_Wire { id: 1, edges: vec![] };
        let w2 = TopoDS_Wire { id: 2, edges: vec![] };
        let face = TopoDS_Face { id: 1, wires: vec![w1.clone(), w2.clone()] };
        let outer = BRepTools::outer_wire(&face).unwrap();
        assert_eq!(outer.id, 1);
    }

    #[test]
    fn outer_wire_empty_face_returns_none() {
        let face = TopoDS_Face { id: 1, wires: vec![] };
        assert!(BRepTools::outer_wire(&face).is_none());
    }

    // ---- UV bounds -------------------------------------------------------------

    #[test]
    fn uv_bounds_unit_square_wire() {
        // Wire with 4 edges forming a unit square in the XY plane.
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let x_dir = Pnt::new(1.0, 0.0, 0.0);
        let y_dir = Pnt::new(0.0, 1.0, 0.0);

        let edges = vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
            edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
            edge2(3, vtx(3, 1.0, 1.0, 0.0), vtx(4, 0.0, 1.0, 0.0)),
            edge2(4, vtx(4, 0.0, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
        ];
        let wire = make_wire(1, edges);
        let (u_min, u_max, v_min, v_max) =
            BRepTools::uv_bounds(&wire, origin, x_dir, y_dir).unwrap();
        assert!((u_min - 0.0).abs() < 1e-12);
        assert!((u_max - 1.0).abs() < 1e-12);
        assert!((v_min - 0.0).abs() < 1e-12);
        assert!((v_max - 1.0).abs() < 1e-12);
    }

    #[test]
    fn uv_bounds_empty_wire_returns_none() {
        let wire = make_wire(1, vec![]);
        let result = BRepTools::uv_bounds(
            &wire,
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        );
        assert!(result.is_none());
    }

    // ---- compare ---------------------------------------------------------------

    #[test]
    fn compare_identical_shapes() {
        let e = TopoDS_Shape::Edge(edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)));
        assert!(BRepTools::compare(&e, &e, 1e-7));
    }

    #[test]
    fn compare_different_shapes() {
        let e1 = TopoDS_Shape::Edge(edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)));
        let e2 = TopoDS_Shape::Edge(edge2(2, vtx(3, 0.0, 0.0, 0.0), vtx(4, 2.0, 0.0, 0.0)));
        // e2 has one vertex at (2,0,0) while e1 has one at (1,0,0) — not equal.
        assert!(!BRepTools::compare(&e1, &e2, 1e-7));
    }

    // ---- nb_edges, nb_vertices -------------------------------------------------

    #[test]
    fn nb_edges_triangle_wire() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 0.5, 1.0, 0.0)),
                edge2(3, vtx(3, 0.5, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        assert_eq!(BRepTools::nb_edges(&wire), 3);
    }

    #[test]
    fn nb_vertices_with_sharing() {
        // Triangle: 3 edges sharing 3 vertices (each appears twice because
        // it's the end of one edge and start of the next).
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 0.5, 1.0, 0.0)),
                edge2(3, vtx(3, 0.5, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        // 3 distinct vertex ids: 1, 2, 3
        assert_eq!(BRepTools::nb_vertices(&wire), 3);
    }

    // ---- is_closed -------------------------------------------------------------

    #[test]
    fn is_closed_triangle() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 0.5, 1.0, 0.0)),
                edge2(3, vtx(3, 0.5, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        assert!(BRepTools::is_closed(&wire, 1e-7));
    }

    #[test]
    fn is_closed_open_chain() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 2.0, 0.0, 0.0)),
            ],
        );
        assert!(!BRepTools::is_closed(&wire, 1e-7));
    }

    // ---- clean_wire ------------------------------------------------------------

    #[test]
    fn clean_wire_removes_degenerate_edge() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                // degenerate: both vertices at same location
                edge2(2, vtx(3, 2.0, 0.0, 0.0), vtx(4, 2.0, 0.0, 0.0)),
                edge2(3, vtx(5, 3.0, 0.0, 0.0), vtx(6, 4.0, 0.0, 0.0)),
            ],
        );
        let cleaned = BRepTools::clean_wire(&wire, 1e-7);
        assert_eq!(cleaned.edges.len(), 2);
    }

    // ---- wire_perimeter --------------------------------------------------------

    #[test]
    fn wire_perimeter_unit_square() {
        // A unit square: 4 edges each of length 1.
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
                edge2(3, vtx(3, 1.0, 1.0, 0.0), vtx(4, 0.0, 1.0, 0.0)),
                edge2(4, vtx(4, 0.0, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        let perimeter = BRepTools::wire_perimeter(&wire);
        assert!((perimeter - 4.0).abs() < 1e-10);
    }

    // ---- wire_centroid ---------------------------------------------------------

    #[test]
    fn wire_centroid_symmetric_wire() {
        // Wire forming a square; centroid should be at (0.5, 0.5, 0).
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
                edge2(3, vtx(3, 1.0, 1.0, 0.0), vtx(4, 0.0, 1.0, 0.0)),
                edge2(4, vtx(4, 0.0, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        let c = BRepTools::wire_centroid(&wire).unwrap();
        assert!((c.x - 0.5).abs() < 1e-10, "cx={}", c.x);
        assert!((c.y - 0.5).abs() < 1e-10, "cy={}", c.y);
        assert!(c.z.abs() < 1e-10, "cz={}", c.z);
    }

    // ---- orient_wire -----------------------------------------------------------

    #[test]
    fn orient_wire_already_connected() {
        // Edges already connected end-to-start.
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
            ],
        );
        let oriented = BRepTools::orient_wire(&wire);
        // Should have the same 2 edges in order.
        assert_eq!(oriented.edges.len(), 2);
        // First edge still starts at (0,0,0).
        let v = &oriented.edges[0].vertices[0];
        assert!((v.x - 0.0).abs() < 1e-12);
    }

    // ---- WireExplorer ----------------------------------------------------------

    #[test]
    fn wire_explorer_traverses_all_edges() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
                edge2(3, vtx(3, 1.0, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        let mut ex = WireExplorer::new();
        ex.init_wire(&wire);

        let mut count = 0;
        while ex.more() {
            let _edge = ex.current_edge();
            count += 1;
            ex.next();
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn wire_explorer_empty_wire() {
        let wire = make_wire(1, vec![]);
        let mut ex = WireExplorer::new();
        ex.init_wire(&wire);
        assert!(!ex.more());
    }

    #[test]
    fn wire_explorer_nb_edges() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 2.0, 0.0, 0.0)),
            ],
        );
        let mut ex = WireExplorer::new();
        ex.init_wire(&wire);
        assert_eq!(ex.nb_edges(), 2);
    }

    #[test]
    fn wire_explorer_current_vertex() {
        let wire = make_wire(
            1,
            vec![edge2(1, vtx(42, 7.0, 8.0, 9.0), vtx(43, 10.0, 11.0, 12.0))],
        );
        let mut ex = WireExplorer::new();
        ex.init_wire(&wire);
        assert!(ex.more());
        let v = ex.current_vertex().unwrap();
        assert_eq!(v.id, 42);
        assert!((v.x - 7.0).abs() < 1e-12);
    }

    #[test]
    fn wire_explorer_reset() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
            ],
        );
        let mut ex = WireExplorer::new();
        ex.init_wire(&wire);
        // Exhaust.
        while ex.more() {
            ex.next();
        }
        assert!(!ex.more());
        ex.reset();
        assert!(ex.more());
        assert_eq!(ex.nb_edges(), 2);
    }

    #[test]
    fn wire_explorer_collect_remaining() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 2.0, 0.0, 0.0)),
                edge2(3, vtx(3, 2.0, 0.0, 0.0), vtx(4, 3.0, 0.0, 0.0)),
            ],
        );
        let mut ex = WireExplorer::new();
        ex.init_wire(&wire);
        // Advance past the first edge.
        ex.next();
        let rest = ex.collect_remaining();
        assert_eq!(rest.len(), 2);
        assert!(!ex.more());
    }

    #[test]
    fn wire_explorer_init_from_face_shape() {
        let wire = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 0.5, 1.0, 0.0)),
                edge2(3, vtx(3, 0.5, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        let face = TopoDS_Shape::Face(make_face(1, wire));
        let mut ex = WireExplorer::new();
        ex.init(&face);
        let rest = ex.collect_remaining();
        assert_eq!(rest.len(), 3);
    }
}
