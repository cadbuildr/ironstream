// FILE: deply_provider.rs
// occt: DEPLY_Provider

//! Provider for writing PLY format CAD files.
//! Mirrors OCCT `DEPLY_Provider` (DataExchange/TKDEPLY/DEPLY):
//! vendor name "OCC", format "PLY"; the import process is NOT supported,
//! the export process is supported.
//!
//! External OCCT plumbing (TDocStd_Document, RWPly_CafWriter) is modeled
//! with local helper types; the provider's own behavior — serializing a
//! triangulated model to an ASCII PLY file — is implemented for real.

use std::fs;

/// Provider for writing PLY format CAD files.
/// Handles PLY file export operations with configuration from DEPLY_ConfigurationNode.
/// PLY (Polygon File Format) export only; no import support.
///
/// Vendor: OCC
/// Format: PLY
/// Supports document-based and shape-based write operations only.
#[derive(Clone)]
pub struct DeplyProvider {
    /// Configuration node for this provider
    pub config_node: Option<DeplyConfigNodeRef>,
}

/// Reference to a PLY configuration node
#[derive(Clone, Debug, Default)]
pub struct DeplyConfigNodeRef;

/// Local model of a TDocStd_Document holding the mesh to export.
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

impl DeplyProvider {
    /// Creates a new provider with default configuration
    pub fn new() -> Self {
        DeplyProvider {
            config_node: None,
        }
    }

    /// Creates a provider with the specified configuration node
    pub fn from_config_node(node: DeplyConfigNodeRef) -> Self {
        DeplyProvider {
            config_node: Some(node),
        }
    }

    /// Gets the CAD format name
    pub fn get_format(&self) -> &'static str {
        "PLY"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Reading PLY is not supported by this provider (as in OCCT).
    pub fn read_document(&self, _file_path: &str) -> Result<Document, String> {
        Err(format!(
            "DEPLY_Provider: {} provider doesn't support the import process",
            self.get_format()
        ))
    }

    /// Writes a document to a PLY file (mirrors `Write(path, document, ...)`).
    pub fn write_document(&self, file_path: &str, doc: &Document) -> Result<(), String> {
        self.write_triangulation(file_path, &doc.mesh)
    }

    /// Writes a mesh triangulation to an ASCII PLY file
    /// (mirrors the RWPly_PlyWriterContext-backed export).
    pub fn write_triangulation(
        &self,
        file_path: &str,
        mesh: &MeshTriangulation,
    ) -> Result<(), String> {
        let n_vert = mesh.vertex_count();

        // Validate optional per-vertex attributes.
        if mesh.has_normals() && mesh.normals.as_ref().unwrap().len() != n_vert {
            return Err("DEPLY_Provider: normal count does not match vertex count".to_string());
        }
        if mesh.has_colors() && mesh.colors.as_ref().unwrap().len() != n_vert {
            return Err("DEPLY_Provider: color count does not match vertex count".to_string());
        }
        if mesh.has_tex_coords() && mesh.tex_coords.as_ref().unwrap().len() != n_vert {
            return Err(
                "DEPLY_Provider: texture coordinate count does not match vertex count".to_string(),
            );
        }
        for (i, &(v1, v2, v3)) in mesh.faces.iter().enumerate() {
            if v1 >= n_vert || v2 >= n_vert || v3 >= n_vert {
                return Err(format!(
                    "DEPLY_Provider: face {} references vertex out of range",
                    i
                ));
            }
        }

        let mut out = String::new();
        out.push_str("ply\n");
        out.push_str("format ascii 1.0\n");
        out.push_str("comment Exported by DEPLY_Provider (OCC)\n");
        out.push_str(&format!("element vertex {}\n", n_vert));
        out.push_str("property float64 x\nproperty float64 y\nproperty float64 z\n");
        if mesh.has_normals() {
            out.push_str("property float64 nx\nproperty float64 ny\nproperty float64 nz\n");
        }
        if mesh.has_tex_coords() {
            out.push_str("property float64 s\nproperty float64 t\n");
        }
        if mesh.has_colors() {
            out.push_str(
                "property uchar red\nproperty uchar green\nproperty uchar blue\nproperty uchar alpha\n",
            );
        }
        out.push_str(&format!("element face {}\n", mesh.face_count()));
        out.push_str("property list uchar int vertex_indices\n");
        out.push_str("end_header\n");

        for i in 0..n_vert {
            let (x, y, z) = mesh.vertices[i];
            out.push_str(&format!("{} {} {}", x, y, z));
            if mesh.has_normals() {
                let (nx, ny, nz) = mesh.normals.as_ref().unwrap()[i];
                out.push_str(&format!(" {} {} {}", nx, ny, nz));
            }
            if mesh.has_tex_coords() {
                let (u, v) = mesh.tex_coords.as_ref().unwrap()[i];
                out.push_str(&format!(" {} {}", u, v));
            }
            if mesh.has_colors() {
                let (r, g, b, a) = mesh.colors.as_ref().unwrap()[i];
                out.push_str(&format!(" {} {} {} {}", r, g, b, a));
            }
            out.push('\n');
        }
        for &(v1, v2, v3) in &mesh.faces {
            out.push_str(&format!("3 {} {} {}\n", v1, v2, v3));
        }

        fs::write(file_path, out)
            .map_err(|e| format!("DEPLY_Provider: cannot write file '{}': {}", file_path, e))
    }
}

