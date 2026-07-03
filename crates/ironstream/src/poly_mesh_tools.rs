// FILE: rust/ironstream/crates/ironstream/src/poly_mesh_tools.rs
//! `poly_mesh_tools` — per-mesh attribute arrays, a purpose-tagged
//! triangulation, PLY I/O, and mesh normal computation, mirroring the
//! attribute-enriched side of OpenCascade's `Poly` package and `RWPly`.
//!
//! | IronStream type    | OCCT origin                        |
//! |--------------------|------------------------------------|
//! | `PolyMeshPurpose`  | `Poly_MeshPurpose`                 |
//! | `PolyNormals`      | `Poly_ArrayOfNodes` (normal data)  |
//! | `PolyUVNodes`      | `Poly_ArrayOfUVNodes`              |
//! | `PolyMeshData`     | `Poly_Triangulation` (with attrs)  |
//! | `MeshPurpose`      | `Poly_MeshPurpose`                 |
//! | `MeshStats`        | `Poly_MeshPurpose` (stat helpers)  |
//! | `PlyWriter`        | `RWPly`                            |
//! | `PlyReader`        | `RWPly`                            |

// ─── PolyMeshPurpose ─────────────────────────────────────────────────────────

/// Semantic tag describing why a mesh was created or how it is used.
///
/// Mirrors `Poly_MeshPurpose` from OpenCascade's `Poly` package.
// occt: Poly_MeshPurpose
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PolyMeshPurpose {
    /// The mesh is the active representation currently in use.
    // occt: Poly_MeshPurpose_ACTIVE
    Active,
    /// The mesh has been loaded from persistent storage but is not yet active.
    // occt: Poly_MeshPurpose_LOADED
    Loaded,
    /// The mesh was produced by an algorithm (e.g. the mesher).
    // occt: Poly_MeshPurpose_COMPUTED
    Computed,
    /// The mesh was supplied directly by the application.
    // occt: Poly_MeshPurpose_USER
    User,
}

// ─── PolyNormals ─────────────────────────────────────────────────────────────

/// A flat, growable array of per-vertex normals stored as `[f32; 3]` triples.
///
/// Each entry is a unit-length (or unnormalised) normal vector `[nx, ny, nz]`.
/// Mirrors `Poly_ArrayOfNodes` when used for normal data in OpenCascade.
// occt: Poly_ArrayOfNodes
#[derive(Clone, Debug, Default)]
pub struct PolyNormals {
    normals: Vec<[f32; 3]>,
}

impl PolyNormals {
    /// Construct an empty normal array.
    #[inline]
    pub fn new() -> Self {
        Self { normals: Vec::new() }
    }

    /// Append a normal vector and return its index.
    #[inline]
    pub fn add(&mut self, n: [f32; 3]) {
        self.normals.push(n);
    }

    /// Return the normal at zero-based index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_normals()`.
    #[inline]
    pub fn get(&self, i: usize) -> [f32; 3] {
        assert!(
            i < self.normals.len(),
            "PolyNormals::get: index {i} out of range [0, {})",
            self.normals.len()
        );
        self.normals[i]
    }

    /// Return the number of normals stored.
    #[inline]
    pub fn nb_normals(&self) -> usize {
        self.normals.len()
    }

    /// Overwrite the normal at zero-based index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_normals()`.
    #[inline]
    pub fn set(&mut self, i: usize, n: [f32; 3]) {
        assert!(
            i < self.normals.len(),
            "PolyNormals::set: index {i} out of range [0, {})",
            self.normals.len()
        );
        self.normals[i] = n;
    }

    /// Return a slice over all stored normals.
    #[inline]
    pub fn iter(&self) -> &[[f32; 3]] {
        &self.normals
    }
}

// ─── PolyUVNodes ─────────────────────────────────────────────────────────────

/// A flat, growable array of per-vertex UV texture coordinates stored as
/// `[f32; 2]` pairs.
///
/// Mirrors `Poly_ArrayOfUVNodes` from OpenCascade's `Poly` package.
// occt: Poly_ArrayOfUVNodes
#[derive(Clone, Debug, Default)]
pub struct PolyUVNodes {
    pub(crate) uvs: Vec<[f32; 2]>,
}

impl PolyUVNodes {
    /// Construct an empty UV array.
    #[inline]
    pub fn new() -> Self {
        Self { uvs: Vec::new() }
    }

