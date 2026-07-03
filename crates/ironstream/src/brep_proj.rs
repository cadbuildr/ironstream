// FILE: brep_proj.rs
//! `BRepProj` — projection of shapes and curves onto B-Rep faces/surfaces,
//! mirroring OpenCascade's `BRepProj` package
//! (`src/ModelingAlgorithms/TKTopAlgo/BRepProj`).
//!
//! # Covered classes
//!
//! * [`BRepProj_Projection`] — projects a wire or edge onto a shape (typically
//!   a face or a set of faces) along a direction, producing a compound of wires
//!   that represent the "shadow" projection.
//!
//! # Algorithm
//!
//! OCCT's `BRepProj_Projection` casts rays from every vertex of the input wire
//! along the projection direction until they hit a face of the target shape.
//! The collection of hit points is then reassembled into wires on the target
//! shape.
//!
//! IronStream's geometry is represented as a tessellated topology (vertices,
//! edges, wires, faces built from `TopoDS_*` types).  The implementation here:
//!
//! 1. Collects all vertices from the source wire/edge.
//! 2. For each vertex, fires a ray along `direction` and tests against every
//!    face of the target shape (each face is treated as a convex polygon fan
//!    of its vertices).
//! 3. Collects all hit points, builds a result `TopoDS_Wire` from consecutive
//!    hit-point edges, and wraps it in a `TopoDS_Compound`.
//!
//! For the *conical* variant, each ray direction is computed from the source
//! vertex to a user-supplied apex point instead of being a fixed direction.

use crate::gp::{Pnt, EPS};
use crate::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape,
    TopoDS_Vertex, TopoDS_Wire,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const TOL: f64 = 1.0e-9;

// ---------------------------------------------------------------------------
// ProjectionType
// ---------------------------------------------------------------------------

/// Selects whether the projection is along a fixed direction (cylindrical) or
/// towards a fixed apex point (conical).
///
/// Mirrors the two `BRepProj_Projection` constructors in OCCT.
// occt: BRepProj_Projection (constructor variant)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProjectionType {
    /// Project along a fixed direction vector (cylindrical projection).
    Cylindrical,
    /// Project from a fixed apex/eye point (conical projection).
    Conical,
}

// ---------------------------------------------------------------------------
// ProjectedPoint — internal record of a single ray-hit
// ---------------------------------------------------------------------------

/// A single point produced by the projection: the hit position in 3D.
/// `face_id` and `ray_t` are retained for grouping and deduplication logic.
#[derive(Clone, Debug)]
struct ProjectedPoint {
    position: Pnt,
    #[allow(dead_code)]
    face_id: u64,
    #[allow(dead_code)]
    ray_t: f64,
}

// ---------------------------------------------------------------------------
// BRepProj_Projection
// ---------------------------------------------------------------------------

/// Projects a wire (or edge, or any shape) onto a target B-Rep shape by
/// casting rays from each source vertex along a direction (cylindrical mode)
/// or towards an apex point (conical mode).
///
/// # OCCT correspondence
///
/// ```text
/// BRepProj_Projection(const TopoDS_Shape& Wire,
///                     const TopoDS_Shape& Shape,
///                     const gp_Dir&       Dir);   // cylindrical
///
/// BRepProj_Projection(const TopoDS_Shape& Wire,
///                     const TopoDS_Shape& Shape,
///                     const gp_Pnt&       P);     // conical
/// ```
///
/// The results are accumulated as `TopoDS_Wire` entries inside a
/// `TopoDS_Compound`.  Iterate via [`next`](Self::next) /
/// [`current`](Self::current) to walk the projected wires, or retrieve all at
/// once with [`wires`](Self::wires).
// occt: BRepProj_Projection
pub struct BRepProj_Projection {
    /// Projection mode (cylindrical or conical).
    proj_type: ProjectionType,
    /// Fixed direction, normalised; `Some` only for cylindrical mode.
    direction: Option<Pnt>,
    /// Apex point; `Some` only for conical mode.
    apex: Option<Pnt>,
    /// All wires found in the projection result.
    result_wires: Vec<TopoDS_Wire>,
    /// Cursor for the [`next`]/[`current`] iterator API.
    cursor: usize,
    /// Whether the computation finished without error.
    is_done: bool,
}

impl BRepProj_Projection {
    // ─────────────────────────── constructors ───────────────────────────────

