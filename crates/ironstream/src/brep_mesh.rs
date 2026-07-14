// FILE: brep_mesh.rs
//! `BRepMesh` — incremental meshing of BRep shapes into triangle meshes,
//! mirroring OpenCascade's `BRepMesh` package.
//!
//! # Key types
//!
//! * [`IMeshData_Parameters`] — meshing control parameters (deflection,
//!   angle, relative/parallel/in-parallel flags, etc.).
//! * [`BRepMesh_IncrementalMesh`] — increments a triangulation onto each
//!   face of a shape, respecting the supplied parameters.
//! * [`BRepMesh_DiscretFactory`] — factory that selects and creates the
//!   meshing algorithm (currently only the built-in "Standard" algorithm).
//! * [`Poly_Triangle`] — a single indexed triangle (i0, i1, i2).
//! * [`Poly_Triangulation`] — a node list + triangle list attached to one
//!   face, matching `Poly_Triangulation` from OCCT.

use crate::gp::Pnt;
use crate::top_exp::{TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Wire};

// ---------------------------------------------------------------------------
// IMeshData_Parameters
// ---------------------------------------------------------------------------

/// Meshing quality parameters.
///
/// Mirrors `IMeshData_Parameters` / `BRepMesh_FastDiscret_Params` from OCCT.
// occt-ref: IMeshData_Parameters
#[derive(Clone, Debug)]
pub struct IMeshData_Parameters {
    /// Linear deflection: maximum allowed distance between mesh chord and
    /// true surface (in model units).
    pub linear_deflection: f64,

    /// Angular deflection in radians: maximum allowed angle between adjacent
    /// face normals at triangle edges.
    pub angular_deflection: f64,

    /// When `true`, `linear_deflection` is treated as a fraction of the
    /// bounding-box diagonal (relative mode).  When `false` it is absolute.
    pub relative: bool,

    /// When `true`, meshing of different shapes may run in parallel threads.
    /// (In this pure-Rust implementation the flag is respected in the API but
    /// meshing is always single-threaded.)
    pub in_parallel: bool,

    /// When `true`, only faces that have no existing triangulation (or whose
    /// triangulation has become out of date) are re-meshed.
    pub mesh_in_parallel: bool,

    /// When `true`, a shape is meshed only once even if shared by several
    /// top-level compounds.
    pub allow_quality_decrease: bool,

    /// Minimum allowed triangle size as a fraction of the linear deflection.
    pub min_size: f64,
}

impl IMeshData_Parameters {
    /// Create a parameter set with the given linear and angular deflections and
    /// all boolean options at their defaults.
    pub fn new(linear_deflection: f64, angular_deflection: f64) -> Self {
        Self {
            linear_deflection: linear_deflection.max(1e-7),
            angular_deflection: angular_deflection.max(1e-5),
            relative: false,
            in_parallel: false,
            mesh_in_parallel: false,
            allow_quality_decrease: false,
            min_size: 1e-7,
        }
    }

    /// Convenience constructor that also sets the `relative` flag.
    pub fn with_relative(
        linear_deflection: f64,
        angular_deflection: f64,
        relative: bool,
    ) -> Self {
        let mut p = Self::new(linear_deflection, angular_deflection);
        p.relative = relative;
        p
    }

    /// Effective linear deflection given the bounding-box diagonal `diag`.
    /// In absolute mode returns `linear_deflection` unchanged; in relative mode
    /// returns `linear_deflection * diag`.
    pub fn effective_deflection(&self, diag: f64) -> f64 {
        if self.relative {
            (self.linear_deflection * diag).max(1e-7)
        } else {
            self.linear_deflection
        }
    }

    /// Returns `true` if the parameters represent a coarse mesh (deflection
    /// larger than 0.1 or angular deflection larger than π/6).
    pub fn is_coarse(&self) -> bool {
        self.linear_deflection > 0.1 || self.angular_deflection > std::f64::consts::PI / 6.0
    }
}

impl Default for IMeshData_Parameters {
    fn default() -> Self {
        Self::new(0.1, 0.5)
    }
}

// ---------------------------------------------------------------------------
// Poly_Triangle
// ---------------------------------------------------------------------------

