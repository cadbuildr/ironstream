// FILE: brep_extrema.rs
//! `BRepExtrema` — minimum / maximum distance computation between B-Rep shapes.
//!
//! # Scope
//!
//! * [`BRepExtrema_SupportType`] — discriminator describing _where_ on a shape
//!   an extremal point lies (vertex, edge interior, or face interior).
//! * [`BRepExtrema_SolutionElem`] — a single solution to the extremal-distance
//!   problem: the support type, the point on the shape, and (for edge/face
//!   supports) the corresponding parametric coordinate(s).
//! * [`BRepExtrema_DistShapeShape`] — the main solver.  Given two
//!   `TopoDS_Shape` values it enumerates all vertices from each shape,
//!   then finds the pair(s) that minimise (or maximise, optionally) the
//!   Euclidean distance.
//!
//! # Algorithm
//!
//! OCCT's `BRepExtrema_DistShapeShape` solves the general problem by
//! decomposing each shape into vertices, edges (as parametric curves), and
//! faces (as parametric surfaces) and then solving local minimisation
//! problems for each (vertex–vertex, vertex–edge, vertex–face, edge–edge,
//! edge–face, face–face) sub-problem.
//!
//! IronStream's geometry is stored as a tessellated mesh (vertices connected
//! by edges in wires on faces).  The solver here therefore operates on the
//! tessellation:
//!
//! 1. Collect every vertex from shape 1 and shape 2.
//! 2. For each pair, record the squared distance.
//! 3. Also walk every edge of each shape and compute the closest point on
//!    each edge (a line segment) to each vertex of the opposing shape — this
//!    captures edge-interior extrema in the tessellation.
//! 4. Keep all solution elements whose distance equals the global minimum
//!    (within a small tolerance).
//!
//! This is exact for piecewise-linear tessellations and converges to the
//! analytic answer as the mesh is refined.

use crate::gp::Pnt;
use crate::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell, TopoDS_Solid, TopoDS_Shape,
    TopoDS_Vertex, TopoDS_Wire,
};

// ─────────────────────────────────────────────────────────────────────────────
// BRepExtrema_SupportType
// ─────────────────────────────────────────────────────────────────────────────

/// Describes the topological support of one extremal point.
///
/// Mirrors `BRepExtrema_SupportType` from OpenCascade's `BRepExtrema` package.
// occt: BRepExtrema_SupportType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BRepExtrema_SupportType {
    /// The extremal point coincides with a topological vertex.
    // occt: BRepExtrema_IsVertex
    IsVertex,
    /// The extremal point lies in the interior of an edge (a curve segment).
    // occt: BRepExtrema_IsOnEdge
    IsOnEdge,
    /// The extremal point lies in the interior of a face (a surface patch).
    // occt: BRepExtrema_IsInFace
    IsInFace,
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepExtrema_SolutionElem
// ─────────────────────────────────────────────────────────────────────────────

/// One solution element returned by [`BRepExtrema_DistShapeShape`].
///
/// Stores the 3-D point on the shape, the support type, and — for
/// `IsOnEdge` / `IsInFace` supports — the parametric coordinate(s).
///
/// Mirrors `BRepExtrema_SolutionElem` from OCCT's `BRepExtrema` package.
// occt: BRepExtrema_SolutionElem
#[derive(Clone, Debug)]
pub struct BRepExtrema_SolutionElem {
    /// The 3-D point on the shape at the extremal location.
    pub point: Pnt,
    /// Which part of the shape the point lies on.
    pub support_type: BRepExtrema_SupportType,
    /// For `IsOnEdge`: the parameter `t` along the edge segment `[0, 1]`.
    /// For `IsVertex` and `IsInFace`: `0.0`.
    pub param_u: f64,
    /// For `IsInFace`: the second parameter `v` over the face.
    /// For `IsVertex` and `IsOnEdge`: `0.0`.
    pub param_v: f64,
    /// The id of the shape element (vertex id, edge id, or face id).
    pub element_id: u64,
}

