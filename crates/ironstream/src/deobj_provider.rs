// FILE: deobj_provider.rs
// occt: DEOBJ_Provider

//! Provider for reading and writing OBJ format CAD files.
//! Mirrors OCCT `DEOBJ_Provider` (DataExchange/TKDEOBJ/DEOBJ):
//! the provider is grouped by vendor name "OCC" and format "OBJ",
//! and both import and export are supported.
//!
//! External OCCT plumbing (TDocStd_Document, XSControl_WorkSession,
//! RWObj readers/writers) is modeled with local helper types; the
//! provider's own behavior — translating between an OBJ file on disk
//! and a triangulated model — is implemented for real.

use std::fs;
use std::path::Path;

/// Reference to a configuration node (local model of DE_ConfigurationNode handle).
#[derive(Clone, Debug, Default)]
pub struct DeObjConfigNodeRef {
    /// Scale factor applied to coordinates on read (OCCT: length unit conversion).
    pub file_length_unit: f64,
}

impl DeObjConfigNodeRef {
    pub fn new() -> Self {
        DeObjConfigNodeRef {
            file_length_unit: 1.0,
        }
    }
}

/// Represents a mesh triangulation read from or to be written to a file
/// (local model of Poly_Triangulation produced by RWObj).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshTriangulation {
    /// Vertices of the mesh
    pub vertices: Vec<(f64, f64, f64)>,
    /// Faces as indices into the vertex list
    pub faces: Vec<(usize, usize, usize)>,
}