impl Default for DeplyProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a mesh triangulation for PLY export
#[derive(Clone, Debug)]
pub struct MeshTriangulation {
    /// Vertices of the mesh (x, y, z)
    pub vertices: Vec<(f64, f64, f64)>,
    /// Vertex normals (nx, ny, nz), optional
    pub normals: Option<Vec<(f64, f64, f64)>>,
    /// Vertex colors (r, g, b, a), optional
    pub colors: Option<Vec<(u8, u8, u8, u8)>>,
    /// Texture coordinates (u, v), optional
    pub tex_coords: Option<Vec<(f64, f64)>>,
    /// Faces as indices into the vertex list (v1, v2, v3)
    pub faces: Vec<(usize, usize, usize)>,
    /// Face metadata (part id or face id), optional
    pub face_metadata: Option<Vec<usize>>,
}

impl MeshTriangulation {
    /// Creates an empty triangulation
    pub fn new() -> Self {
        MeshTriangulation {
            vertices: Vec::new(),
            normals: None,
            colors: None,
            tex_coords: None,
            faces: Vec::new(),
            face_metadata: None,
        }
    }

    /// Adds a vertex and returns its index
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        let idx = self.vertices.len();
        self.vertices.push((x, y, z));
        idx
    }

    /// Adds a normal vector
    pub fn add_normal(&mut self, nx: f64, ny: f64, nz: f64) {
        if self.normals.is_none() {
            self.normals = Some(Vec::new());
        }
        if let Some(ref mut normals) = self.normals {
            normals.push((nx, ny, nz));
        }
    }

    /// Adds a vertex color
    pub fn add_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        if self.colors.is_none() {
            self.colors = Some(Vec::new());
        }
        if let Some(ref mut colors) = self.colors {
            colors.push((r, g, b, a));
        }
    }

    /// Adds texture coordinates
    pub fn add_tex_coord(&mut self, u: f64, v: f64) {
        if self.tex_coords.is_none() {
            self.tex_coords = Some(Vec::new());
        }
        if let Some(ref mut coords) = self.tex_coords {
            coords.push((u, v));
        }
    }

    /// Adds a triangular face
    pub fn add_face(&mut self, v1: usize, v2: usize, v3: usize) {
        self.faces.push((v1, v2, v3));
    }

    /// Sets face metadata (part id or face id)
    pub fn set_face_metadata(&mut self, metadata: Vec<usize>) {
        self.face_metadata = Some(metadata);
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of faces
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Returns whether the mesh has normals
    pub fn has_normals(&self) -> bool {
        self.normals.is_some() && self.normals.as_ref().map_or(false, |n| !n.is_empty())
    }

    /// Returns whether the mesh has colors
    pub fn has_colors(&self) -> bool {
        self.colors.is_some() && self.colors.as_ref().map_or(false, |c| !c.is_empty())
    }

    /// Returns whether the mesh has texture coordinates
    pub fn has_tex_coords(&self) -> bool {
        self.tex_coords.is_some() && self.tex_coords.as_ref().map_or(false, |t| !t.is_empty())
    }

    /// Returns whether the mesh has face metadata
    pub fn has_face_metadata(&self) -> bool {
        self.face_metadata.is_some() && self.face_metadata.as_ref().map_or(false, |m| !m.is_empty())
    }
}