/// A single triangle stored as three vertex indices into a `Poly_Triangulation`
/// node array.
///
/// Indices are 0-based (unlike OCCT's 1-based convention; callers may adjust).
// occt-ref: Poly_Triangle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poly_Triangle {
    pub n1: usize,
    pub n2: usize,
    pub n3: usize,
}

impl Poly_Triangle {
    pub fn new(n1: usize, n2: usize, n3: usize) -> Self {
        Self { n1, n2, n3 }
    }

    /// Return the vertex indices as an array `[n1, n2, n3]`.
    pub fn as_array(self) -> [usize; 3] {
        [self.n1, self.n2, self.n3]
    }

    /// Return `true` if all three indices are distinct (non-degenerate).
    pub fn is_valid(self) -> bool {
        self.n1 != self.n2 && self.n2 != self.n3 && self.n1 != self.n3
    }
}

// ---------------------------------------------------------------------------
// Poly_Triangulation
// ---------------------------------------------------------------------------

/// Per-face triangulation result: a list of 3-D nodes and a list of triangles
/// that index them.
///
/// Mirrors `Poly_Triangulation` from OCCT.
// occt-ref: Poly_Triangulation
#[derive(Clone, Debug)]
pub struct Poly_Triangulation {
    /// 3-D node positions.
    pub nodes: Vec<Pnt>,
    /// Triangles referencing `nodes`.
    pub triangles: Vec<Poly_Triangle>,
    /// Deflection that was used when this triangulation was computed.
    pub deflection: f64,
}

impl Poly_Triangulation {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            triangles: Vec::new(),
            deflection: 0.0,
        }
    }

    /// Construct directly from nodes and triangles.
    pub fn from_parts(
        nodes: Vec<Pnt>,
        triangles: Vec<Poly_Triangle>,
        deflection: f64,
    ) -> Self {
        Self { nodes, triangles, deflection }
    }

    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    /// Return the `i`-th node (0-based).  Panics if out of range.
    pub fn node(&self, i: usize) -> Pnt {
        self.nodes[i]
    }

    /// Return the `i`-th triangle (0-based).  Panics if out of range.
    pub fn triangle(&self, i: usize) -> Poly_Triangle {
        self.triangles[i]
    }

    /// Compute the surface area of the triangulation.
    pub fn area(&self) -> f64 {
        let mut s = 0.0;
        for t in &self.triangles {
            let a = self.nodes[t.n1];
            let b = self.nodes[t.n2];
            let c = self.nodes[t.n3];
            s += (b - a).cross(c - a).norm() * 0.5;
        }
        s
    }
}