    /// Append a UV coordinate pair and record its position.
    #[inline]
    pub fn add(&mut self, uv: [f32; 2]) {
        self.uvs.push(uv);
    }

    /// Return the UV coordinate at zero-based index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_uvs()`.
    #[inline]
    pub fn get(&self, i: usize) -> [f32; 2] {
        assert!(
            i < self.uvs.len(),
            "PolyUVNodes::get: index {i} out of range [0, {})",
            self.uvs.len()
        );
        self.uvs[i]
    }

    /// Return the number of UV coordinates stored.
    #[inline]
    pub fn nb_uvs(&self) -> usize {
        self.uvs.len()
    }
}

// ─── PolyMeshData ────────────────────────────────────────────────────────────

/// A complete, attribute-enriched triangulation: 3-D nodes, triangle indices,
/// per-vertex normals, per-vertex UV coordinates, a mesh purpose tag, and the
/// linear deflection used during tessellation.
///
/// Mirrors `Poly_Triangulation` (with its attribute arrays) from OpenCascade's
/// `Poly` package.
// occt: Poly_Triangulation
#[derive(Clone, Debug)]
pub struct PolyMeshData {
    /// 3-D vertex positions `[x, y, z]` in model units.
    pub nodes: Vec<[f64; 3]>,
    /// Triangle connectivity: each entry is `[i0, i1, i2]` as zero-based
    /// indices into `nodes`.
    pub triangles: Vec<[usize; 3]>,
    /// Per-vertex normals, parallel to `nodes`.
    pub normals: PolyNormals,
    /// Per-vertex UV texture coordinates, parallel to `nodes`.
    pub uvs: PolyUVNodes,
    /// Semantic purpose of this mesh.
    pub purpose: PolyMeshPurpose,
    /// Linear deflection (chord error) used when the mesh was computed.
    pub deflection: f64,
}

impl PolyMeshData {
    /// Construct an empty mesh with the given tessellation `deflection`.
    ///
    /// The purpose is initialised to [`PolyMeshPurpose::Computed`], matching
    /// the typical output of a meshing algorithm.
    pub fn new(deflection: f64) -> Self {
        Self {
            nodes: Vec::new(),
            triangles: Vec::new(),
            normals: PolyNormals::new(),
            uvs: PolyUVNodes::new(),
            purpose: PolyMeshPurpose::Computed,
            deflection,
        }
    }

    /// Append a 3-D node and grow the parallel attribute arrays by one slot
    /// (normal defaults to `[0.0, 0.0, 0.0]`, UV defaults to `[0.0, 0.0]`).
    pub fn add_node(&mut self, p: [f64; 3]) {
        self.nodes.push(p);
        self.normals.add([0.0, 0.0, 0.0]);
        self.uvs.add([0.0, 0.0]);
    }

    /// Append a triangle given three zero-based node indices.
    #[inline]
    pub fn add_triangle(&mut self, tri: [usize; 3]) {
        self.triangles.push(tri);
    }

    /// Overwrite the normal at zero-based node index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_nodes()`.
    #[inline]
    pub fn set_normal(&mut self, i: usize, n: [f32; 3]) {
        assert!(
            i < self.nodes.len(),
            "PolyMeshData::set_normal: index {i} out of range [0, {})",
            self.nodes.len()
        );
        self.normals.set(i, n);
    }

    /// Overwrite the UV coordinate at zero-based node index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_nodes()`.
    #[inline]
    pub fn set_uv(&mut self, i: usize, uv: [f32; 2]) {
        assert!(
            i < self.nodes.len(),
            "PolyMeshData::set_uv: index {i} out of range [0, {})",
            self.nodes.len()
        );
        self.uvs.uvs[i] = uv;
    }

    /// Return the number of nodes (vertices).
    #[inline]
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of triangles.
    #[inline]
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Return the tessellation deflection.
    #[inline]
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Return the mesh purpose.
    #[inline]
    pub fn purpose(&self) -> PolyMeshPurpose {
        self.purpose
    }
}

// ─── MeshPurpose ─────────────────────────────────────────────────────────────

/// Simplified mesh purpose tag used for new PLY/analysis pipelines.
///
/// Mirrors `Poly_MeshPurpose` from OpenCascade's `Poly` package.
// occt: Poly_MeshPurpose
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MeshPurpose {
    /// General-purpose mesh with no specific semantic intent.
    Default,
    /// Mesh intended for visual display / rendering.
    Presentation,
    /// Mesh intended for numerical analysis (FEA, CFD, etc.).
    Analysis,
}

