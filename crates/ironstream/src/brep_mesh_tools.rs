// FILE: rust/ironstream/crates/ironstream/src/brep_mesh_tools.rs
//! `BRepMesh` utility types — low-level mesh primitives used by the
//! incremental mesher, mirroring OpenCascade's `BRepMesh` package helpers.
//!
//! # Key types
//!
//! * [`MeshVertex`]   — mesh vertex carrying 2-D parametric, 3-D Cartesian,
//!                       and UV location coordinates.
//! * [`MeshEdge`]     — oriented edge connecting two vertex indices, with a
//!                       free-edge flag.
//! * [`MeshFaceData`] — per-face mesh accumulator: vertices, edges, triangles,
//!                       deflection, and a "done" marker.

// ---------------------------------------------------------------------------
// MeshVertex
// ---------------------------------------------------------------------------

/// A mesh vertex holding its 2-D parametric coordinates, 3-D Cartesian
/// position, and UV location in the surface parameter space.
///
/// The three coordinate pairs are kept together so that the mesher can look up
/// any representation without extra indirection.
///
/// # Example
/// ```
/// use ironstream::brep_mesh_tools::MeshVertex;
/// let v = MeshVertex::new([0.0, 0.5], [1.0, 2.0, 3.0]);
/// assert_eq!(v.point_2d(), [0.0, 0.5]);
/// assert_eq!(v.point_3d(), [1.0, 2.0, 3.0]);
/// ```
// occt: BRepMesh_Vertex
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshVertex {
    /// 2-D parametric coordinates of this vertex on the host surface.
    pub point_2d: [f64; 2],
    /// 3-D Cartesian position of this vertex in model space.
    pub point_3d: [f64; 3],
    /// UV location in the surface parameter domain (may differ from
    /// `point_2d` when the surface has a seam or periodic boundary).
    pub uv_location: [f64; 2],
}

impl MeshVertex {
    /// Construct a new vertex from its 2-D parametric and 3-D Cartesian
    /// coordinates.  The `uv_location` defaults to the same value as
    /// `point_2d`.
    ///
    /// Mirrors `BRepMesh_Vertex::BRepMesh_Vertex(uv, 3d, movability)`.
    pub fn new(pt2d: [f64; 2], pt3d: [f64; 3]) -> Self {
        Self {
            point_2d: pt2d,
            point_3d: pt3d,
            uv_location: pt2d,
        }
    }

    /// Return the 2-D parametric coordinates.
    pub fn point_2d(&self) -> [f64; 2] {
        self.point_2d
    }

    /// Return the 3-D Cartesian position.
    pub fn point_3d(&self) -> [f64; 3] {
        self.point_3d
    }

    /// Return the UV location in the surface parameter domain.
    pub fn uv_location(&self) -> [f64; 2] {
        self.uv_location
    }
}

// ---------------------------------------------------------------------------
// MeshEdge
// ---------------------------------------------------------------------------

/// A mesh edge connecting two vertex indices in a `MeshFaceData` vertex array.
///
/// The `is_free` flag indicates that the edge is on the boundary of the
/// triangulated face (i.e. not shared by two triangles).
///
/// # Example
/// ```
/// use ironstream::brep_mesh_tools::{MeshEdge, MeshVertex};
/// let v0 = MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]);
/// let v1 = MeshVertex::new([1.0, 0.0], [1.0, 0.0, 0.0]);
/// let vertices = [v0, v1];
/// let mut e = MeshEdge::new(0, 1);
/// assert!((e.length(&vertices) - 1.0).abs() < 1e-12);
/// e.set_free(false);
/// assert!(!e.is_free());
/// ```
// occt: BRepMesh_Edge
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshEdge {
    /// Index of the first endpoint vertex in the owning face's vertex array.
    pub v0: usize,
    /// Index of the second endpoint vertex in the owning face's vertex array.
    pub v1: usize,
    /// `true` if this edge is on the boundary of the mesh (not interior).
    pub is_free: bool,
}

impl MeshEdge {
    /// Construct a new edge between vertices `v0` and `v1`.
    /// The edge starts as a free (boundary) edge.
    ///
    /// Mirrors `BRepMesh_Edge::BRepMesh_Edge(v0, v1, movability)`.
    pub fn new(v0: usize, v1: usize) -> Self {
        Self { v0, v1, is_free: true }
    }