impl Default for MeshTriangulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = DeplyProvider::new();
        assert_eq!(provider.get_format(), "PLY");
        assert_eq!(provider.get_vendor(), "OCC");
        assert!(provider.config_node.is_none());
    }

    #[test]
    fn test_provider_default() {
        let provider = DeplyProvider::default();
        assert_eq!(provider.get_format(), "PLY");
        assert_eq!(provider.get_vendor(), "OCC");
    }

    fn temp_path(tag: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!(
            "deply_provider_{}_{}_{}.ply",
            tag,
            std::process::id(),
            nanos
        ));
        path
    }

    #[test]
    fn test_read_document_not_supported() {
        let provider = DeplyProvider::new();
        let result = provider.read_document("test.ply");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("doesn't support the import"));
    }

    #[test]
    fn test_write_document_produces_valid_ply() {
        let provider = DeplyProvider::new();
        let mut doc = Document::new();
        let v0 = doc.mesh.add_vertex(0.0, 0.0, 0.0);
        let v1 = doc.mesh.add_vertex(1.0, 0.0, 0.0);
        let v2 = doc.mesh.add_vertex(0.0, 1.0, 0.0);
        doc.mesh.add_face(v0, v1, v2);

        let path = temp_path("doc");
        let path_str = path.to_string_lossy().into_owned();
        provider.write_document(&path_str, &doc).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        assert!(content.starts_with("ply\nformat ascii 1.0\n"));
        assert!(content.contains("element vertex 3\n"));
        assert!(content.contains("element face 1\n"));
        assert!(content.contains("end_header\n"));
        assert!(content.contains("3 0 1 2\n"));
    }

    #[test]
    fn test_mesh_triangulation_creation() {
        let tri = MeshTriangulation::new();
        assert_eq!(tri.vertex_count(), 0);
        assert_eq!(tri.face_count(), 0);
        assert!(!tri.has_normals());
        assert!(!tri.has_colors());
        assert!(!tri.has_tex_coords());
        assert!(!tri.has_face_metadata());
    }

    #[test]
    fn test_mesh_triangulation_add_vertex() {
        let mut tri = MeshTriangulation::new();
        let idx1 = tri.add_vertex(0.0, 0.0, 0.0);
        let idx2 = tri.add_vertex(1.0, 0.0, 0.0);
        let idx3 = tri.add_vertex(0.0, 1.0, 0.0);

        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx3, 2);
        assert_eq!(tri.vertex_count(), 3);
    }

    #[test]
    fn test_mesh_triangulation_add_normal() {
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_normal(0.0, 0.0, 1.0);

        assert!(tri.has_normals());
        assert_eq!(tri.normals.as_ref().unwrap().len(), 1);
        assert_eq!(tri.normals.as_ref().unwrap()[0], (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_mesh_triangulation_add_color() {
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_color(255, 128, 64, 255);

        assert!(tri.has_colors());
        assert_eq!(tri.colors.as_ref().unwrap().len(), 1);
        assert_eq!(tri.colors.as_ref().unwrap()[0], (255, 128, 64, 255));
    }

    #[test]
    fn test_mesh_triangulation_add_tex_coord() {
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_tex_coord(0.5, 0.5);

        assert!(tri.has_tex_coords());
        assert_eq!(tri.tex_coords.as_ref().unwrap().len(), 1);
        assert_eq!(tri.tex_coords.as_ref().unwrap()[0], (0.5, 0.5));
    }

    #[test]
    fn test_mesh_triangulation_add_face() {
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_vertex(1.0, 0.0, 0.0);
        tri.add_vertex(0.0, 1.0, 0.0);
        tri.add_face(0, 1, 2);

        assert_eq!(tri.face_count(), 1);
        assert_eq!(tri.faces[0], (0, 1, 2));
    }

    #[test]
    fn test_mesh_triangulation_face_metadata() {
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_vertex(1.0, 0.0, 0.0);
        tri.add_vertex(0.0, 1.0, 0.0);
        tri.add_face(0, 1, 2);

        let metadata = vec![1, 1, 2, 2];
        tri.set_face_metadata(metadata);

        assert!(tri.has_face_metadata());
        assert_eq!(tri.face_metadata.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_mesh_triangulation_complete() {
        let mut tri = MeshTriangulation::new();
        let v0 = tri.add_vertex(0.0, 0.0, 0.0);
        let v1 = tri.add_vertex(1.0, 0.0, 0.0);
        let v2 = tri.add_vertex(1.0, 1.0, 0.0);

        tri.add_normal(0.0, 0.0, 1.0);
        tri.add_normal(0.0, 0.0, 1.0);
        tri.add_normal(0.0, 0.0, 1.0);

        tri.add_color(255, 0, 0, 255);
        tri.add_color(0, 255, 0, 255);
        tri.add_color(0, 0, 255, 255);

        tri.add_face(v0, v1, v2);

        assert_eq!(tri.vertex_count(), 3);
        assert_eq!(tri.face_count(), 1);
        assert!(tri.has_normals());
        assert!(tri.has_colors());
    }

    #[test]
    fn test_write_triangulation_with_attributes() {
        let provider = DeplyProvider::new();
        let mut tri = MeshTriangulation::new();
        let v0 = tri.add_vertex(0.0, 0.0, 0.0);
        let v1 = tri.add_vertex(1.0, 0.0, 0.0);
        let v2 = tri.add_vertex(1.0, 1.0, 0.0);
        tri.add_normal(0.0, 0.0, 1.0);
        tri.add_normal(0.0, 0.0, 1.0);
        tri.add_normal(0.0, 0.0, 1.0);
        tri.add_color(255, 0, 0, 255);
        tri.add_color(0, 255, 0, 255);
        tri.add_color(0, 0, 255, 255);
        tri.add_face(v0, v1, v2);

        let path = temp_path("attrs");
        provider
            .write_triangulation(&path.to_string_lossy(), &tri)
            .unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        assert!(content.contains("property float64 nx\n"));
        assert!(content.contains("property uchar red\n"));
        // First vertex row: position, normal, color.
        assert!(content.contains("0 0 0 0 0 1 255 0 0 255\n"));
    }

    #[test]
    fn test_write_triangulation_rejects_mismatched_normals() {
        let provider = DeplyProvider::new();
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_vertex(1.0, 0.0, 0.0);
        tri.add_normal(0.0, 0.0, 1.0); // only one normal for two vertices

        let path = temp_path("badnormals");
        let result = provider.write_triangulation(&path.to_string_lossy(), &tri);
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_triangulation_rejects_out_of_range_face() {
        let provider = DeplyProvider::new();
        let mut tri = MeshTriangulation::new();
        tri.add_vertex(0.0, 0.0, 0.0);
        tri.add_face(0, 1, 2);

        let path = temp_path("badface");
        let result = provider.write_triangulation(&path.to_string_lossy(), &tri);
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_empty_triangulation() {
        let provider = DeplyProvider::from_config_node(DeplyConfigNodeRef);
        let tri = MeshTriangulation::new();
        let path = temp_path("empty");
        provider
            .write_triangulation(&path.to_string_lossy(), &tri)
            .unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert!(content.contains("element vertex 0\n"));
        assert!(content.contains("element face 0\n"));
    }
}