    /// **Cylindrical projection**: project `source` onto `target` along `dir`.
    ///
    /// Mirrors:
    /// ```text
    /// BRepProj_Projection(wire, shape, gp_Dir dir)
    /// ```
    pub fn cylindrical(source: &TopoDS_Shape, target: &TopoDS_Shape, dir: Pnt) -> Self {
        let mut proj = Self {
            proj_type: ProjectionType::Cylindrical,
            direction: Some(dir.normalized()),
            apex: None,
            result_wires: Vec::new(),
            cursor: 0,
            is_done: false,
        };
        proj.compute(source, target);
        proj
    }

    /// **Conical projection**: project `source` onto `target` from apex `p`.
    ///
    /// Each source vertex is projected from `p` towards the vertex position.
    ///
    /// Mirrors:
    /// ```text
    /// BRepProj_Projection(wire, shape, gp_Pnt p)
    /// ```
    pub fn conical(source: &TopoDS_Shape, target: &TopoDS_Shape, apex: Pnt) -> Self {
        let mut proj = Self {
            proj_type: ProjectionType::Conical,
            direction: None,
            apex: Some(apex),
            result_wires: Vec::new(),
            cursor: 0,
            is_done: false,
        };
        proj.compute(source, target);
        proj
    }

    // ─────────────────────────── state queries ──────────────────────────────

    /// `BRepProj_Projection::IsDone` — returns `true` if the projection
    /// completed without error.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// `BRepProj_Projection::More` — returns `true` if there are more wires
    /// to iterate over (the cursor has not reached the end).
    pub fn more(&self) -> bool {
        self.cursor < self.result_wires.len()
    }

    /// `BRepProj_Projection::Next` — advance the iterator cursor by one.
    pub fn next(&mut self) {
        if self.cursor < self.result_wires.len() {
            self.cursor += 1;
        }
    }

    /// `BRepProj_Projection::Current` — return the current wire in the
    /// iteration sequence, or `None` if the cursor is past the end.
    pub fn current(&self) -> Option<&TopoDS_Wire> {
        self.result_wires.get(self.cursor)
    }

    /// Reset the iterator cursor to the beginning.
    ///
    /// This is not part of the original OCCT API but is useful for re-traversal.
    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    /// Return all projected wires as a slice (zero-copy).
    pub fn wires(&self) -> &[TopoDS_Wire] {
        &self.result_wires
    }

    /// Return the number of projected wires found.
    pub fn nb_wires(&self) -> usize {
        self.result_wires.len()
    }

    /// Return the full projection result as a `TopoDS_Compound` of wires,
    /// mirroring `BRepProj_Projection::Shape()`.
    pub fn shape(&self) -> TopoDS_Shape {
        let shapes: Vec<TopoDS_Shape> = self
            .result_wires
            .iter()
            .map(|w| TopoDS_Shape::Wire(w.clone()))
            .collect();
        TopoDS_Shape::Compound(TopoDS_Compound { id: 1, shapes })
    }

    /// `BRepProj_Projection::Ancestor` — return the index of the face (in the
    /// target shape's face list) on which wire `wire_idx` was projected, or
    /// `None` if `wire_idx` is out of range.
    ///
    /// In IronStream the face id from the target shape is embedded in the wire
    /// id field for convenient lookup.
    pub fn ancestor(&self, wire_idx: usize) -> Option<u64> {
        self.result_wires.get(wire_idx).map(|w| w.id)
    }

    // ─────────────────────────── projection type ────────────────────────────

    /// Return the projection type used.
    pub fn projection_type(&self) -> ProjectionType {
        self.proj_type
    }

    /// Return the projection direction (cylindrical mode), or `None`.
    pub fn direction(&self) -> Option<Pnt> {
        self.direction
    }

    /// Return the apex point (conical mode), or `None`.
    pub fn apex(&self) -> Option<Pnt> {
        self.apex
    }

    // ─────────────────────────── core computation ───────────────────────────