    /// Return the index of the first vertex endpoint.
    pub fn v0(&self) -> usize {
        self.v0
    }

    /// Return the index of the second vertex endpoint.
    pub fn v1(&self) -> usize {
        self.v1
    }

    /// Return `true` if this edge lies on the mesh boundary.
    pub fn is_free(&self) -> bool {
        self.is_free
    }

    /// Set the free-edge flag.
    ///
    /// Mirrors toggling `BRepMesh_DegreeOfFreedom` movability in OCCT.
    pub fn set_free(&mut self, free: bool) {
        self.is_free = free;
    }

    /// Compute the 3-D Euclidean length of this edge using the vertex
    /// positions stored in `verts`.
    ///
    /// # Panics
    /// Panics if `self.v0` or `self.v1` are out of range for `verts`.
    pub fn length(&self, verts: &[MeshVertex]) -> f64 {
        let a = verts[self.v0].point_3d;
        let b = verts[self.v1].point_3d;
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let dz = b[2] - a[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ---------------------------------------------------------------------------
// MeshFaceData
// ---------------------------------------------------------------------------

/// Mesh data accumulator for a single BRep face, storing the lists of
/// vertices, edges, and triangles produced by the incremental mesher, along
/// with the deflection value used during meshing and a completion flag.
///
/// # Example
/// ```
/// use ironstream::brep_mesh_tools::{MeshFaceData, MeshVertex, MeshEdge};
/// let mut fd = MeshFaceData::new(0.1);
/// let vi = fd.add_vertex(MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]));
/// let ei = fd.add_edge(MeshEdge::new(0, 1));
/// fd.add_triangle([0, 1, 2]);
/// fd.mark_done();
/// assert!(fd.is_done());
/// assert_eq!(fd.nb_vertices(), 1);
/// assert_eq!(fd.nb_edges(), 1);
/// assert_eq!(fd.nb_triangles(), 1);
/// ```
// occt: BRepMesh_FaceAttribute
#[derive(Clone, Debug)]
pub struct MeshFaceData {
    /// Mesh vertices for this face.
    pub vertices: Vec<MeshVertex>,
    /// Mesh edges for this face.
    pub edges: Vec<MeshEdge>,
    /// Triangles, each stored as three 0-based vertex indices.
    pub triangles: Vec<[usize; 3]>,
    /// Linear deflection used when meshing this face.
    pub deflection: f64,
    /// Whether meshing of this face has been completed.
    pub is_done: bool,
}

impl MeshFaceData {
    /// Construct an empty face mesh accumulator with the given deflection.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::BRepMesh_FaceAttribute(face, deflection)`.
    pub fn new(deflection: f64) -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            triangles: Vec::new(),
            deflection,
            is_done: false,
        }
    }

    /// Append a vertex and return its 0-based index.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::AddNode`.
    pub fn add_vertex(&mut self, v: MeshVertex) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(v);
        idx
    }

    /// Append an edge and return its 0-based index.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::AddLink`.
    pub fn add_edge(&mut self, e: MeshEdge) -> usize {
        let idx = self.edges.len();
        self.edges.push(e);
        idx
    }

    /// Append a triangle given as three 0-based vertex indices.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::AddTriangle`.
    pub fn add_triangle(&mut self, t: [usize; 3]) {
        self.triangles.push(t);
    }

    /// Return the number of vertices accumulated so far.
    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Return the number of edges accumulated so far.
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Return the number of triangles accumulated so far.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Mark this face's mesh as complete.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::SetDone`.
    pub fn mark_done(&mut self) {
        self.is_done = true;
    }

    /// Return `true` if meshing of this face has been completed.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::IsDone`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return a reference to the vertex at 0-based index `i`.
    ///
    /// # Panics
    /// Panics if `i >= self.nb_vertices()`.
    ///
    /// Mirrors `BRepMesh_FaceAttribute::GetNode`.
    pub fn vertex(&self, i: usize) -> &MeshVertex {
        &self.vertices[i]
    }
}