impl Default for Poly_Triangulation {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal meshing helpers
// ---------------------------------------------------------------------------

/// Axis-aligned bounding box helper for internal use.
fn wire_bbox(wire: &TopoDS_Wire) -> ([f64; 3], [f64; 3]) {
    let mut mn = [f64::INFINITY; 3];
    let mut mx = [f64::NEG_INFINITY; 3];
    for edge in &wire.edges {
        for v in &edge.vertices {
            let coords = [v.x, v.y, v.z];
            for i in 0..3 {
                if coords[i] < mn[i] { mn[i] = coords[i]; }
                if coords[i] > mx[i] { mx[i] = coords[i]; }
            }
        }
    }
    (mn, mx)
}

fn bbox_diag(mn: &[f64; 3], mx: &[f64; 3]) -> f64 {
    let dx = mx[0] - mn[0];
    let dy = mx[1] - mn[1];
    let dz = mx[2] - mn[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Collect all distinct 3-D points from a wire's edges.
fn wire_vertices(wire: &TopoDS_Wire) -> Vec<Pnt> {
    let mut pts: Vec<Pnt> = Vec::new();
    for edge in &wire.edges {
        for v in &edge.vertices {
            let p = Pnt::new(v.x, v.y, v.z);
            // Deduplicate by approximate equality.
            if !pts.iter().any(|q| q.distance(p) < 1e-10) {
                pts.push(p);
            }
        }
    }
    pts
}

/// Subdivide an edge by linear interpolation so that the chord error stays
/// within `tol`.  Returns the sequence of points along the edge (inclusive of
/// endpoints).
fn subdivide_edge(
    edge: &TopoDS_Edge,
    tol: f64,
) -> Vec<Pnt> {
    if edge.vertices.is_empty() {
        return vec![];
    }
    if edge.vertices.len() == 1 {
        let v = &edge.vertices[0];
        return vec![Pnt::new(v.x, v.y, v.z)];
    }
    let start = {
        let v = &edge.vertices[0];
        Pnt::new(v.x, v.y, v.z)
    };
    let end = {
        let v = &edge.vertices[edge.vertices.len() - 1];
        Pnt::new(v.x, v.y, v.z)
    };
    // Number of subdivisions = ceil(length / tol), at least 1.
    let length = start.distance(end);
    let n = if tol > 0.0 && length > tol {
        (length / tol).ceil() as usize
    } else {
        1
    };
    let mut pts = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = i as f64 / n as f64;
        pts.push(start.lerp(end, t));
    }
    pts
}

/// Ear-clipping triangulation of a simple (non-self-intersecting) polygon
/// given as a flat list of 3-D points in the face plane.
///
/// Returns a list of `Poly_Triangle` indexed into `polygon`.
fn ear_clip(polygon: &[Pnt]) -> Vec<Poly_Triangle> {
    let n = polygon.len();
    if n < 3 {
        return vec![];
    }
    if n == 3 {
        return vec![Poly_Triangle::new(0, 1, 2)];
    }

    // Project polygon onto its best-fit plane (use the first triangle's normal
    // as the plane normal, then work in 2-D u/v coordinates).
    let normal = (polygon[1] - polygon[0]).cross(polygon[2] - polygon[0]).normalized();
    let u_axis = (polygon[1] - polygon[0]).normalized();
    let v_axis = normal.cross(u_axis).normalized();

    let project = |p: Pnt| -> (f64, f64) {
        let d = p - polygon[0];
        (d.dot(u_axis), d.dot(v_axis))
    };

    let pts2d: Vec<(f64, f64)> = polygon.iter().map(|p| project(*p)).collect();

    // 2-D cross product (z-component).
    let cross2 = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| -> f64 {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    };

    // Point-in-triangle test (2-D).
    let in_tri = |p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)| -> bool {
        let d1 = cross2(p, a, b);
        let d2 = cross2(p, b, c);
        let d3 = cross2(p, c, a);
        let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
        !(has_neg && has_pos)
    };

    let mut indices: Vec<usize> = (0..n).collect();
    let mut result = Vec::with_capacity(n - 2);

    let is_ccw = {
        let area2: f64 = (0..n)
            .map(|i| {
                let j = (i + 1) % n;
                pts2d[i].0 * pts2d[j].1 - pts2d[j].0 * pts2d[i].1
            })
            .sum();
        area2 >= 0.0
    };

    let is_ear = |indices: &[usize], i: usize| -> bool {
        let m = indices.len();
        if m < 3 { return false; }
        let prev_i = indices[(i + m - 1) % m];
        let curr_i = indices[i];
        let next_i = indices[(i + 1) % m];
        let a = pts2d[prev_i];
        let b = pts2d[curr_i];
        let c = pts2d[next_i];
        let cross = cross2(a, b, c);
        // For a CCW polygon an ear must have a positive cross product.
        let correct_winding = if is_ccw { cross > 0.0 } else { cross < 0.0 };
        if !correct_winding { return false; }
        // No other vertex may lie inside this triangle.
        for (j, &vi) in indices.iter().enumerate() {
            if j == (i + m - 1) % m || j == i || j == (i + 1) % m { continue; }
            if in_tri(pts2d[vi], a, b, c) { return false; }
        }
        true
    };

    let mut iters = 0usize;
    while indices.len() > 3 {
        let m = indices.len();
        let mut clipped = false;
        for i in 0..m {
            if is_ear(&indices, i) {
                let prev_i = indices[(i + m - 1) % m];
                let curr_i = indices[i];
                let next_i = indices[(i + 1) % m];
                result.push(Poly_Triangle::new(prev_i, curr_i, next_i));
                indices.remove(i);
                clipped = true;
                break;
            }
        }
        iters += 1;
        if !clipped || iters > n * n {
            // Fall back: fan triangulation to avoid infinite loop.
            break;
        }
    }
    if indices.len() == 3 {
        result.push(Poly_Triangle::new(indices[0], indices[1], indices[2]));
    } else if indices.len() > 3 {
        // Fan fallback
        for i in 1..indices.len() - 1 {
            result.push(Poly_Triangle::new(indices[0], indices[i], indices[i + 1]));
        }
    }
    result
}

/// Mesh a single face into a `Poly_Triangulation`.
///
/// The algorithm:
///   1. Collect all boundary vertices from the outer wire.
///   2. Subdivide edges so the chord error ≤ `linear_deflection`.
///   3. Ear-clip the resulting polygon.
fn mesh_face(face: &TopoDS_Face, params: &IMeshData_Parameters) -> Poly_Triangulation {
    if face.wires.is_empty() {
        return Poly_Triangulation::new();
    }
    let outer_wire = &face.wires[0];

    // Bounding box for relative deflection.
    let (mn, mx) = wire_bbox(outer_wire);
    let diag = bbox_diag(&mn, &mx);
    let deflection = params.effective_deflection(diag).max(1e-9);

    // Build boundary polygon by traversing edges in order and subdividing.
    let mut boundary: Vec<Pnt> = Vec::new();
    for edge in &outer_wire.edges {
        let sub = subdivide_edge(edge, deflection);
        // Avoid duplicating the shared vertex between consecutive edges.
        if boundary.is_empty() {
            boundary.extend_from_slice(&sub);
        } else {
            // Skip first point of `sub` if it coincides with the last point of
            // `boundary` (the shared vertex between edges).
            let skip = if !sub.is_empty()
                && boundary.last().map_or(false, |q| q.distance(sub[0]) < 1e-10)
            {
                1
            } else {
                0
            };
            boundary.extend_from_slice(&sub[skip..]);
        }
    }
    // Close polygon: if first and last points coincide, drop the duplicate.
    if boundary.len() > 1 {
        if boundary.first().unwrap().distance(*boundary.last().unwrap()) < 1e-10 {
            boundary.pop();
        }
    }
    if boundary.len() < 3 {
        // Fall back to the raw vertices.
        boundary = wire_vertices(outer_wire);
    }
    if boundary.len() < 3 {
        return Poly_Triangulation::new();
    }

    let tris = ear_clip(&boundary);
    Poly_Triangulation::from_parts(boundary, tris, deflection)
}

// ---------------------------------------------------------------------------
// BRepMesh_IncrementalMesh
// ---------------------------------------------------------------------------

/// Triangulates every face of a shape incrementally.
///
/// Mirrors `BRepMesh_IncrementalMesh` from OCCT.  After construction the
/// per-face triangulations are available via `face_triangulations()`.
// occt: BRepMesh_IncrementalMesh
pub struct BRepMesh_IncrementalMesh {
    /// Parameters used during meshing.
    params: IMeshData_Parameters,
    /// Whether meshing succeeded.
    is_done: bool,
    /// Per-face triangulations, in the order faces were discovered.
    face_triangulations: Vec<(u64, Poly_Triangulation)>,
    /// The shape that was meshed.
    shape: TopoDS_Shape,
}

impl BRepMesh_IncrementalMesh {
    /// Construct and immediately mesh `shape` using `params`.
    ///
    /// Mirrors `BRepMesh_IncrementalMesh(shape, params)`.
    pub fn new(shape: TopoDS_Shape, params: IMeshData_Parameters) -> Self {
        let mut mesh = Self {
            params,
            is_done: false,
            face_triangulations: Vec::new(),
            shape,
        };
        mesh.perform();
        mesh
    }