// ─── MeshStats ───────────────────────────────────────────────────────────────

/// Lightweight statistics descriptor for a triangulated mesh.
///
/// Mirrors counting helpers found around `Poly_MeshPurpose` in OCCT.
// occt: Poly_MeshPurpose
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshStats {
    /// Total number of vertex nodes.
    pub node_count: u32,
    /// Total number of triangular faces.
    pub triangle_count: u32,
    /// Total number of edges (boundary + interior).
    pub edge_count: u32,
}

impl MeshStats {
    /// Construct a zeroed `MeshStats`.
    pub fn new() -> Self {
        Self { node_count: 0, triangle_count: 0, edge_count: 0 }
    }

    /// Return `true` when the mesh contains no geometry (all counts are zero).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.node_count == 0 && self.triangle_count == 0 && self.edge_count == 0
    }
}

impl Default for MeshStats {
    fn default() -> Self {
        Self::new()
    }
}

// ─── PlyWriter ───────────────────────────────────────────────────────────────

/// Writer for the Stanford PLY polygon file format.
///
/// Mirrors `RWPly` from OpenCascade's `RWPly` package.
// occt: RWPly
pub struct PlyWriter {
    /// When `true` the output is little-endian binary PLY; when `false` ASCII.
    pub binary: bool,
    /// Comment lines embedded in the PLY header (one string per comment).
    pub comments: Vec<String>,
}

impl PlyWriter {
    /// Construct a new writer with ASCII output and no comments.
    pub fn new() -> Self {
        Self { binary: false, comments: Vec::new() }
    }

    /// Switch between binary (`true`) and ASCII (`false`) output.
    #[inline]
    pub fn set_binary(&mut self, b: bool) {
        self.binary = b;
    }

    /// Append a comment that will appear in the PLY header.
    #[inline]
    pub fn add_comment(&mut self, c: &str) {
        self.comments.push(c.to_owned());
    }

    /// Serialise `nodes` and `tris` to PLY bytes.
    ///
    /// The returned `Vec<u8>` is a self-contained PLY file that can be written
    /// directly to disk or streamed over a network.
    ///
    /// # Format
    /// * Header: standard PLY ASCII header regardless of `binary` flag for the
    ///   header itself; body follows the `binary` setting.
    /// * Vertex element: three `double` (or ASCII) properties `x y z`.
    /// * Face element: `uchar` list count followed by three `int` indices.
    pub fn write_mesh(&self, nodes: &[[f64; 3]], tris: &[[u32; 3]]) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        // ── header ──────────────────────────────────────────────────────────
        let format_str = if self.binary {
            "binary_little_endian 1.0"
        } else {
            "ascii 1.0"
        };

        // magic + format
        out.extend_from_slice(b"ply\n");
        out.extend_from_slice(b"format ");
        out.extend_from_slice(format_str.as_bytes());
        out.push(b'\n');

        // comments
        for c in &self.comments {
            out.extend_from_slice(b"comment ");
            out.extend_from_slice(c.as_bytes());
            out.push(b'\n');
        }

        // vertex element
        let nv = nodes.len();
        out.extend_from_slice(b"element vertex ");
        out.extend_from_slice(nv.to_string().as_bytes());
        out.push(b'\n');
        if self.binary {
            out.extend_from_slice(b"property double x\n");
            out.extend_from_slice(b"property double y\n");
            out.extend_from_slice(b"property double z\n");
        } else {
            out.extend_from_slice(b"property float x\n");
            out.extend_from_slice(b"property float y\n");
            out.extend_from_slice(b"property float z\n");
        }

        // face element
        let nf = tris.len();
        out.extend_from_slice(b"element face ");
        out.extend_from_slice(nf.to_string().as_bytes());
        out.push(b'\n');
        out.extend_from_slice(b"property list uchar int vertex_indices\n");
        out.extend_from_slice(b"end_header\n");