impl MeshTriangulation {
    /// Creates an empty triangulation
    pub fn new() -> Self {
        MeshTriangulation {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    /// Adds a vertex and returns its index
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        let idx = self.vertices.len();
        self.vertices.push((x, y, z));
        idx
    }

    /// Adds a triangular face
    pub fn add_face(&mut self, v1: usize, v2: usize, v3: usize) {
        self.faces.push((v1, v2, v3));
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of faces
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

/// Local model of a TDocStd_Document holding the transferred mesh.
#[derive(Clone, Debug, Default)]
pub struct Document {
    pub mesh: MeshTriangulation,
    pub name: String,
}

impl Document {
    pub fn new() -> Self {
        Document {
            mesh: MeshTriangulation::new(),
            name: String::new(),
        }
    }
}

/// Provider for reading and writing OBJ format CAD files.
///
/// Vendor: OCC
/// Format: OBJ
/// Supports both document-based and shape-based read/write operations.
#[derive(Clone, Default)]
pub struct DeObjProvider {
    /// Configuration node for this provider
    pub config_node: Option<DeObjConfigNodeRef>,
}

impl DeObjProvider {
    /// Creates a new provider with default configuration
    pub fn new() -> Self {
        DeObjProvider { config_node: None }
    }

    /// Creates a provider with the specified configuration node
    pub fn from_config_node(node: DeObjConfigNodeRef) -> Self {
        DeObjProvider {
            config_node: Some(node),
        }
    }

    /// Gets the CAD format name
    pub fn get_format(&self) -> &'static str {
        "OBJ"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    fn scale(&self) -> f64 {
        self.config_node
            .as_ref()
            .map(|n| {
                if n.file_length_unit > 0.0 {
                    n.file_length_unit
                } else {
                    1.0
                }
            })
            .unwrap_or(1.0)
    }

    /// Reads an OBJ file into a document (mirrors `Read(path, document, ...)`).
    pub fn read_document(&self, file_path: &str) -> Result<Document, String> {
        let mesh = self.read_triangulation(file_path)?;
        let name = Path::new(file_path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        Ok(Document { mesh, name })
    }

    /// Writes a document to an OBJ file (mirrors `Write(path, document, ...)`).
    pub fn write_document(&self, file_path: &str, doc: &Document) -> Result<(), String> {
        self.write_triangulation(file_path, &doc.mesh)
    }

    /// Reads an OBJ file and returns mesh triangulation
    /// (mirrors `Read(path, shape, ...)` backed by RWObj_TriangulationReader).
    pub fn read_triangulation(&self, file_path: &str) -> Result<MeshTriangulation, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("DEOBJ_Provider: cannot read file '{}': {}", file_path, e))?;
        let scale = self.scale();
        let mut mesh = MeshTriangulation::new();

        for (line_no, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("v") => {
                    let coords: Vec<f64> = tokens
                        .take(3)
                        .map(|t| t.parse::<f64>())
                        .collect::<Result<_, _>>()
                        .map_err(|e| {
                            format!(
                                "DEOBJ_Provider: invalid vertex at line {}: {}",
                                line_no + 1,
                                e
                            )
                        })?;
                    if coords.len() != 3 {
                        return Err(format!(
                            "DEOBJ_Provider: vertex at line {} has {} coordinates, expected 3",
                            line_no + 1,
                            coords.len()
                        ));
                    }
                    mesh.add_vertex(coords[0] * scale, coords[1] * scale, coords[2] * scale);
                }
                Some("f") => {
                    // Face indices may be "i", "i/t", "i/t/n" or "i//n"; 1-based,
                    // negative values are relative to the current vertex count.
                    let mut idxs: Vec<usize> = Vec::new();
                    for tok in tokens {
                        let vert_part = tok.split('/').next().unwrap_or("");
                        let raw: i64 = vert_part.parse().map_err(|_| {
                            format!(
                                "DEOBJ_Provider: invalid face index '{}' at line {}",
                                tok,
                                line_no + 1
                            )
                        })?;
                        let n = mesh.vertex_count() as i64;
                        let resolved = if raw > 0 { raw - 1 } else { n + raw };
                        if resolved < 0 || resolved >= n {
                            return Err(format!(
                                "DEOBJ_Provider: face index {} out of range at line {}",
                                raw,
                                line_no + 1
                            ));
                        }
                        idxs.push(resolved as usize);
                    }
                    if idxs.len() < 3 {
                        return Err(format!(
                            "DEOBJ_Provider: face at line {} has fewer than 3 vertices",
                            line_no + 1
                        ));
                    }
                    // Fan-triangulate polygons, as RWObj does.
                    for k in 1..idxs.len() - 1 {
                        mesh.add_face(idxs[0], idxs[k], idxs[k + 1]);
                    }
                }
                // Ignore normals, texture coords, groups, materials, etc.
                _ => {}
            }
        }
        Ok(mesh)
    }

    /// Writes a mesh triangulation to an OBJ file
    /// (mirrors `Write(path, shape, ...)` backed by RWObj_CafWriter).
    pub fn write_triangulation(
        &self,
        file_path: &str,
        mesh: &MeshTriangulation,
    ) -> Result<(), String> {
        for (i, &(v1, v2, v3)) in mesh.faces.iter().enumerate() {
            let n = mesh.vertex_count();
            if v1 >= n || v2 >= n || v3 >= n {
                return Err(format!(
                    "DEOBJ_Provider: face {} references vertex out of range",
                    i
                ));
            }
        }
        let mut out = String::new();
        out.push_str("# Exported by DEOBJ_Provider (OCC)\n");
        for &(x, y, z) in &mesh.vertices {
            out.push_str(&format!("v {} {} {}\n", x, y, z));
        }
        for &(v1, v2, v3) in &mesh.faces {
            out.push_str(&format!("f {} {} {}\n", v1 + 1, v2 + 1, v3 + 1));
        }
        fs::write(file_path, out)
            .map_err(|e| format!("DEOBJ_Provider: cannot write file '{}': {}", file_path, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(tag: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!(
            "deobj_provider_{}_{}_{}.obj",
            tag,
            std::process::id(),
            nanos
        ));
        path
    }

    #[test]
    fn test_provider_creation() {
        let provider = DeObjProvider::new();
        assert_eq!(provider.get_format(), "OBJ");
        assert_eq!(provider.get_vendor(), "OCC");
        assert!(provider.config_node.is_none());
    }

    #[test]
    fn test_provider_default() {
        let provider = DeObjProvider::default();
        assert_eq!(provider.get_format(), "OBJ");
        assert_eq!(provider.get_vendor(), "OCC");
    }

    #[test]
    fn test_provider_from_config_node() {
        let provider = DeObjProvider::from_config_node(DeObjConfigNodeRef::new());
        assert!(provider.config_node.is_some());
    }

    #[test]
    fn test_read_document_missing_file() {
        let provider = DeObjProvider::new();
        let result = provider.read_document("/nonexistent/dir/test.obj");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_read_roundtrip_triangulation() {
        let provider = DeObjProvider::new();
        let mut mesh = MeshTriangulation::new();
        let v0 = mesh.add_vertex(0.0, 0.0, 0.0);
        let v1 = mesh.add_vertex(1.0, 0.0, 0.0);
        let v2 = mesh.add_vertex(1.0, 1.0, 0.0);
        let v3 = mesh.add_vertex(0.0, 1.0, 0.0);
        mesh.add_face(v0, v1, v2);
        mesh.add_face(v0, v2, v3);

        let path = temp_path("roundtrip");
        let path_str = path.to_string_lossy().into_owned();
        provider.write_triangulation(&path_str, &mesh).unwrap();
        let read_back = provider.read_triangulation(&path_str).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(read_back, mesh);
    }

    #[test]
    fn test_read_triangulation_parses_obj_text() {
        let provider = DeObjProvider::new();
        let path = temp_path("parse");
        let content = "# comment\n\
                       v 0 0 0\n\
                       v 1 0 0\n\
                       v 0 1 0\n\
                       vn 0 0 1\n\
                       f 1/1/1 2/2/1 3/3/1\n";
        std::fs::write(&path, content).unwrap();
        let mesh = provider
            .read_triangulation(&path.to_string_lossy())
            .unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.face_count(), 1);
        assert_eq!(mesh.faces[0], (0, 1, 2));
        assert_eq!(mesh.vertices[1], (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_read_triangulation_quad_is_fan_triangulated() {
        let provider = DeObjProvider::new();
        let path = temp_path("quad");
        let content = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\n";
        std::fs::write(&path, content).unwrap();
        let mesh = provider
            .read_triangulation(&path.to_string_lossy())
            .unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(mesh.face_count(), 2);
        assert_eq!(mesh.faces[0], (0, 1, 2));
        assert_eq!(mesh.faces[1], (0, 2, 3));
    }

    #[test]
    fn test_read_triangulation_negative_indices() {
        let provider = DeObjProvider::new();
        let path = temp_path("neg");
        let content = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
        std::fs::write(&path, content).unwrap();
        let mesh = provider
            .read_triangulation(&path.to_string_lossy())
            .unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(mesh.faces[0], (0, 1, 2));
    }

    #[test]
    fn test_read_triangulation_bad_index_errors() {
        let provider = DeObjProvider::new();
        let path = temp_path("bad");
        std::fs::write(&path, "v 0 0 0\nf 1 2 3\n").unwrap();
        let result = provider.read_triangulation(&path.to_string_lossy());
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_node_length_unit_scales_vertices() {
        let mut node = DeObjConfigNodeRef::new();
        node.file_length_unit = 1000.0; // e.g. metres -> millimetres
        let provider = DeObjProvider::from_config_node(node);

        let path = temp_path("scale");
        std::fs::write(&path, "v 1 2 3\nv 0 0 0\nv 1 0 0\nf 1 2 3\n").unwrap();
        let mesh = provider
            .read_triangulation(&path.to_string_lossy())
            .unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(mesh.vertices[0], (1000.0, 2000.0, 3000.0));
    }

    #[test]
    fn test_document_roundtrip() {
        let provider = DeObjProvider::new();
        let mut doc = Document::new();
        let v0 = doc.mesh.add_vertex(0.0, 0.0, 0.0);
        let v1 = doc.mesh.add_vertex(2.0, 0.0, 0.0);
        let v2 = doc.mesh.add_vertex(0.0, 2.0, 0.0);
        doc.mesh.add_face(v0, v1, v2);

        let path = temp_path("doc");
        let path_str = path.to_string_lossy().into_owned();
        provider.write_document(&path_str, &doc).unwrap();
        let read_back = provider.read_document(&path_str).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(read_back.mesh, doc.mesh);
        assert!(!read_back.name.is_empty());
    }

    #[test]
    fn test_write_triangulation_rejects_out_of_range_face() {
        let provider = DeObjProvider::new();
        let mut mesh = MeshTriangulation::new();
        mesh.add_vertex(0.0, 0.0, 0.0);
        mesh.add_face(0, 1, 2); // indices 1 and 2 do not exist

        let path = temp_path("reject");
        let result = provider.write_triangulation(&path.to_string_lossy(), &mesh);
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_mesh_triangulation_basics() {
        let mut tri = MeshTriangulation::new();
        assert_eq!(tri.vertex_count(), 0);
        assert_eq!(tri.face_count(), 0);

        let idx1 = tri.add_vertex(0.0, 0.0, 0.0);
        let idx2 = tri.add_vertex(1.0, 0.0, 0.0);
        let idx3 = tri.add_vertex(0.0, 1.0, 0.0);
        assert_eq!((idx1, idx2, idx3), (0, 1, 2));

        tri.add_face(idx1, idx2, idx3);
        assert_eq!(tri.face_count(), 1);
        assert_eq!(tri.faces[0], (0, 1, 2));
    }
}