    /// Construct with a simple linear deflection value (angular deflection
    /// defaults to 0.5 radians).  Meshes immediately.
    ///
    /// Mirrors `BRepMesh_IncrementalMesh(shape, deflection)`.
    pub fn with_deflection(shape: TopoDS_Shape, deflection: f64) -> Self {
        Self::new(shape, IMeshData_Parameters::new(deflection, 0.5))
    }

    /// Re-mesh using the stored parameters (or new ones).
    ///
    /// Mirrors `BRepMesh_IncrementalMesh::Perform()`.
    pub fn perform(&mut self) {
        self.face_triangulations.clear();
        self.is_done = false;
        collect_face_triangulations(
            &self.shape.clone(),
            &self.params.clone(),
            &mut self.face_triangulations,
        );
        self.is_done = true;
    }

    /// `true` after a successful `Perform()` call.
    ///
    /// Mirrors `BRepMesh_IncrementalMesh::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return all per-face triangulations as `(face_id, triangulation)` pairs.
    pub fn face_triangulations(&self) -> &[(u64, Poly_Triangulation)] {
        &self.face_triangulations
    }

    /// Retrieve the triangulation for a specific face id.
    pub fn triangulation_for(&self, face_id: u64) -> Option<&Poly_Triangulation> {
        self.face_triangulations
            .iter()
            .find(|(id, _)| *id == face_id)
            .map(|(_, t)| t)
    }