        // ── body ────────────────────────────────────────────────────────────
        if self.binary {
            // vertices: 3 x f64 little-endian
            for node in nodes {
                out.extend_from_slice(&node[0].to_le_bytes());
                out.extend_from_slice(&node[1].to_le_bytes());
                out.extend_from_slice(&node[2].to_le_bytes());
            }
            // faces: u8 count (3) + 3 x i32 little-endian
            for tri in tris {
                out.push(3u8);
                out.extend_from_slice(&(tri[0] as i32).to_le_bytes());
                out.extend_from_slice(&(tri[1] as i32).to_le_bytes());
                out.extend_from_slice(&(tri[2] as i32).to_le_bytes());
            }
        } else {
            // ASCII vertices
            for node in nodes {
                let line = format!("{} {} {}\n", node[0], node[1], node[2]);
                out.extend_from_slice(line.as_bytes());
            }
            // ASCII faces
            for tri in tris {
                let line = format!("3 {} {} {}\n", tri[0], tri[1], tri[2]);
                out.extend_from_slice(line.as_bytes());
            }
        }

        out
    }
}

impl Default for PlyWriter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── PlyReader ───────────────────────────────────────────────────────────────

/// Reader for the Stanford PLY polygon file format (ASCII and binary).
///
/// Mirrors `RWPly` from OpenCascade's `RWPly` package.
// occt: RWPly
pub struct PlyReader {
    /// Parsed vertex positions.
    pub nodes: Vec<[f64; 3]>,
    /// Parsed triangle index triplets (faces with exactly 3 vertices).
    pub triangles: Vec<[u32; 3]>,
}

impl PlyReader {
    /// Construct an empty reader (no data loaded yet).
    pub fn new() -> Self {
        Self { nodes: Vec::new(), triangles: Vec::new() }
    }

    /// Parse a PLY file from raw bytes, populating [`nodes`] and [`triangles`].
    ///
    /// Supports:
    /// * ASCII PLY
    /// * Binary little-endian PLY
    ///
    /// Returns `Err(String)` on malformed input.
    pub fn parse(&mut self, data: &[u8]) -> Result<(), String> {
        self.nodes.clear();
        self.triangles.clear();

        // ── locate header end ────────────────────────────────────────────────
        let end_header_marker = b"end_header\n";
        let header_end_pos = Self::find_subsequence(data, end_header_marker)
            .ok_or_else(|| "PLY parse error: missing end_header".to_owned())?;
        let body_start = header_end_pos + end_header_marker.len();

        let header_bytes = &data[..header_end_pos];
        let header_str = core::str::from_utf8(header_bytes)
            .map_err(|e| format!("PLY parse error: header not valid UTF-8: {e}"))?;

        // ── parse header ─────────────────────────────────────────────────────
        let mut is_binary_le = false;
        let mut vertex_count: usize = 0;
        let mut face_count: usize = 0;
        let mut in_vertex = false;
        let mut in_face = false;

        for line in header_str.lines() {
            let line = line.trim();
            if line.starts_with("format binary_little_endian") {
                is_binary_le = true;
            } else if line.starts_with("element vertex") {
                let n = line
                    .split_whitespace()
                    .nth(2)
                    .and_then(|s| s.parse::<usize>().ok())
                    .ok_or_else(|| {
                        format!("PLY parse error: malformed element vertex line: '{line}'")
                    })?;
                vertex_count = n;
                in_vertex = true;
                in_face = false;
            } else if line.starts_with("element face") {
                let n = line
                    .split_whitespace()
                    .nth(2)
                    .and_then(|s| s.parse::<usize>().ok())
                    .ok_or_else(|| {
                        format!("PLY parse error: malformed element face line: '{line}'")
                    })?;
                face_count = n;
                in_vertex = false;
                in_face = true;
            } else if line.starts_with("property") {
                // We accept but do not validate property lines beyond element context.
                let _ = (in_vertex, in_face);
            }
        }

        // ── parse body ───────────────────────────────────────────────────────
        let body = &data[body_start..];

        if is_binary_le {
            Self::parse_binary_le(body, vertex_count, face_count, &mut self.nodes, &mut self.triangles)?;
        } else {
            Self::parse_ascii(body, vertex_count, face_count, &mut self.nodes, &mut self.triangles)?;
        }

        Ok(())
    }