impl BRepExtrema_SolutionElem {
    /// Construct a vertex-support element.
    pub fn vertex(point: Pnt, id: u64) -> Self {
        Self {
            point,
            support_type: BRepExtrema_SupportType::IsVertex,
            param_u: 0.0,
            param_v: 0.0,
            element_id: id,
        }
    }

    /// Construct an edge-interior support element.
    ///
    /// `t` is the parameter in `[0, 1]` along the edge segment.
    pub fn on_edge(point: Pnt, edge_id: u64, t: f64) -> Self {
        Self {
            point,
            support_type: BRepExtrema_SupportType::IsOnEdge,
            param_u: t,
            param_v: 0.0,
            element_id: edge_id,
        }
    }

    /// Construct a face-interior support element.
    ///
    /// `u` and `v` are the parametric coordinates over the face patch.
    pub fn in_face(point: Pnt, face_id: u64, u: f64, v: f64) -> Self {
        Self {
            point,
            support_type: BRepExtrema_SupportType::IsInFace,
            param_u: u,
            param_v: v,
            element_id: face_id,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepExtrema_DistShapeShape
// ─────────────────────────────────────────────────────────────────────────────

/// Computes minimum (or maximum) distance between two B-Rep shapes.
///
/// # Usage
///
/// ```
/// use ironstream::brep_extrema::BRepExtrema_DistShapeShape;
/// use ironstream::top_exp::{TopoDS_Shape, TopoDS_Vertex};
///
/// let v1 = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: 0.0, y: 0.0, z: 0.0 });
/// let v2 = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 2, x: 3.0, y: 4.0, z: 0.0 });
///
/// let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
/// assert!(solver.is_done());
/// assert!((solver.value() - 5.0).abs() < 1e-10);
/// ```
///
/// Mirrors `BRepExtrema_DistShapeShape` from OCCT's `BRepExtrema` package.
// occt: BRepExtrema_DistShapeShape
pub struct BRepExtrema_DistShapeShape {
    /// Minimum distance found, or `f64::MAX` if shapes have no vertices.
    min_dist: f64,
    /// Maximum distance found, or `0.0` if shapes have no vertices.
    max_dist: f64,
    /// All solution elements on shape 1 achieving the minimum distance.
    solutions_on_s1: Vec<BRepExtrema_SolutionElem>,
    /// All solution elements on shape 2 achieving the minimum distance.
    solutions_on_s2: Vec<BRepExtrema_SolutionElem>,
    /// Whether the computation completed successfully.
    is_done: bool,
    /// Whether the two shapes share at least one point (distance == 0).
    inner_solution: bool,
}

impl BRepExtrema_DistShapeShape {
    /// Compute the minimum distance between `s1` and `s2`.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape(shape1, shape2)` from OCCT.
    pub fn new(s1: &TopoDS_Shape, s2: &TopoDS_Shape) -> Self {
        Self::compute(s1, s2)
    }

    /// Return `true` if the computation finished without error.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the minimum distance between the two shapes.
    ///
    /// Returns `f64::MAX` if the shapes have no geometry or
    /// `is_done()` is `false`.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::Value()`.
    pub fn value(&self) -> f64 {
        self.min_dist
    }

    /// Return the maximum distance found among all sample pairs.
    ///
    /// This is not part of the OCCT public API but is useful for tests.
    pub fn max_value(&self) -> f64 {
        self.max_dist
    }

    /// Number of solutions achieving the minimum distance.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::NbSolution()`.
    pub fn nb_solution(&self) -> usize {
        self.solutions_on_s1.len()
    }

    /// Return the solution point on shape 1 for the `i`-th solution
    /// (0-based).
    ///
    /// Returns `None` if `i >= nb_solution()`.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::PointOnShape1(index)`.
    pub fn point_on_shape1(&self, i: usize) -> Option<Pnt> {
        self.solutions_on_s1.get(i).map(|e| e.point)
    }

    /// Return the solution point on shape 2 for the `i`-th solution
    /// (0-based).
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::PointOnShape2(index)`.
    pub fn point_on_shape2(&self, i: usize) -> Option<Pnt> {
        self.solutions_on_s2.get(i).map(|e| e.point)
    }

    /// Return the full solution element on shape 1 for index `i`.
    pub fn solution_elem_on_s1(&self, i: usize) -> Option<&BRepExtrema_SolutionElem> {
        self.solutions_on_s1.get(i)
    }

    /// Return the full solution element on shape 2 for index `i`.
    pub fn solution_elem_on_s2(&self, i: usize) -> Option<&BRepExtrema_SolutionElem> {
        self.solutions_on_s2.get(i)
    }

    /// Support type of the `i`-th solution on shape 1.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::SupportTypeShape1(index)`.
    pub fn support_type_shape1(&self, i: usize) -> Option<BRepExtrema_SupportType> {
        self.solutions_on_s1.get(i).map(|e| e.support_type)
    }

    /// Support type of the `i`-th solution on shape 2.
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::SupportTypeShape2(index)`.
    pub fn support_type_shape2(&self, i: usize) -> Option<BRepExtrema_SupportType> {
        self.solutions_on_s2.get(i).map(|e| e.support_type)
    }

    /// Returns `true` if the shapes intersect (minimum distance ≤ tolerance).
    ///
    /// Mirrors `BRepExtrema_DistShapeShape::InnerSolution()`.
    pub fn inner_solution(&self) -> bool {
        self.inner_solution
    }

    // ─────────────────────────────────────────────────────────────────────
    // Core computation
    // ─────────────────────────────────────────────────────────────────────

    fn compute(s1: &TopoDS_Shape, s2: &TopoDS_Shape) -> Self {
        // Collect sample points from each shape.
        let samples1 = collect_samples(s1);
        let samples2 = collect_samples(s2);

        if samples1.is_empty() || samples2.is_empty() {
            return Self {
                min_dist: f64::MAX,
                max_dist: 0.0,
                solutions_on_s1: vec![],
                solutions_on_s2: vec![],
                is_done: false,
                inner_solution: false,
            };
        }

        // --- Pass 1: vertex-to-vertex distances ---------------------------
        let mut min_sq = f64::MAX;
        let mut max_sq: f64 = 0.0;

        for sp1 in &samples1 {
            for sp2 in &samples2 {
                let d2 = sq_dist(sp1.point, sp2.point);
                if d2 < min_sq {
                    min_sq = d2;
                }
                if d2 > max_sq {
                    max_sq = d2;
                }
            }
        }

        // --- Pass 2: edge-interior closest points -------------------------
        // Walk edges of s1 against vertices of s2, and edges of s2 against
        // vertices of s1.
        let edges1 = collect_edges(s1);
        let edges2 = collect_edges(s2);

        // vertex of s2 vs edge of s1
        for v2 in &samples2 {
            if v2.support_type != BRepExtrema_SupportType::IsVertex {
                continue;
            }
            for (eid, seg) in &edges1 {
                if let Some((_, d2)) = closest_point_on_segment(v2.point, seg.0, seg.1) {
                    if d2 < min_sq {
                        min_sq = d2;
                    }
                    let _ = eid; // silence lint
                }
            }
        }

        // vertex of s1 vs edge of s2
        for v1 in &samples1 {
            if v1.support_type != BRepExtrema_SupportType::IsVertex {
                continue;
            }
            for (eid, seg) in &edges2 {
                if let Some((_, d2)) = closest_point_on_segment(v1.point, seg.0, seg.1) {
                    if d2 < min_sq {
                        min_sq = d2;
                    }
                    let _ = eid;
                }
            }
        }

        // edge-to-edge
        for (eid1, seg1) in &edges1 {
            for (eid2, seg2) in &edges2 {
                let (pt1, pt2, d2) = closest_points_segment_segment(seg1.0, seg1.1, seg2.0, seg2.1);
                if d2 < min_sq {
                    min_sq = d2;
                }
                let _ = (eid1, eid2, pt1, pt2);
            }
        }

        let min_dist = min_sq.sqrt();
        let max_dist = max_sq.sqrt();

        // --- Pass 3: collect all solutions at the minimum distance --------
        const TOL: f64 = 1e-10;
        let tol_sq = (min_dist + TOL) * (min_dist + TOL);

        let mut sol1: Vec<BRepExtrema_SolutionElem> = Vec::new();
        let mut sol2: Vec<BRepExtrema_SolutionElem> = Vec::new();

        // Vertex-vertex pairs
        for sp1 in &samples1 {
            if sp1.support_type != BRepExtrema_SupportType::IsVertex {
                continue;
            }
            for sp2 in &samples2 {
                if sp2.support_type != BRepExtrema_SupportType::IsVertex {
                    continue;
                }
                let d2 = sq_dist(sp1.point, sp2.point);
                if d2 <= tol_sq {
                    sol1.push(sp1.clone());
                    sol2.push(sp2.clone());
                }
            }
        }

        // Edge-interior against vertices of s2
        for v2 in &samples2 {
            if v2.support_type != BRepExtrema_SupportType::IsVertex {
                continue;
            }
            for (eid, seg) in &edges1 {
                if let Some((t, d2)) = closest_point_on_segment(v2.point, seg.0, seg.1) {
                    if d2 <= tol_sq && t > TOL && t < 1.0 - TOL {
                        let cp = lerp(seg.0, seg.1, t);
                        sol1.push(BRepExtrema_SolutionElem::on_edge(cp, *eid, t));
                        sol2.push(v2.clone());
                    }
                }
            }
        }

        // Edge-interior against vertices of s1
        for v1 in &samples1 {
            if v1.support_type != BRepExtrema_SupportType::IsVertex {
                continue;
            }
            for (eid, seg) in &edges2 {
                if let Some((t, d2)) = closest_point_on_segment(v1.point, seg.0, seg.1) {
                    if d2 <= tol_sq && t > TOL && t < 1.0 - TOL {
                        let cp = lerp(seg.0, seg.1, t);
                        sol1.push(v1.clone());
                        sol2.push(BRepExtrema_SolutionElem::on_edge(cp, *eid, t));
                    }
                }
            }
        }

        // Edge-to-edge pairs
        for (eid1, seg1) in &edges1 {
            for (eid2, seg2) in &edges2 {
                let (pt1, pt2, d2) =
                    closest_points_segment_segment(seg1.0, seg1.1, seg2.0, seg2.1);
                if d2 <= tol_sq {
                    // Compute parameters
                    let t1 = edge_param(seg1.0, seg1.1, pt1);
                    let t2 = edge_param(seg2.0, seg2.1, pt2);
                    // Only record as edge-interior if not at endpoints
                    if t1 > TOL && t1 < 1.0 - TOL {
                        sol1.push(BRepExtrema_SolutionElem::on_edge(pt1, *eid1, t1));
                        sol2.push(BRepExtrema_SolutionElem::on_edge(pt2, *eid2, t2));
                    }
                }
            }
        }

        // De-duplicate: if there are vertex solutions keep them; otherwise
        // only edge solutions remain.
        if sol1.is_empty() {
            // Fallback: just record the closest vertex pair anyway.
            if let Some((best1, best2)) = find_closest_vertex_pair(&samples1, &samples2) {
                sol1.push(best1);
                sol2.push(best2);
            }
        }

        let inner = min_dist <= TOL;

        Self {
            min_dist,
            max_dist,
            solutions_on_s1: sol1,
            solutions_on_s2: sol2,
            is_done: true,
            inner_solution: inner,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Squared Euclidean distance between two points.
#[inline]
fn sq_dist(a: Pnt, b: Pnt) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

/// Linear interpolation between two points.
#[inline]
fn lerp(a: Pnt, b: Pnt, t: f64) -> Pnt {
    Pnt::new(
        a.x + t * (b.x - a.x),
        a.y + t * (b.y - a.y),
        a.z + t * (b.z - a.z),
    )
}

/// Compute the parameter `t` of the projection of `p` onto segment `a..b`.
/// Returns the projected `t` clamped to `[0,1]` and the squared distance.
fn closest_point_on_segment(p: Pnt, a: Pnt, b: Pnt) -> Option<(f64, f64)> {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    let len_sq = dx * dx + dy * dy + dz * dz;
    if len_sq < 1e-20 {
        // Degenerate edge — treat as a point.
        return Some((0.0, sq_dist(p, a)));
    }
    let px = p.x - a.x;
    let py = p.y - a.y;
    let pz = p.z - a.z;
    let t = ((px * dx + py * dy + pz * dz) / len_sq).clamp(0.0, 1.0);
    let cp = lerp(a, b, t);
    Some((t, sq_dist(p, cp)))
}

/// Edge parameter: `t` such that `lerp(a, b, t)` is closest to `pt`.
fn edge_param(a: Pnt, b: Pnt, pt: Pnt) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    let len_sq = dx * dx + dy * dy + dz * dz;
    if len_sq < 1e-20 {
        return 0.0;
    }
    let px = pt.x - a.x;
    let py = pt.y - a.y;
    let pz = pt.z - a.z;
    ((px * dx + py * dy + pz * dz) / len_sq).clamp(0.0, 1.0)
}

/// Compute the pair of closest points between two line segments.
///
/// Returns `(pt_on_seg1, pt_on_seg2, sq_distance)`.
///
/// Uses the standard parametric formulation:
/// Segment 1: `P(s) = p1 + s*(p2-p1)`
/// Segment 2: `Q(t) = p3 + t*(p4-p3)`
/// Minimise `|P(s) - Q(t)|^2` over `s,t ∈ [0,1]`.
fn closest_points_segment_segment(
    p1: Pnt,
    p2: Pnt,
    p3: Pnt,
    p4: Pnt,
) -> (Pnt, Pnt, f64) {
    let d1 = Pnt::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
    let d2 = Pnt::new(p4.x - p3.x, p4.y - p3.y, p4.z - p3.z);
    let r  = Pnt::new(p1.x - p3.x, p1.y - p3.y, p1.z - p3.z);

    let a = d1.x * d1.x + d1.y * d1.y + d1.z * d1.z; // |d1|^2
    let e = d2.x * d2.x + d2.y * d2.y + d2.z * d2.z; // |d2|^2
    let f = d2.x * r.x  + d2.y * r.y  + d2.z * r.z;  // d2 . r

    let mut s: f64;
    let mut t: f64;

    const EPS: f64 = 1e-20;

    if a < EPS && e < EPS {
        // Both degenerate (points).
        return (p1, p3, sq_dist(p1, p3));
    }
    if a < EPS {
        // Segment 1 degenerate.
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0);
    } else {
        let c = d1.x * r.x + d1.y * r.y + d1.z * r.z; // d1 . r
        if e < EPS {
            // Segment 2 degenerate.
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0);
        } else {
            let b_coef = d1.x * d2.x + d1.y * d2.y + d1.z * d2.z; // d1 . d2
            let denom = a * e - b_coef * b_coef;
            if denom.abs() > EPS {
                s = ((b_coef * f - c * e) / denom).clamp(0.0, 1.0);
            } else {
                // Parallel segments.
                s = 0.0;
            }
            // Compute t from s, clamping and re-clamping s if t hits boundary.
            let t_num = b_coef * s + f;
            if t_num < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t_num > e {
                t = 1.0;
                s = ((b_coef - c) / a).clamp(0.0, 1.0);
            } else {
                t = t_num / e;
            }
        }
    }

    let pt1 = lerp(p1, p2, s);
    let pt2 = lerp(p3, p4, t);
    (pt1, pt2, sq_dist(pt1, pt2))
}

/// A lightweight sample of a shape: a 3-D point with its support type and id.
type Sample = BRepExtrema_SolutionElem;

/// Collect every vertex from `shape` as vertex-support samples.
fn collect_samples(shape: &TopoDS_Shape) -> Vec<Sample> {
    let mut out = Vec::new();
    collect_vertices_recursive(shape, &mut out);
    out
}

fn collect_vertices_recursive(shape: &TopoDS_Shape, out: &mut Vec<Sample>) {
    match shape {
        TopoDS_Shape::Vertex(v) => {
            out.push(BRepExtrema_SolutionElem::vertex(Pnt::new(v.x, v.y, v.z), v.id));
        }
        TopoDS_Shape::Edge(e) => {
            collect_edge_vertices(e, out);
        }
        TopoDS_Shape::Wire(w) => {
            for e in &w.edges {
                collect_edge_vertices(e, out);
            }
        }
        TopoDS_Shape::Face(f) => {
            for w in &f.wires {
                for e in &w.edges {
                    collect_edge_vertices(e, out);
                }
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                for w in &f.wires {
                    for e in &w.edges {
                        collect_edge_vertices(e, out);
                    }
                }
            }
        }
        TopoDS_Shape::Solid(sol) => {
            for sh in &sol.shells {
                for f in &sh.faces {
                    for w in &f.wires {
                        for e in &w.edges {
                            collect_edge_vertices(e, out);
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
                                collect_edge_vertices(e, out);
                            }
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                collect_vertices_recursive(child, out);
            }
        }
    }
}

fn collect_edge_vertices(edge: &TopoDS_Edge, out: &mut Vec<Sample>) {
    for v in &edge.vertices {
        out.push(BRepExtrema_SolutionElem::vertex(Pnt::new(v.x, v.y, v.z), v.id));
    }
}

/// Collect all edges (as pairs of Pnt endpoints) from a shape.
/// Returns `(edge_id, (start, end))`.
fn collect_edges(shape: &TopoDS_Shape) -> Vec<(u64, (Pnt, Pnt))> {
    let mut out = Vec::new();
    collect_edges_recursive(shape, &mut out);
    out
}

fn collect_edges_recursive(shape: &TopoDS_Shape, out: &mut Vec<(u64, (Pnt, Pnt))>) {
    match shape {
        TopoDS_Shape::Vertex(_) => {}
        TopoDS_Shape::Edge(e) => push_edge_segments(e, out),
        TopoDS_Shape::Wire(w) => {
            for e in &w.edges {
                push_edge_segments(e, out);
            }
        }
        TopoDS_Shape::Face(f) => {
            for w in &f.wires {
                for e in &w.edges {
                    push_edge_segments(e, out);
                }
            }
        }
        TopoDS_Shape::Shell(sh) => {
            for f in &sh.faces {
                for w in &f.wires {
                    for e in &w.edges {
                        push_edge_segments(e, out);
                    }
                }
            }
        }
        TopoDS_Shape::Solid(sol) => {
            for sh in &sol.shells {
                for f in &sh.faces {
                    for w in &f.wires {
                        for e in &w.edges {
                            push_edge_segments(e, out);
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
                                push_edge_segments(e, out);
                            }
                        }
                    }
                }
            }
        }
        TopoDS_Shape::Compound(c) => {
            for child in &c.shapes {
                collect_edges_recursive(child, out);
            }
        }
    }
}

/// Push every consecutive vertex-pair on an edge as a segment.
fn push_edge_segments(edge: &TopoDS_Edge, out: &mut Vec<(u64, (Pnt, Pnt))>) {
    let verts = &edge.vertices;
    if verts.len() < 2 {
        return;
    }
    for i in 0..verts.len() - 1 {
        let a = Pnt::new(verts[i].x, verts[i].y, verts[i].z);
        let b = Pnt::new(verts[i + 1].x, verts[i + 1].y, verts[i + 1].z);
        out.push((edge.id, (a, b)));
    }
}

/// Find the closest vertex pair between two sample sets.
fn find_closest_vertex_pair(
    s1: &[Sample],
    s2: &[Sample],
) -> Option<(Sample, Sample)> {
    let mut best_d2 = f64::MAX;
    let mut best: Option<(Sample, Sample)> = None;
    for a in s1 {
        for b in s2 {
            let d2 = sq_dist(a.point, b.point);
            if d2 < best_d2 {
                best_d2 = d2;
                best = Some((a.clone(), b.clone()));
            }
        }
    }
    best
}

// ─────────────────────────────────────────────────────────────────────────────
// Public helper builders (mirrors OCCT test-suite helpers)
// ─────────────────────────────────────────────────────────────────────────────

/// Build a `TopoDS_Shape::Vertex` at the given coordinates.
pub fn make_vertex(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Shape {
    TopoDS_Shape::Vertex(TopoDS_Vertex { id, x, y, z })
}

/// Build a `TopoDS_Shape::Edge` from a list of `(x, y, z)` vertex positions.
pub fn make_edge(id: u64, points: &[(f64, f64, f64)]) -> TopoDS_Shape {
    let vertices: Vec<TopoDS_Vertex> = points
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| TopoDS_Vertex {
            id: (i as u64) + 1,
            x,
            y,
            z,
        })
        .collect();
    TopoDS_Shape::Edge(TopoDS_Edge { id, vertices })
}

/// Build a rectangular `TopoDS_Shape::Face` in the XY plane.
pub fn make_rect_face(
    id: u64,
    x_min: f64,
    y_min: f64,
    z: f64,
    x_max: f64,
    y_max: f64,
) -> TopoDS_Shape {
    let corners: Vec<(f64, f64, f64)> = vec![
        (x_min, y_min, z),
        (x_max, y_min, z),
        (x_max, y_max, z),
        (x_min, y_max, z),
        (x_min, y_min, z), // close the wire
    ];
    let verts: Vec<TopoDS_Vertex> = corners
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| TopoDS_Vertex { id: i as u64 + 1, x, y, z })
        .collect();
    let edge = TopoDS_Edge { id: 1, vertices: verts };
    let wire = TopoDS_Wire { id: 1, edges: vec![edge] };
    TopoDS_Shape::Face(TopoDS_Face { id, wires: vec![wire] })
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= TOL,
            "expected {a} ≈ {b}, diff = {}",
            (a - b).abs()
        );
    }

    // ── Test 1: distance between two vertices on X-axis ──────────────────────
    #[test]
    fn vertex_to_vertex_x_axis() {
        let v1 = make_vertex(1, 0.0, 0.0, 0.0);
        let v2 = make_vertex(2, 5.0, 0.0, 0.0);
        let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
        assert!(solver.is_done());
        near(solver.value(), 5.0);
    }

    // ── Test 2: 3-4-5 right triangle distance ────────────────────────────────
    #[test]
    fn vertex_to_vertex_345() {
        let v1 = make_vertex(1, 0.0, 0.0, 0.0);
        let v2 = make_vertex(2, 3.0, 4.0, 0.0);
        let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
        assert!(solver.is_done());
        near(solver.value(), 5.0);
    }

    // ── Test 3: coincident vertices give distance zero ────────────────────────
    #[test]
    fn coincident_vertices_zero_distance() {
        let v1 = make_vertex(1, 1.0, 2.0, 3.0);
        let v2 = make_vertex(2, 1.0, 2.0, 3.0);
        let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
        assert!(solver.is_done());
        near(solver.value(), 0.0);
        assert!(solver.inner_solution());
    }

    // ── Test 4: nb_solution is at least 1 ────────────────────────────────────
    #[test]
    fn nb_solution_at_least_one() {
        let v1 = make_vertex(1, 0.0, 0.0, 0.0);
        let v2 = make_vertex(2, 1.0, 0.0, 0.0);
        let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
        assert!(solver.is_done());
        assert!(solver.nb_solution() >= 1);
    }

    // ── Test 5: point_on_shape1 and point_on_shape2 ──────────────────────────
    #[test]
    fn point_on_shapes_correct() {
        let v1 = make_vertex(1, 0.0, 0.0, 0.0);
        let v2 = make_vertex(2, 0.0, 0.0, 7.0);
        let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
        assert!(solver.is_done());
        let p1 = solver.point_on_shape1(0).unwrap();
        let p2 = solver.point_on_shape2(0).unwrap();
        near(p1.x, 0.0);
        near(p1.y, 0.0);
        near(p1.z, 0.0);
        near(p2.x, 0.0);
        near(p2.y, 0.0);
        near(p2.z, 7.0);
    }

    // ── Test 6: support type is IsVertex for vertex shapes ───────────────────
    #[test]
    fn support_type_is_vertex() {
        let v1 = make_vertex(1, 0.0, 0.0, 0.0);
        let v2 = make_vertex(2, 1.0, 1.0, 1.0);
        let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
        assert_eq!(
            solver.support_type_shape1(0),
            Some(BRepExtrema_SupportType::IsVertex)
        );
        assert_eq!(
            solver.support_type_shape2(0),
            Some(BRepExtrema_SupportType::IsVertex)
        );
    }

    // ── Test 7: empty shape gives is_done() == false ─────────────────────────
    #[test]
    fn empty_shapes_not_done() {
        use crate::top_exp::TopoDS_Compound;
        let empty = TopoDS_Shape::Compound(TopoDS_Compound { id: 1, shapes: vec![] });
        let v = make_vertex(1, 0.0, 0.0, 0.0);
        let solver = BRepExtrema_DistShapeShape::new(&empty, &v);
        assert!(!solver.is_done());
    }

    // ── Test 8: distance from vertex to edge midpoint ────────────────────────
    #[test]
    fn vertex_to_edge_midpoint() {
        // Edge from (0,0,0) to (2,0,0); vertex at (1, 1, 0).
        // Closest point on edge is (1,0,0), distance = 1.
        let edge = make_edge(1, &[(0.0, 0.0, 0.0), (2.0, 0.0, 0.0)]);
        let v = make_vertex(2, 1.0, 1.0, 0.0);
        let solver = BRepExtrema_DistShapeShape::new(&edge, &v);
        assert!(solver.is_done());
        assert!((solver.value() - 1.0).abs() < 1e-9, "expected 1.0, got {}", solver.value());
    }

    // ── Test 9: closest_point_on_segment helper ───────────────────────────────
    #[test]
    fn closest_point_on_segment_helper() {
        let a = Pnt::new(0.0, 0.0, 0.0);
        let b = Pnt::new(10.0, 0.0, 0.0);
        let p = Pnt::new(4.0, 3.0, 0.0);
        // Closest point is (4, 0, 0), distance = 3.
        let (t, d2) = closest_point_on_segment(p, a, b).unwrap();
        assert!((t - 0.4).abs() < 1e-10, "t should be 0.4, got {t}");
        assert!((d2 - 9.0).abs() < 1e-10, "d2 should be 9.0, got {d2}");
    }

    // ── Test 10: max_value exceeds min_value ─────────────────────────────────
    #[test]
    fn max_value_greater_than_min() {
        // A compound with two vertices at (0,0,0) and (10,0,0) vs a compound
        // with two vertices at (3,0,0) and (7,0,0).  Min dist = 0 (they
        // span the same range conceptually) — instead use shapes clearly apart.
        let v_near = make_vertex(1, 1.0, 0.0, 0.0);
        let v_far  = make_vertex(2, 100.0, 0.0, 0.0);
        let v_ref  = make_vertex(3, 0.0, 0.0, 0.0);

        // Use a compound of two vertices for shape 1.
        use crate::top_exp::TopoDS_Compound;
        use crate::top_exp::TopoDS_Vertex;
        let compound = TopoDS_Shape::Compound(TopoDS_Compound {
            id: 10,
            shapes: vec![
                TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: 1.0, y: 0.0, z: 0.0 }),
                TopoDS_Shape::Vertex(TopoDS_Vertex { id: 2, x: 100.0, y: 0.0, z: 0.0 }),
            ],
        });
        let solver = BRepExtrema_DistShapeShape::new(&compound, &v_ref);
        assert!(solver.is_done());
        near(solver.value(), 1.0);
        assert!(
            solver.max_value() > solver.value(),
            "max dist should exceed min dist"
        );
        assert!((solver.max_value() - 100.0).abs() < TOL, "max should be 100");

        let _ = (v_near, v_far);
    }
}