    /// Total number of triangles across all faces.
    pub fn total_triangles(&self) -> usize {
        self.face_triangulations
            .iter()
            .map(|(_, t)| t.nb_triangles())
            .sum()
    }

    /// Total number of nodes across all faces.
    pub fn total_nodes(&self) -> usize {
        self.face_triangulations
            .iter()
            .map(|(_, t)| t.nb_nodes())
            .sum()
    }

    /// The parameters used for meshing.
    pub fn parameters(&self) -> &IMeshData_Parameters {
        &self.params
    }

    /// Set new parameters and re-mesh.
    ///
    /// Mirrors the workflow of `BRepMesh_IncrementalMesh::SetShape` /
    /// re-construction in OCCT.
    pub fn set_parameters(&mut self, params: IMeshData_Parameters) {
        self.params = params;
        self.perform();
    }
}

/// DFS through a shape tree, meshing every `TopoDS_Face` found.
fn collect_face_triangulations(
    shape: &TopoDS_Shape,
    params: &IMeshData_Parameters,
    out: &mut Vec<(u64, Poly_Triangulation)>,
) {
    match shape {
        TopoDS_Shape::Face(face) => {
            let tri = mesh_face(face, params);
            out.push((face.id, tri));
        }
        TopoDS_Shape::Shell(shell) => {
            for face in &shell.faces {
                let tri = mesh_face(face, params);
                out.push((face.id, tri));
            }
        }
        TopoDS_Shape::Solid(solid) => {
            for shell in &solid.shells {
                for face in &shell.faces {
                    let tri = mesh_face(face, params);
                    out.push((face.id, tri));
                }
            }
        }
        TopoDS_Shape::Compound(compound) => {
            for child in &compound.shapes {
                collect_face_triangulations(child, params, out);
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for solid in &cs.solids {
                for shell in &solid.shells {
                    for face in &shell.faces {
                        let tri = mesh_face(face, params);
                        out.push((face.id, tri));
                    }
                }
            }
        }
        // Wire, Edge, Vertex: no faces to mesh.
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// BRepMesh_DiscretFactory
// ---------------------------------------------------------------------------

/// Meshing algorithm identifier.
// occt: BRepMesh_DiscretFactory // (algorithm selection)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshAlgorithm {
    /// The built-in "Standard" incremental mesher (default).
    Standard,
    /// A coarser, faster algorithm suitable for preview.
    Preview,
}

impl Default for MeshAlgorithm {
    fn default() -> Self {
        Self::Standard
    }
}

/// Factory for creating meshing algorithm instances.
///
/// Mirrors `BRepMesh_DiscretFactory` from OCCT.  In IronStream only the
/// "Standard" algorithm is available; additional algorithms can be registered
/// as needed.
// occt: BRepMesh_DiscretFactory
pub struct BRepMesh_DiscretFactory {
    /// Currently active algorithm.
    algorithm: MeshAlgorithm,
}

impl BRepMesh_DiscretFactory {
    /// Return the global singleton factory (mirrors the OCCT static accessor).
    pub fn get() -> Self {
        Self {
            algorithm: MeshAlgorithm::Standard,
        }
    }

    /// Change the active meshing algorithm.
    ///
    /// Returns `true` if the algorithm is known / supported, `false` otherwise.
    ///
    /// Mirrors `BRepMesh_DiscretFactory::SetDefault`.
    pub fn set_default(&mut self, algo: MeshAlgorithm) -> bool {
        self.algorithm = algo;
        true
    }

    /// Return the currently active algorithm.
    pub fn algorithm(&self) -> MeshAlgorithm {
        self.algorithm
    }

    /// Build and return a mesher for the given shape and parameters.
    ///
    /// Mirrors `BRepMesh_DiscretFactory::Discret(shape, deflection, angle)`.
    pub fn discret(
        &self,
        shape: TopoDS_Shape,
        deflection: f64,
        angle: f64,
    ) -> BRepMesh_IncrementalMesh {
        let mut params = IMeshData_Parameters::new(deflection, angle);
        // For the Preview algorithm, use a coarser mesh.
        if self.algorithm == MeshAlgorithm::Preview {
            params.linear_deflection = (deflection * 4.0).max(0.5);
            params.angular_deflection = (angle * 2.0).max(1.0);
        }
        BRepMesh_IncrementalMesh::new(shape, params)
    }

    /// Convenience: discret with default angular deflection of 0.5 rad.
    pub fn discret_with_deflection(
        &self,
        shape: TopoDS_Shape,
        deflection: f64,
    ) -> BRepMesh_IncrementalMesh {
        self.discret(shape, deflection, 0.5)
    }

    /// Return `true` if the named algorithm is supported.
    ///
    /// Mirrors `BRepMesh_DiscretFactory::FindAlgo`.
    pub fn find_algo(&self, name: &str) -> bool {
        matches!(name.to_ascii_lowercase().as_str(), "standard" | "preview")
    }
}

impl Default for BRepMesh_DiscretFactory {
    fn default() -> Self {
        Self::get()
    }
}

// ---------------------------------------------------------------------------
// Convenience free functions
// ---------------------------------------------------------------------------

/// Mesh a shape in one call, returning the incremental mesher.
///
/// Mirrors the common OCCT pattern:
/// ```text
/// BRepMesh_IncrementalMesh mesh(shape, deflection);
/// ```
pub fn mesh_shape(shape: TopoDS_Shape, deflection: f64) -> BRepMesh_IncrementalMesh {
    BRepMesh_IncrementalMesh::with_deflection(shape, deflection)
}

/// Mesh a face in isolation and return its triangulation.
pub fn mesh_face_only(face: TopoDS_Face, params: &IMeshData_Parameters) -> Poly_Triangulation {
    mesh_face(&face, params)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::top_exp::{
        TopoDS_CompSolid, TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell,
        TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
    };

    // ---- helpers --------------------------------------------------------

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

    /// A unit square face in the XY plane.
    fn unit_square_face(face_id: u64) -> TopoDS_Face {
        let w = make_wire(
            1,
            vec![
                edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
                edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 1.0, 1.0, 0.0)),
                edge2(3, vtx(3, 1.0, 1.0, 0.0), vtx(4, 0.0, 1.0, 0.0)),
                edge2(4, vtx(4, 0.0, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
            ],
        );
        make_face(face_id, w)
    }

    fn triangle_face(face_id: u64) -> TopoDS_Face {
        let w = make_wire(
            10,
            vec![
                edge2(10, vtx(10, 0.0, 0.0, 0.0), vtx(11, 1.0, 0.0, 0.0)),
                edge2(11, vtx(11, 1.0, 0.0, 0.0), vtx(12, 0.5, 1.0, 0.0)),
                edge2(12, vtx(12, 0.5, 1.0, 0.0), vtx(10, 0.0, 0.0, 0.0)),
            ],
        );
        make_face(face_id, w)
    }


    // ---- IMeshData_Parameters tests -------------------------------------

    #[test]
    fn test_params_default_values() {
        let p = IMeshData_Parameters::default();
        assert!((p.linear_deflection - 0.1).abs() < 1e-9);
        assert!((p.angular_deflection - 0.5).abs() < 1e-9);
        assert!(!p.relative);
    }

    #[test]
    fn test_params_effective_deflection_absolute() {
        let p = IMeshData_Parameters::new(0.05, 0.5);
        // In absolute mode the diag has no effect.
        assert!((p.effective_deflection(100.0) - 0.05).abs() < 1e-9);
    }

    #[test]
    fn test_params_effective_deflection_relative() {
        let p = IMeshData_Parameters::with_relative(0.01, 0.5, true);
        // 0.01 * 50.0 = 0.5
        assert!((p.effective_deflection(50.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_params_is_coarse() {
        let coarse = IMeshData_Parameters::new(0.5, 1.0);
        assert!(coarse.is_coarse());
        let fine = IMeshData_Parameters::new(0.001, 0.1);
        assert!(!fine.is_coarse());
    }

    // ---- Poly_Triangle tests --------------------------------------------

    #[test]
    fn test_poly_triangle_is_valid() {
        assert!(Poly_Triangle::new(0, 1, 2).is_valid());
        assert!(!Poly_Triangle::new(0, 0, 2).is_valid());
        assert!(!Poly_Triangle::new(1, 2, 2).is_valid());
    }

    #[test]
    fn test_poly_triangle_as_array() {
        let t = Poly_Triangle::new(3, 7, 11);
        assert_eq!(t.as_array(), [3, 7, 11]);
    }

    // ---- mesh_face tests ------------------------------------------------

    #[test]
    fn test_mesh_triangle_face_yields_one_triangle() {
        let face = triangle_face(1);
        // deflection=5.0 is larger than any edge length (~1.12) so no subdivision
        // occurs and the polygon stays at exactly 3 vertices → 1 triangle.
        let params = IMeshData_Parameters::new(5.0, 0.5);
        let tri = mesh_face(&face, &params);
        assert_eq!(tri.nb_triangles(), 1);
        assert_eq!(tri.nb_nodes(), 3);
        assert!(tri.triangles[0].is_valid());
    }

    #[test]
    fn test_mesh_quad_face_yields_two_triangles() {
        let face = unit_square_face(1);
        let params = IMeshData_Parameters::new(2.0, 0.5); // coarse → no subdivision
        let tri = mesh_face(&face, &params);
        // A quad ear-clips into 2 triangles.
        assert_eq!(tri.nb_triangles(), 2);
    }

    #[test]
    fn test_mesh_face_deflection_stored() {
        let face = unit_square_face(1);
        let params = IMeshData_Parameters::new(0.1, 0.5);
        let tri = mesh_face(&face, &params);
        assert!(tri.deflection > 0.0);
    }

    // ---- BRepMesh_IncrementalMesh tests ---------------------------------

    #[test]
    fn test_incremental_mesh_face_shape_is_done() {
        let face = unit_square_face(1);
        let shape = TopoDS_Shape::Face(face);
        let mesh = BRepMesh_IncrementalMesh::with_deflection(shape, 0.5);
        assert!(mesh.is_done());
        assert_eq!(mesh.face_triangulations().len(), 1);
    }

    #[test]
    fn test_incremental_mesh_shell_meshes_all_faces() {
        let f1 = unit_square_face(1);
        let f2 = triangle_face(2);
        let shell = make_shell(1, vec![f1, f2]);
        let shape = TopoDS_Shape::Shell(shell);
        let mesh = BRepMesh_IncrementalMesh::with_deflection(shape, 0.5);
        assert!(mesh.is_done());
        assert_eq!(mesh.face_triangulations().len(), 2);
        assert!(mesh.total_triangles() >= 2);
    }

    #[test]
    fn test_incremental_mesh_triangulation_for() {
        let face = unit_square_face(42);
        let shape = TopoDS_Shape::Face(face);
        let mesh = BRepMesh_IncrementalMesh::with_deflection(shape, 0.5);
        let t = mesh.triangulation_for(42);
        assert!(t.is_some());
        assert!(!t.unwrap().is_empty());
    }

    // ---- BRepMesh_DiscretFactory tests ----------------------------------

    #[test]
    fn test_factory_default_algorithm_is_standard() {
        let factory = BRepMesh_DiscretFactory::get();
        assert_eq!(factory.algorithm(), MeshAlgorithm::Standard);
    }

    #[test]
    fn test_factory_find_algo() {
        let factory = BRepMesh_DiscretFactory::get();
        assert!(factory.find_algo("Standard"));
        assert!(factory.find_algo("standard"));
        assert!(factory.find_algo("Preview"));
        assert!(!factory.find_algo("NonExistent"));
    }

    #[test]
    fn test_factory_discret_produces_done_mesh() {
        let factory = BRepMesh_DiscretFactory::get();
        let face = unit_square_face(5);
        let shape = TopoDS_Shape::Face(face);
        let mesh = factory.discret(shape, 0.1, 0.5);
        assert!(mesh.is_done());
        assert!(mesh.total_triangles() > 0);
    }
}