    /// Return the number of parsed nodes.
    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // ── private helpers ──────────────────────────────────────────────────────

    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|w| w == needle)
    }

    fn parse_ascii(
        body: &[u8],
        vertex_count: usize,
        face_count: usize,
        nodes: &mut Vec<[f64; 3]>,
        triangles: &mut Vec<[u32; 3]>,
    ) -> Result<(), String> {
        let text = core::str::from_utf8(body)
            .map_err(|e| format!("PLY parse error: body not valid UTF-8: {e}"))?;
        let mut lines = text.lines();

        // vertices
        for i in 0..vertex_count {
            let line = lines
                .next()
                .ok_or_else(|| format!("PLY parse error: expected vertex line {i}"))?;
            let mut parts = line.split_whitespace();
            let x: f64 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: vertex {i} bad x"))?;
            let y: f64 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: vertex {i} bad y"))?;
            let z: f64 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: vertex {i} bad z"))?;
            nodes.push([x, y, z]);
        }

        // faces (only triangles supported)
        for i in 0..face_count {
            let line = lines
                .next()
                .ok_or_else(|| format!("PLY parse error: expected face line {i}"))?;
            let mut parts = line.split_whitespace();
            let count: u32 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: face {i} missing vertex count"))?;
            if count != 3 {
                return Err(format!(
                    "PLY parse error: face {i} has {count} vertices, only triangles (3) supported"
                ));
            }
            let a: u32 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: face {i} bad index 0"))?;
            let b: u32 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: face {i} bad index 1"))?;
            let c: u32 = parts
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("PLY parse error: face {i} bad index 2"))?;
            triangles.push([a, b, c]);
        }

        Ok(())
    }

    fn parse_binary_le(
        body: &[u8],
        vertex_count: usize,
        face_count: usize,
        nodes: &mut Vec<[f64; 3]>,
        triangles: &mut Vec<[u32; 3]>,
    ) -> Result<(), String> {
        let mut pos = 0usize;

        let read_f64 = |buf: &[u8], p: &mut usize| -> Result<f64, String> {
            if *p + 8 > buf.len() {
                return Err(format!("PLY binary parse error: unexpected end at byte {p}"));
            }
            let bytes: [u8; 8] = buf[*p..*p + 8].try_into().unwrap();
            *p += 8;
            Ok(f64::from_le_bytes(bytes))
        };

        let read_u8 = |buf: &[u8], p: &mut usize| -> Result<u8, String> {
            if *p >= buf.len() {
                return Err(format!("PLY binary parse error: unexpected end at byte {p}"));
            }
            let v = buf[*p];
            *p += 1;
            Ok(v)
        };

        let read_i32 = |buf: &[u8], p: &mut usize| -> Result<i32, String> {
            if *p + 4 > buf.len() {
                return Err(format!("PLY binary parse error: unexpected end at byte {p}"));
            }
            let bytes: [u8; 4] = buf[*p..*p + 4].try_into().unwrap();
            *p += 4;
            Ok(i32::from_le_bytes(bytes))
        };

        for i in 0..vertex_count {
            let x = read_f64(body, &mut pos)
                .map_err(|e| format!("{e} (vertex {i} x)"))?;
            let y = read_f64(body, &mut pos)
                .map_err(|e| format!("{e} (vertex {i} y)"))?;
            let z = read_f64(body, &mut pos)
                .map_err(|e| format!("{e} (vertex {i} z)"))?;
            nodes.push([x, y, z]);
        }

        for i in 0..face_count {
            let count = read_u8(body, &mut pos)
                .map_err(|e| format!("{e} (face {i} count)"))?;
            if count != 3 {
                return Err(format!(
                    "PLY binary parse error: face {i} has {count} vertices, only triangles supported"
                ));
            }
            let a = read_i32(body, &mut pos)
                .map_err(|e| format!("{e} (face {i} index 0)"))?;
            let b = read_i32(body, &mut pos)
                .map_err(|e| format!("{e} (face {i} index 1)"))?;
            let c = read_i32(body, &mut pos)
                .map_err(|e| format!("{e} (face {i} index 2)"))?;
            if a < 0 || b < 0 || c < 0 {
                return Err(format!("PLY binary parse error: face {i} has negative index"));
            }
            triangles.push([a as u32, b as u32, c as u32]);
        }

        Ok(())
    }
}

impl Default for PlyReader {
    fn default() -> Self {
        Self::new()
    }
}

// ─── mesh_algo_normals ───────────────────────────────────────────────────────