    /// Internal entry point; dispatches to cylindrical or conical logic.
    fn compute(&mut self, source: &TopoDS_Shape, target: &TopoDS_Shape) {
        // Collect source vertices.
        let source_verts = collect_vertices(source);
        if source_verts.is_empty() {
            return;
        }

        // Collect target faces as lists of triangle fans.
        let target_faces = collect_faces(target);
        if target_faces.is_empty() {
            return;
        }

        // For each source vertex, cast a ray and collect hits per face.
        // We group all hits that land on the same face into a single result wire.
        // hits: face_id → Vec<(ray_t, position, source_vertex_id)>
        let mut face_hits: std::collections::HashMap<u64, Vec<ProjectedPoint>> =
            std::collections::HashMap::new();

        for sv in &source_verts {
            let origin = Pnt::new(sv.x, sv.y, sv.z);

            // Determine ray direction for this vertex.
            let ray_dir = match self.proj_type {
                ProjectionType::Cylindrical => {
                    // direction is always Some in cylindrical mode.
                    self.direction.unwrap()
                }
                ProjectionType::Conical => {
                    let apex = self.apex.unwrap();
                    // Ray goes from apex towards the source vertex.
                    (origin - apex).normalized()
                }
            };

            if ray_dir.norm() < EPS {
                continue;
            }

            // Test the ray against every target face.
            for (face_id, tri_fans) in &target_faces {
                for tri in tri_fans {
                    if let Some((t, hit)) = ray_triangle_intersect(origin, ray_dir, tri[0], tri[1], tri[2]) {
                        if t > -TOL {
                            let entry = face_hits.entry(*face_id).or_default();
                            entry.push(ProjectedPoint {
                                position: hit,
                                face_id: *face_id,
                                ray_t: t,
                            });
                        }
                    }
                }
            }
        }

        // Build one result wire per face that received at least one hit.
        // Hits are sorted by the source vertex order (their insertion order
        // reflects the source wire traversal).
        let mut wire_id: u64 = 1;
        let mut sorted_face_ids: Vec<u64> = face_hits.keys().copied().collect();
        sorted_face_ids.sort_unstable();

        let mut edge_id_counter: u64 = 1;
        let mut vert_id_counter: u64 = 1;

        for face_id in sorted_face_ids {
            let hits = &face_hits[&face_id];
            if hits.is_empty() {
                continue;
            }

            // Deduplicate nearby hits (within TOL).
            let deduped = deduplicate_hits(hits);
            if deduped.is_empty() {
                continue;
            }

            // Build edges: consecutive projected points become edges of the wire.
            let mut edges: Vec<TopoDS_Edge> = Vec::new();
            for window in deduped.windows(2) {
                let a = &window[0];
                let b = &window[1];
                let va = TopoDS_Vertex {
                    id: vert_id_counter,
                    x: a.position.x,
                    y: a.position.y,
                    z: a.position.z,
                };
                vert_id_counter += 1;
                let vb = TopoDS_Vertex {
                    id: vert_id_counter,
                    x: b.position.x,
                    y: b.position.y,
                    z: b.position.z,
                };
                vert_id_counter += 1;
                edges.push(TopoDS_Edge {
                    id: edge_id_counter,
                    vertices: vec![va, vb],
                });
                edge_id_counter += 1;
            }

            // If only one hit point, create a degenerate single-vertex edge.
            if edges.is_empty() {
                let only = &deduped[0];
                let v = TopoDS_Vertex {
                    id: vert_id_counter,
                    x: only.position.x,
                    y: only.position.y,
                    z: only.position.z,
                };
                vert_id_counter += 1;
                edges.push(TopoDS_Edge {
                    id: edge_id_counter,
                    vertices: vec![v],
                });
                edge_id_counter += 1;
            }

            // Wire id = face id (used by ancestor()).
            self.result_wires.push(TopoDS_Wire { id: face_id, edges });
            wire_id += 1;
        }

        self.is_done = true;
        let _ = wire_id; // suppress warning
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Collect all vertices from a shape, preserving order.
fn collect_vertices(shape: &TopoDS_Shape) -> Vec<TopoDS_Vertex> {
    let mut out = Vec::new();
    collect_vertices_rec(shape, &mut out);
    out
}

fn collect_vertices_rec(shape: &TopoDS_Shape, out: &mut Vec<TopoDS_Vertex>) {
    match shape {
        TopoDS_Shape::Vertex(v) => out.push(v.clone()),
        TopoDS_Shape::Edge(e) => {
            for v in &e.vertices {
                out.push(v.clone());
            }
        }
        TopoDS_Shape::Wire(w) => {
            for e in &w.edges {
                for v in &e.vertices {
                    out.push(v.clone());
                }
            }
        }
        TopoDS_Shape::Face(f) => {
            for w in &f.wires {
                for e in &w.edges {
                    for v in &e.vertices {
                        out.push(v.clone());
                    }
                }
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                for w in &f.wires {
                    for e in &w.edges {
                        for v in &e.vertices {
                            out.push(v.clone());
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Solid(sol) => {
            for sh in &sol.shells {
                for f in &sh.faces {
                    for w in &f.wires {
                        for e in &w.edges {
                            for v in &e.vertices {
                                out.push(v.clone());
                            }
                        }
                    }
                }
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for sol in &cs.solids {
                for sh in &sol.shells {
                    for f in &sh.faces {
                        for w in &f.wires {
                            for e in &w.edges {
                                for v in &e.vertices {
                                    out.push(v.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                collect_vertices_rec(child, out);
            }
        }
    }
}

/// Collect all faces from a shape as `(face_id, triangles)`.
///
/// Each face is tessellated as a convex polygon fan: vertex 0 is the fan hub,
/// and for every consecutive triple `(0, i, i+1)` we create one triangle.
fn collect_faces(shape: &TopoDS_Shape) -> Vec<(u64, Vec<[Pnt; 3]>)> {
    let mut out = Vec::new();
    collect_faces_rec(shape, &mut out);
    out
}

fn face_to_tris(face: &TopoDS_Face) -> Vec<[Pnt; 3]> {
    // Gather all unique vertex positions from the face.
    let mut pts: Vec<Pnt> = Vec::new();
    for wire in &face.wires {
        for edge in &wire.edges {
            for v in &edge.vertices {
                let p = Pnt::new(v.x, v.y, v.z);
                // Only add if not a near-duplicate.
                if !pts.iter().any(|q| q.distance(p) < TOL) {
                    pts.push(p);
                }
            }
        }
    }
    if pts.len() < 3 {
        return Vec::new();
    }
    // Fan triangulation from pts[0].
    let mut tris = Vec::new();
    for i in 1..pts.len() - 1 {
        tris.push([pts[0], pts[i], pts[i + 1]]);
    }
    tris
}

fn collect_faces_rec(shape: &TopoDS_Shape, out: &mut Vec<(u64, Vec<[Pnt; 3]>)>) {
    match shape {
        TopoDS_Shape::Face(f) => {
            let tris = face_to_tris(f);
            if !tris.is_empty() {
                out.push((f.id, tris));
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                let tris = face_to_tris(f);
                if !tris.is_empty() {
                    out.push((f.id, tris));
                }
            }
        }
        TopoDS_Shape::Solid(sol) => {
            for sh in &sol.shells {
                for f in &sh.faces {
                    let tris = face_to_tris(f);
                    if !tris.is_empty() {
                        out.push((f.id, tris));
                    }
                }
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for sol in &cs.solids {
                for sh in &sol.shells {
                    for f in &sh.faces {
                        let tris = face_to_tris(f);
                        if !tris.is_empty() {
                            out.push((f.id, tris));
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                collect_faces_rec(child, out);
            }
        }
        // Vertices, edges, wires: no faces to extract.
        _ => {}
    }
}

/// Möller–Trumbore ray–triangle intersection.
///
/// Returns `Some((t, hit_point))` where `t` is the ray parameter (distance
/// along `dir` from `origin`) and `hit_point` is the 3D intersection point.
/// Returns `None` if the ray misses the triangle or the triangle is degenerate.
///
/// `dir` must be normalised before calling.
fn ray_triangle_intersect(
    origin: Pnt,
    dir: Pnt,
    v0: Pnt,
    v1: Pnt,
    v2: Pnt,
) -> Option<(f64, Pnt)> {
    let e1 = v1 - v0;
    let e2 = v2 - v0;

    let h = dir.cross(e2);
    let a = e1.dot(h);

    if a.abs() < TOL {
        return None; // Ray parallel to triangle.
    }

    let f = 1.0 / a;
    let s = origin - v0;
    let u = f * s.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(e1);
    let v = f * dir.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * e2.dot(q);
    let hit = origin + dir * t;
    Some((t, hit))
}

/// Remove hits that are within `TOL` of each other to avoid duplicate wire
/// vertices.
fn deduplicate_hits(hits: &[ProjectedPoint]) -> Vec<ProjectedPoint> {
    let mut out: Vec<ProjectedPoint> = Vec::new();
    for h in hits {
        if !out.iter().any(|q| q.position.distance(h.position) < TOL) {
            out.push(h.clone());
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Public builder helpers (mirror the OCCT test-suite helpers used in tests)
// ---------------------------------------------------------------------------

/// Build a `TopoDS_Shape::Wire` from a list of `(x, y, z)` positions.
///
/// Consecutive positions become edges; vertices are shared between consecutive
/// edges (by id).
pub fn make_wire_from_points(id: u64, pts: &[(f64, f64, f64)]) -> TopoDS_Shape {
    if pts.is_empty() {
        return TopoDS_Shape::Wire(TopoDS_Wire { id, edges: vec![] });
    }
    let mut edges = Vec::new();
    for (i, window) in pts.windows(2).enumerate() {
        let (ax, ay, az) = window[0];
        let (bx, by, bz) = window[1];
        let va = TopoDS_Vertex { id: (i as u64) * 2 + 1, x: ax, y: ay, z: az };
        let vb = TopoDS_Vertex { id: (i as u64) * 2 + 2, x: bx, y: by, z: bz };
        edges.push(TopoDS_Edge { id: i as u64 + 1, vertices: vec![va, vb] });
    }
    TopoDS_Shape::Wire(TopoDS_Wire { id, edges })
}

/// Build a planar `TopoDS_Shape::Face` (a quadrilateral in the XY plane) from
/// four corner coordinates.
///
/// The winding order is `corners[0] → corners[1] → corners[2] → corners[3]`,
/// closed back to `corners[0]`.
pub fn make_quad_face(
    id: u64,
    corners: [(f64, f64, f64); 4],
) -> TopoDS_Shape {
    let verts: Vec<TopoDS_Vertex> = corners
        .iter()
        .chain(std::iter::once(&corners[0])) // close the loop
        .enumerate()
        .map(|(i, &(x, y, z))| TopoDS_Vertex { id: i as u64 + 1, x, y, z })
        .collect();
    let edge = TopoDS_Edge { id: 1, vertices: verts };
    let wire = TopoDS_Wire { id: 1, edges: vec![edge] };
    TopoDS_Shape::Face(TopoDS_Face { id, wires: vec![wire] })
}

/// Build a `TopoDS_Shape::Vertex` at the given coordinates.
pub fn make_proj_vertex(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Shape {
    TopoDS_Shape::Vertex(TopoDS_Vertex { id, x, y, z })
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;

    const TOL_TEST: f64 = 1e-6;

    fn near(a: f64, b: f64, label: &str) {
        assert!(
            (a - b).abs() <= TOL_TEST,
            "{label}: expected {a} ≈ {b}, diff = {}",
            (a - b).abs()
        );
    }

    fn pnt_near(a: Pnt, b: Pnt, label: &str) {
        assert!(
            a.distance(b) <= TOL_TEST,
            "{label}: expected {:?} ≈ {:?}, dist = {}",
            a,
            b,
            a.distance(b)
        );
    }

    /// A unit square face in the XY plane (z = 0).
    fn xy_face() -> TopoDS_Shape {
        make_quad_face(
            1,
            [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)],
        )
    }

    /// A 2×2 square face in the XY plane.
    fn wide_xy_face() -> TopoDS_Shape {
        make_quad_face(
            2,
            [(0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (2.0, 2.0, 0.0), (0.0, 2.0, 0.0)],
        )
    }

    // ── Test 1: is_done after a successful cylindrical projection ─────────────
    #[test]
    fn cylindrical_projection_is_done() {
        // Wire at z = 1, projected downward (-Z) onto a face at z = 0.
        let wire = make_wire_from_points(1, &[(0.5, 0.5, 1.0), (0.5, 0.5, 1.0)]);
        let face = xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&wire, &face, dir);
        assert!(proj.is_done(), "projection should succeed");
    }

    // ── Test 2: projection onto face z=0 along -Z lands at z≈0 ───────────────
    #[test]
    fn cylindrical_projection_z_to_zero() {
        // Source vertex at (0.5, 0.5, 3.0), projected along -Z onto z=0 face.
        let src = make_proj_vertex(1, 0.5, 0.5, 3.0);
        let face = xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        assert!(proj.is_done());
        assert!(!proj.wires().is_empty(), "should have at least one result wire");

        // Every projected vertex should be at z ≈ 0.
        for wire in proj.wires() {
            for edge in &wire.edges {
                for v in &edge.vertices {
                    near(v.z, 0.0, "projected z");
                }
            }
        }
    }

    // ── Test 3: projected point (x,y) matches source (x,y) for vertical ray ──
    #[test]
    fn cylindrical_projection_xy_preserved() {
        let src = make_proj_vertex(1, 0.3, 0.7, 5.0);
        let face = wide_xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        assert!(proj.is_done());

        let wires = proj.wires();
        assert!(!wires.is_empty());
        let v = &wires[0].edges[0].vertices[0];
        near(v.x, 0.3, "x preserved");
        near(v.y, 0.7, "y preserved");
    }

    // ── Test 4: ray that misses the face gives no wires ──────────────────────
    #[test]
    fn cylindrical_miss_gives_no_wires() {
        // Vertex at (5.0, 5.0, 1.0) projected along -Z: the unit face at z=0
        // only covers [0,1]×[0,1], so the ray misses.
        let src = make_proj_vertex(1, 5.0, 5.0, 1.0);
        let face = xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        // is_done may still be true (computation ran); just no wires.
        assert_eq!(proj.nb_wires(), 0, "should hit no face");
    }

    // ── Test 5: projection_type accessor ─────────────────────────────────────
    #[test]
    fn projection_type_cylindrical() {
        let src = make_proj_vertex(1, 0.5, 0.5, 1.0);
        let face = xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        assert_eq!(proj.projection_type(), ProjectionType::Cylindrical);
        assert!(proj.direction().is_some());
        assert!(proj.apex().is_none());
    }

    // ── Test 6: conical projection — direction from apex to source vertex ─────
    #[test]
    fn conical_projection_is_done() {
        // Apex at (0.5, 0.5, 2.0); source vertex at (0.5, 0.5, 1.0).
        // Ray from apex through source continues and hits face at z=0.
        let src = make_proj_vertex(1, 0.5, 0.5, 1.0);
        let face = xy_face();
        let apex = Pnt::new(0.5, 0.5, 2.0);
        let proj = BRepProj_Projection::conical(&src, &face, apex);
        assert!(proj.is_done());
    }

    // ── Test 7: conical projection_type accessor ──────────────────────────────
    #[test]
    fn projection_type_conical() {
        let src = make_proj_vertex(1, 0.5, 0.5, 1.0);
        let face = xy_face();
        let apex = Pnt::new(0.5, 0.5, 3.0);
        let proj = BRepProj_Projection::conical(&src, &face, apex);
        assert_eq!(proj.projection_type(), ProjectionType::Conical);
        assert!(proj.apex().is_some());
        assert!(proj.direction().is_none());
        pnt_near(proj.apex().unwrap(), apex, "apex preserved");
    }

    // ── Test 8: iterator API — more/next/current ──────────────────────────────
    #[test]
    fn iterator_api_works() {
        let src = make_wire_from_points(1, &[(0.2, 0.2, 1.0), (0.8, 0.8, 1.0)]);
        let face = xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let mut proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        assert!(proj.is_done());

        let nb = proj.nb_wires();
        if nb > 0 {
            assert!(proj.more());
            let _w = proj.current().unwrap();
            proj.next();
            // After advancing past all wires, more() should be false.
            for _ in 0..nb {
                proj.next();
            }
            assert!(!proj.more());
        }
    }

    // ── Test 9: ancestor returns the face id where the wire was projected ──────
    #[test]
    fn ancestor_returns_face_id() {
        let src = make_proj_vertex(1, 0.5, 0.5, 2.0);
        let face = xy_face(); // face id = 1
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        assert!(proj.is_done());
        if proj.nb_wires() > 0 {
            let face_id = proj.ancestor(0).unwrap();
            assert_eq!(face_id, 1, "wire should be on face id 1");
        }
    }

    // ── Test 10: shape() returns a compound containing the wires ──────────────
    #[test]
    fn shape_returns_compound_of_wires() {
        let src = make_proj_vertex(1, 0.5, 0.5, 4.0);
        let face = xy_face();
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
        assert!(proj.is_done());
        let s = proj.shape();
        match s {
            TopoDS_Shape::Compound(c) => {
                assert_eq!(c.shapes.len(), proj.nb_wires(), "compound children = nb_wires");
                for child in &c.shapes {
                    assert!(
                        matches!(child, TopoDS_Shape::Wire(_)),
                        "compound children must be wires"
                    );
                }
            }
            _ => panic!("shape() must return a Compound"),
        }
    }
}