/// Compute per-vertex normals by averaging the face normals of all triangles
/// incident to each vertex.
///
/// Mirrors the normal-computation utilities in `Poly_MeshPurpose` / `PolyAlgo`
/// from OpenCascade's `Poly` package.
///
/// # Algorithm
/// 1. For each triangle, compute the face normal via the cross product of two
///    edge vectors (area-weighted contribution).
/// 2. Accumulate the face normal into each of its three vertex accumulators.
/// 3. Normalise each accumulated vector to unit length.  Isolated vertices
///    (not referenced by any triangle) receive `[0.0, 0.0, 0.0]`.
///
/// # Returns
/// A `Vec` of length `nodes.len()` where each entry is the unit normal at the
/// corresponding vertex.
// occt: Poly_MeshPurpose
pub fn mesh_algo_normals(nodes: &[[f64; 3]], tris: &[[u32; 3]]) -> Vec<[f64; 3]> {
    let n = nodes.len();
    let mut accum: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]; n];

    for tri in tris {
        let [i0, i1, i2] = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
        if i0 >= n || i1 >= n || i2 >= n {
            continue;
        }
        let p0 = nodes[i0];
        let p1 = nodes[i1];
        let p2 = nodes[i2];

        // edge vectors
        let e1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
        let e2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];

        // cross product (area-weighted face normal)
        let cx = e1[1] * e2[2] - e1[2] * e2[1];
        let cy = e1[2] * e2[0] - e1[0] * e2[2];
        let cz = e1[0] * e2[1] - e1[1] * e2[0];

        for idx in [i0, i1, i2] {
            accum[idx][0] += cx;
            accum[idx][1] += cy;
            accum[idx][2] += cz;
        }
    }

    // normalise
    for v in &mut accum {
        let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if len > 1e-20 {
            v[0] /= len;
            v[1] /= len;
            v[2] /= len;
        }
    }

    accum
}

// ─── Internal unit tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── PolyMeshPurpose ──────────────────────────────────────────────────────

    #[test]
    fn purpose_variants_distinct() {
        assert_ne!(PolyMeshPurpose::Active, PolyMeshPurpose::Loaded);
        assert_ne!(PolyMeshPurpose::Computed, PolyMeshPurpose::User);
    }

    #[test]
    fn purpose_copy_and_debug() {
        let p = PolyMeshPurpose::Active;
        let q = p; // Copy
        assert_eq!(p, q);
        let s = format!("{:?}", p);
        assert!(s.contains("Active"));
    }

    // ── PolyNormals ──────────────────────────────────────────────────────────

    #[test]
    fn normals_new_is_empty() {
        let n = PolyNormals::new();
        assert_eq!(n.nb_normals(), 0);
        assert_eq!(n.iter().len(), 0);
    }

    #[test]
    fn normals_add_and_get() {
        let mut n = PolyNormals::new();
        n.add([0.0, 0.0, 1.0]);
        n.add([1.0, 0.0, 0.0]);
        assert_eq!(n.nb_normals(), 2);
        assert_eq!(n.get(0), [0.0_f32, 0.0, 1.0]);
        assert_eq!(n.get(1), [1.0_f32, 0.0, 0.0]);
    }

    #[test]
    fn normals_set_overwrites() {
        let mut n = PolyNormals::new();
        n.add([0.0, 0.0, 1.0]);
        n.set(0, [0.0, 1.0, 0.0]);
        assert_eq!(n.get(0), [0.0_f32, 1.0, 0.0]);
    }

    #[test]
    fn normals_iter_is_slice() {
        let mut n = PolyNormals::new();
        n.add([1.0, 0.0, 0.0]);
        n.add([0.0, 1.0, 0.0]);
        n.add([0.0, 0.0, 1.0]);
        let s = n.iter();
        assert_eq!(s.len(), 3);
        assert_eq!(s[2], [0.0_f32, 0.0, 1.0]);
    }

    #[test]
    #[should_panic]
    fn normals_get_out_of_range_panics() {
        let n = PolyNormals::new();
        let _ = n.get(0);
    }

    #[test]
    #[should_panic]
    fn normals_set_out_of_range_panics() {
        let mut n = PolyNormals::new();
        n.set(0, [0.0, 0.0, 1.0]);
    }

    // ── PolyUVNodes ──────────────────────────────────────────────────────────

    #[test]
    fn uvnodes_new_is_empty() {
        let uv = PolyUVNodes::new();
        assert_eq!(uv.nb_uvs(), 0);
    }

    #[test]
    fn uvnodes_add_and_get() {
        let mut uv = PolyUVNodes::new();
        uv.add([0.0, 0.0]);
        uv.add([1.0, 0.5]);
        assert_eq!(uv.nb_uvs(), 2);
        assert_eq!(uv.get(0), [0.0_f32, 0.0]);
        assert_eq!(uv.get(1), [1.0_f32, 0.5]);
    }

    #[test]
    #[should_panic]
    fn uvnodes_get_out_of_range_panics() {
        let uv = PolyUVNodes::new();
        let _ = uv.get(0);
    }

    // ── PolyMeshData ─────────────────────────────────────────────────────────

    #[test]
    fn mesh_data_new_defaults() {
        let m = PolyMeshData::new(0.1);
        assert_eq!(m.nb_nodes(), 0);
        assert_eq!(m.nb_triangles(), 0);
        assert_eq!(m.purpose(), PolyMeshPurpose::Computed);
        assert!((m.deflection() - 0.1).abs() < 1e-15);
    }

    #[test]
    fn mesh_data_add_nodes_and_triangles() {
        let mut m = PolyMeshData::new(0.01);
        m.add_node([0.0, 0.0, 0.0]);
        m.add_node([1.0, 0.0, 0.0]);
        m.add_node([0.0, 1.0, 0.0]);
        m.add_triangle([0, 1, 2]);
        assert_eq!(m.nb_nodes(), 3);
        assert_eq!(m.nb_triangles(), 1);
        assert_eq!(m.triangles[0], [0, 1, 2]);
    }

    #[test]
    fn mesh_data_add_node_grows_normals_and_uvs() {
        let mut m = PolyMeshData::new(0.05);
        m.add_node([1.0, 2.0, 3.0]);
        assert_eq!(m.normals.nb_normals(), 1);
        assert_eq!(m.uvs.nb_uvs(), 1);
        assert_eq!(m.normals.get(0), [0.0_f32, 0.0, 0.0]);
        assert_eq!(m.uvs.get(0), [0.0_f32, 0.0]);
    }

    #[test]
    fn mesh_data_set_normal_and_uv() {
        let mut m = PolyMeshData::new(0.01);
        m.add_node([0.0, 0.0, 0.0]);
        m.set_normal(0, [0.0, 0.0, 1.0]);
        m.set_uv(0, [0.5, 0.5]);
        assert_eq!(m.normals.get(0), [0.0_f32, 0.0, 1.0]);
        assert_eq!(m.uvs.get(0), [0.5_f32, 0.5]);
    }

    #[test]
    #[should_panic]
    fn mesh_data_set_normal_no_node_panics() {
        let mut m = PolyMeshData::new(0.01);
        m.set_normal(0, [0.0, 0.0, 1.0]);
    }

    #[test]
    #[should_panic]
    fn mesh_data_set_uv_no_node_panics() {
        let mut m = PolyMeshData::new(0.01);
        m.set_uv(0, [0.0, 0.0]);
    }

    #[test]
    fn mesh_data_purpose_field_writable() {
        let mut m = PolyMeshData::new(0.0);
        m.purpose = PolyMeshPurpose::User;
        assert_eq!(m.purpose(), PolyMeshPurpose::User);
    }

    #[test]
    fn mesh_data_deflection_accessor() {
        let m = PolyMeshData::new(0.25);
        assert!((m.deflection() - 0.25).abs() < 1e-15);
    }

    // ── MeshPurpose ──────────────────────────────────────────────────────────

    #[test]
    fn mesh_purpose_variants() {
        assert_ne!(MeshPurpose::Default, MeshPurpose::Presentation);
        assert_ne!(MeshPurpose::Presentation, MeshPurpose::Analysis);
        let p = MeshPurpose::Analysis;
        let q = p;
        assert_eq!(p, q);
    }

    // ── MeshStats ────────────────────────────────────────────────────────────

    #[test]
    fn mesh_stats_new_is_empty() {
        let s = MeshStats::new();
        assert!(s.is_empty());
        assert_eq!(s.node_count, 0);
        assert_eq!(s.triangle_count, 0);
        assert_eq!(s.edge_count, 0);
    }

    #[test]
    fn mesh_stats_not_empty_when_nodes_present() {
        let s = MeshStats { node_count: 3, triangle_count: 1, edge_count: 3 };
        assert!(!s.is_empty());
    }

    #[test]
    fn mesh_stats_partial_zero_not_empty() {
        let s = MeshStats { node_count: 0, triangle_count: 1, edge_count: 0 };
        assert!(!s.is_empty());
    }

    // ── PlyWriter ────────────────────────────────────────────────────────────

    #[test]
    fn ply_writer_ascii_round_trip() {
        let mut w = PlyWriter::new();
        w.add_comment("test mesh");

        let nodes: &[[f64; 3]] = &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let tris: &[[u32; 3]] = &[[0, 1, 2]];
        let bytes = w.write_mesh(nodes, tris);

        let text = core::str::from_utf8(&bytes).expect("ascii PLY must be valid UTF-8");
        assert!(text.starts_with("ply\n"));
        assert!(text.contains("format ascii"));
        assert!(text.contains("comment test mesh"));
        assert!(text.contains("element vertex 3"));
        assert!(text.contains("element face 1"));
        assert!(text.contains("end_header\n"));
        assert!(text.contains("3 0 1 2"));
    }

    #[test]
    fn ply_writer_binary_starts_with_ply() {
        let mut w = PlyWriter::new();
        w.set_binary(true);
        let nodes: &[[f64; 3]] = &[[0.0, 0.0, 0.0]];
        let tris: &[[u32; 3]] = &[];
        let bytes = w.write_mesh(nodes, tris);
        assert!(bytes.starts_with(b"ply\n"));
        assert!(core::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == b'\n').unwrap() + 1])
            .unwrap()
            .starts_with("ply"));
    }

    // ── PlyReader ────────────────────────────────────────────────────────────

    #[test]
    fn ply_reader_ascii_parse() {
        let ply = b"ply\nformat ascii 1.0\nelement vertex 3\nproperty float x\nproperty float y\nproperty float z\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n0 0 0\n1 0 0\n0 1 0\n3 0 1 2\n";
        let mut r = PlyReader::new();
        r.parse(ply).expect("parse should succeed");
        assert_eq!(r.node_count(), 3);
        assert_eq!(r.triangles.len(), 1);
        assert_eq!(r.triangles[0], [0, 1, 2]);
    }

    #[test]
    fn ply_reader_binary_round_trip() {
        let nodes: &[[f64; 3]] = &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let tris: &[[u32; 3]] = &[[0, 1, 2]];

        let mut w = PlyWriter::new();
        w.set_binary(true);
        let bytes = w.write_mesh(nodes, tris);

        let mut r = PlyReader::new();
        r.parse(&bytes).expect("binary parse should succeed");
        assert_eq!(r.node_count(), 3);
        assert_eq!(r.triangles.len(), 1);
        assert_eq!(r.triangles[0], [0, 1, 2]);

        let eps = 1e-12;
        for (a, b) in r.nodes.iter().zip(nodes.iter()) {
            assert!((a[0] - b[0]).abs() < eps);
            assert!((a[1] - b[1]).abs() < eps);
            assert!((a[2] - b[2]).abs() < eps);
        }
    }

    #[test]
    fn ply_reader_missing_end_header_errors() {
        let bad = b"ply\nformat ascii 1.0\n";
        let mut r = PlyReader::new();
        assert!(r.parse(bad).is_err());
    }

    // ── mesh_algo_normals ────────────────────────────────────────────────────

    #[test]
    fn normals_single_triangle_points_up() {
        let nodes: &[[f64; 3]] = &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let tris: &[[u32; 3]] = &[[0, 1, 2]];
        let normals = mesh_algo_normals(nodes, tris);
        assert_eq!(normals.len(), 3);
        // all three vertices share this single triangle, normal should be +Z
        for n in &normals {
            assert!((n[0]).abs() < 1e-10, "nx should be ~0");
            assert!((n[1]).abs() < 1e-10, "ny should be ~0");
            assert!((n[2] - 1.0).abs() < 1e-10, "nz should be ~1");
        }
    }

    #[test]
    fn normals_isolated_vertex_is_zero() {
        let nodes: &[[f64; 3]] = &[
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [5.0, 5.0, 5.0], // isolated
        ];
        let tris: &[[u32; 3]] = &[[0, 1, 2]];
        let normals = mesh_algo_normals(nodes, tris);
        assert_eq!(normals.len(), 4);
        assert_eq!(normals[3], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn normals_empty_mesh() {
        let normals = mesh_algo_normals(&[], &[]);
        assert!(normals.is_empty());
    }
}
