// FILE: dexcaf_provider.rs
// occt: DEXCAF_Provider

//! Provider for reading and writing XDE/XCAF document files.
//! Mirrors OCCT `DEXCAF_Provider` (DataExchange/TKDECascade/DEXCAF):
//! vendor name "OCC", format "XCAF"; both import and export supported.
//!
//! External OCCT plumbing (TDocStd_Document, PCDM storage drivers,
//! XCAFDoc_ShapeTool) is modeled with local helper types; the provider's
//! own behavior — persisting a labelled document (and shapes stored under
//! labels) to a file and restoring it — is implemented for real using a
//! simple line-based serialization.

use std::fs;

/// Provider for reading and writing XDE/XCAF document files.
/// Handles XBF format import/export operations with configuration from DEXCAF_ConfigurationNode.
///
/// Vendor: OCC
/// Format: XCAF
/// Supports document-based and shape-based read/write operations.
#[derive(Clone)]
pub struct DexcafProvider {
    /// Configuration node for this provider
    pub config_node: Option<DexcafConfigNodeRef>,
}

/// Reference to a XCAF configuration node
#[derive(Clone, Debug, Default)]
pub struct DexcafConfigNodeRef;

const MAGIC: &str = "XCAF-DOC";

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\t', "\\t").replace('\n', "\\n")
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

impl DexcafProvider {
    /// Creates a new provider with default configuration
    pub fn new() -> Self {
        DexcafProvider {
            config_node: None,
        }
    }

    /// Creates a provider with the specified configuration node
    pub fn from_config_node(node: DexcafConfigNodeRef) -> Self {
        DexcafProvider {
            config_node: Some(node),
        }
    }

    /// Gets the CAD format name
    pub fn get_format(&self) -> &'static str {
        "XCAF"
    }

    /// Gets the vendor name
    pub fn get_vendor(&self) -> &'static str {
        "OCC"
    }

    /// Reads an XCAF document from the specified path
    /// (mirrors `Read(path, document, ...)`).
    pub fn read_document(&self, file_path: &str) -> Result<XcafDocument, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("DEXCAF_Provider: cannot read file '{}': {}", file_path, e))?;
        let mut lines = content.lines();

        // Header: "XCAF-DOC <version>"
        let header = lines
            .next()
            .ok_or_else(|| "DEXCAF_Provider: empty document file".to_string())?;
        let mut header_parts = header.splitn(2, '\t');
        if header_parts.next() != Some(MAGIC) {
            return Err(format!(
                "DEXCAF_Provider: '{}' is not an XCAF document",
                file_path
            ));
        }
        let version = header_parts.next().unwrap_or("1.0").to_string();

        let mut doc = XcafDocument::new();
        doc.metadata.version = version;

        for (line_no, line) in lines.enumerate() {
            if line.is_empty() {
                continue;
            }
            let mut parts = line.splitn(3, '\t');
            match parts.next() {
                Some("created") => {
                    doc.metadata.created = unescape(parts.next().unwrap_or(""));
                }
                Some("label") => {
                    let path = parts.next().ok_or_else(|| {
                        format!("DEXCAF_Provider: label without path at line {}", line_no + 2)
                    })?;
                    doc.add_label(unescape(path));
                }
                Some("attr") => {
                    if doc.labels.is_empty() {
                        return Err(format!(
                            "DEXCAF_Provider: attribute before any label at line {}",
                            line_no + 2
                        ));
                    }
                    let name = parts.next().ok_or_else(|| {
                        format!("DEXCAF_Provider: attr without name at line {}", line_no + 2)
                    })?;
                    let value = parts.next().unwrap_or("");
                    doc.add_attribute(unescape(name), unescape(value));
                }
                Some(other) => {
                    return Err(format!(
                        "DEXCAF_Provider: unknown record '{}' at line {}",
                        other,
                        line_no + 2
                    ));
                }
                None => {}
            }
        }
        Ok(doc)
    }

    /// Writes an XCAF document to the specified path
    /// (mirrors `Write(path, document, ...)`).
    pub fn write_document(&self, file_path: &str, doc: &XcafDocument) -> Result<(), String> {
        let mut out = String::new();
        out.push_str(&format!("{}\t{}\n", MAGIC, doc.metadata.version));
        if !doc.metadata.created.is_empty() {
            out.push_str(&format!("created\t{}\n", escape(&doc.metadata.created)));
        }
        for label in &doc.labels {
            out.push_str(&format!("label\t{}\n", escape(&label.path)));
            for (name, value) in &label.attributes {
                out.push_str(&format!("attr\t{}\t{}\n", escape(name), escape(value)));
            }
        }
        fs::write(file_path, out)
            .map_err(|e| format!("DEXCAF_Provider: cannot write file '{}': {}", file_path, e))
    }

    /// Reads a shape from an XCAF document file
    /// (mirrors `Read(path, shape, ...)`: reads the document, then extracts
    /// the shape stored under its labels).
    pub fn read_shape(&self, file_path: &str) -> Result<Shape, String> {
        let doc = self.read_document(file_path)?;
        let mut shape = Shape::new();
        let parse3 = |v: &str| -> Result<(f64, f64, f64), String> {
            let nums: Vec<f64> = v
                .split_whitespace()
                .map(|t| t.parse::<f64>())
                .collect::<Result<_, _>>()
                .map_err(|e| format!("DEXCAF_Provider: bad coordinate triple '{}': {}", v, e))?;
            if nums.len() != 3 {
                return Err(format!("DEXCAF_Provider: expected 3 numbers in '{}'", v));
            }
            Ok((nums[0], nums[1], nums[2]))
        };
        for label in &doc.labels {
            for (name, value) in &label.attributes {
                match name.as_str() {
                    "vertex" => {
                        let (x, y, z) = parse3(value)?;
                        shape.add_vertex(x, y, z);
                    }
                    "edge" => {
                        let idx: Vec<usize> = value
                            .split_whitespace()
                            .map(|t| t.parse::<usize>())
                            .collect::<Result<_, _>>()
                            .map_err(|e| format!("DEXCAF_Provider: bad edge '{}': {}", value, e))?;
                        if idx.len() != 2 {
                            return Err(format!("DEXCAF_Provider: bad edge '{}'", value));
                        }
                        shape.add_edge(idx[0], idx[1]);
                    }
                    "face" => {
                        let idx: Vec<usize> = value
                            .split_whitespace()
                            .map(|t| t.parse::<usize>())
                            .collect::<Result<_, _>>()
                            .map_err(|e| format!("DEXCAF_Provider: bad face '{}': {}", value, e))?;
                        if idx.len() != 3 {
                            return Err(format!("DEXCAF_Provider: bad face '{}'", value));
                        }
                        shape.add_face(idx[0], idx[1], idx[2]);
                    }
                    _ => {}
                }
            }
        }
        Ok(shape)
    }

    /// Writes a shape to an XCAF document file
    /// (mirrors `Write(path, shape, ...)`: stores the shape under a
    /// document label, then writes the document).
    pub fn write_shape(&self, file_path: &str, shape: &Shape) -> Result<(), String> {
        let n = shape.vertex_count();
        for &(v1, v2) in &shape.edges {
            if v1 >= n || v2 >= n {
                return Err("DEXCAF_Provider: edge references vertex out of range".to_string());
            }
        }
        for &(v1, v2, v3) in &shape.faces {
            if v1 >= n || v2 >= n || v3 >= n {
                return Err("DEXCAF_Provider: face references vertex out of range".to_string());
            }
        }
        let mut doc = XcafDocument::new();
        doc.add_label("0:1:1".to_string());
        for &(x, y, z) in &shape.vertices {
            doc.add_attribute("vertex".to_string(), format!("{} {} {}", x, y, z));
        }
        for &(v1, v2) in &shape.edges {
            doc.add_attribute("edge".to_string(), format!("{} {}", v1, v2));
        }
        for &(v1, v2, v3) in &shape.faces {
            doc.add_attribute("face".to_string(), format!("{} {} {}", v1, v2, v3));
        }
        self.write_document(file_path, &doc)
    }
}

impl Default for DexcafProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an XCAF document structure
#[derive(Clone, Debug)]
pub struct XcafDocument {
    /// Label entries in the document
    pub labels: Vec<LabelEntry>,
    /// Document metadata
    pub metadata: DocumentMetadata,
}

/// A label entry in an XCAF document
#[derive(Clone, Debug)]
pub struct LabelEntry {
    /// Label path (e.g., "0:1:2")
    pub path: String,
    /// Attributes of this label
    pub attributes: Vec<(String, String)>,
}

/// Document metadata
#[derive(Clone, Debug)]
pub struct DocumentMetadata {
    /// Document format version
    pub version: String,
    /// Creation timestamp
    pub created: String,
}

impl XcafDocument {
    /// Creates an empty XCAF document
    pub fn new() -> Self {
        XcafDocument {
            labels: Vec::new(),
            metadata: DocumentMetadata {
                version: "1.0".to_string(),
                created: String::new(),
            },
        }
    }

    /// Adds a label entry to the document
    pub fn add_label(&mut self, path: String) {
        self.labels.push(LabelEntry {
            path,
            attributes: Vec::new(),
        });
    }

    /// Adds an attribute to the last label
    pub fn add_attribute(&mut self, name: String, value: String) {
        if let Some(last_label) = self.labels.last_mut() {
            last_label.attributes.push((name, value));
        }
    }

    /// Returns the number of labels
    pub fn label_count(&self) -> usize {
        self.labels.len()
    }
}

impl Default for XcafDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a geometric shape
#[derive(Clone, Debug)]
pub struct Shape {
    /// Shape vertices
    pub vertices: Vec<(f64, f64, f64)>,
    /// Shape edges (pairs of vertex indices)
    pub edges: Vec<(usize, usize)>,
    /// Shape faces (triangles of vertex indices)
    pub faces: Vec<(usize, usize, usize)>,
}

impl Shape {
    /// Creates an empty shape
    pub fn new() -> Self {
        Shape {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    /// Adds a vertex to the shape
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        let idx = self.vertices.len();
        self.vertices.push((x, y, z));
        idx
    }

    /// Adds an edge to the shape
    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.edges.push((v1, v2));
    }

    /// Adds a face to the shape
    pub fn add_face(&mut self, v1: usize, v2: usize, v3: usize) {
        self.faces.push((v1, v2, v3));
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the number of faces
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = DexcafProvider::new();
        assert_eq!(provider.get_format(), "XCAF");
        assert_eq!(provider.get_vendor(), "OCC");
        assert!(provider.config_node.is_none());
    }

    #[test]
    fn test_provider_default() {
        let provider = DexcafProvider::default();
        assert_eq!(provider.get_format(), "XCAF");
        assert_eq!(provider.get_vendor(), "OCC");
    }

    fn temp_path(tag: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!(
            "dexcaf_provider_{}_{}_{}.xbf",
            tag,
            std::process::id(),
            nanos
        ));
        path
    }

    #[test]
    fn test_read_document_missing_file() {
        let provider = DexcafProvider::new();
        let result = provider.read_document("/nonexistent/dir/test.xbf");
        assert!(result.is_err());
    }

    #[test]
    fn test_document_roundtrip() {
        let provider = DexcafProvider::new();
        let mut doc = XcafDocument::new();
        doc.metadata.created = "2026-07-02T00:00:00".to_string();
        doc.add_label("0:1".to_string());
        doc.add_attribute("Name".to_string(), "MyPart".to_string());
        doc.add_attribute("Type".to_string(), "Solid".to_string());
        doc.add_label("0:2".to_string());
        doc.add_attribute("Name".to_string(), "Other Part".to_string());

        let path = temp_path("docrt");
        let path_str = path.to_string_lossy().into_owned();
        provider.write_document(&path_str, &doc).unwrap();
        let read_back = provider.read_document(&path_str).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(read_back.metadata.version, "1.0");
        assert_eq!(read_back.metadata.created, "2026-07-02T00:00:00");
        assert_eq!(read_back.label_count(), 2);
        assert_eq!(read_back.labels[0].path, "0:1");
        assert_eq!(
            read_back.labels[0].attributes,
            vec![
                ("Name".to_string(), "MyPart".to_string()),
                ("Type".to_string(), "Solid".to_string())
            ]
        );
        assert_eq!(read_back.labels[1].path, "0:2");
        assert_eq!(
            read_back.labels[1].attributes,
            vec![("Name".to_string(), "Other Part".to_string())]
        );
    }

    #[test]
    fn test_document_roundtrip_escaped_values() {
        let provider = DexcafProvider::new();
        let mut doc = XcafDocument::new();
        doc.add_label("0:1".to_string());
        doc.add_attribute("Note".to_string(), "line1\nline2\twith tab \\ backslash".to_string());

        let path = temp_path("escape");
        let path_str = path.to_string_lossy().into_owned();
        provider.write_document(&path_str, &doc).unwrap();
        let read_back = provider.read_document(&path_str).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(
            read_back.labels[0].attributes[0].1,
            "line1\nline2\twith tab \\ backslash"
        );
    }

    #[test]
    fn test_read_document_rejects_non_xcaf_file() {
        let provider = DexcafProvider::new();
        let path = temp_path("notxcaf");
        std::fs::write(&path, "definitely not an xcaf document\n").unwrap();
        let result = provider.read_document(&path.to_string_lossy());
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not an XCAF document"));
    }

    #[test]
    fn test_xcaf_document_creation() {
        let doc = XcafDocument::new();
        assert_eq!(doc.label_count(), 0);
        assert_eq!(doc.metadata.version, "1.0");
    }

    #[test]
    fn test_xcaf_document_add_label() {
        let mut doc = XcafDocument::new();
        doc.add_label("0:1".to_string());
        doc.add_label("0:2".to_string());

        assert_eq!(doc.label_count(), 2);
        assert_eq!(doc.labels[0].path, "0:1");
        assert_eq!(doc.labels[1].path, "0:2");
    }

    #[test]
    fn test_xcaf_document_add_attribute() {
        let mut doc = XcafDocument::new();
        doc.add_label("0:1".to_string());
        doc.add_attribute("Name".to_string(), "MyPart".to_string());
        doc.add_attribute("Type".to_string(), "Solid".to_string());

        assert_eq!(doc.labels[0].attributes.len(), 2);
        assert_eq!(doc.labels[0].attributes[0], ("Name".to_string(), "MyPart".to_string()));
    }

    #[test]
    fn test_shape_creation() {
        let shape = Shape::new();
        assert_eq!(shape.vertex_count(), 0);
        assert_eq!(shape.edge_count(), 0);
        assert_eq!(shape.face_count(), 0);
    }

    #[test]
    fn test_shape_add_vertex() {
        let mut shape = Shape::new();
        let idx1 = shape.add_vertex(0.0, 0.0, 0.0);
        let idx2 = shape.add_vertex(1.0, 0.0, 0.0);
        let idx3 = shape.add_vertex(0.0, 1.0, 0.0);

        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx3, 2);
        assert_eq!(shape.vertex_count(), 3);
    }

    #[test]
    fn test_shape_add_edge() {
        let mut shape = Shape::new();
        shape.add_vertex(0.0, 0.0, 0.0);
        shape.add_vertex(1.0, 0.0, 0.0);
        shape.add_edge(0, 1);

        assert_eq!(shape.edge_count(), 1);
        assert_eq!(shape.edges[0], (0, 1));
    }

    #[test]
    fn test_shape_add_face() {
        let mut shape = Shape::new();
        shape.add_vertex(0.0, 0.0, 0.0);
        shape.add_vertex(1.0, 0.0, 0.0);
        shape.add_vertex(0.0, 1.0, 0.0);
        shape.add_face(0, 1, 2);

        assert_eq!(shape.face_count(), 1);
        assert_eq!(shape.faces[0], (0, 1, 2));
    }

    #[test]
    fn test_shape_complete() {
        let mut shape = Shape::new();
        let v0 = shape.add_vertex(0.0, 0.0, 0.0);
        let v1 = shape.add_vertex(1.0, 0.0, 0.0);
        let v2 = shape.add_vertex(1.0, 1.0, 0.0);
        let v3 = shape.add_vertex(0.0, 1.0, 0.0);

        shape.add_edge(v0, v1);
        shape.add_edge(v1, v2);
        shape.add_edge(v2, v3);
        shape.add_edge(v3, v0);

        shape.add_face(v0, v1, v2);
        shape.add_face(v0, v2, v3);

        assert_eq!(shape.vertex_count(), 4);
        assert_eq!(shape.edge_count(), 4);
        assert_eq!(shape.face_count(), 2);
    }

    #[test]
    fn test_shape_roundtrip() {
        let provider = DexcafProvider::new();
        let mut shape = Shape::new();
        let v0 = shape.add_vertex(0.0, 0.0, 0.0);
        let v1 = shape.add_vertex(1.5, 0.0, 0.0);
        let v2 = shape.add_vertex(0.0, 2.5, 0.0);
        shape.add_edge(v0, v1);
        shape.add_edge(v1, v2);
        shape.add_face(v0, v1, v2);

        let path = temp_path("shapert");
        let path_str = path.to_string_lossy().into_owned();
        provider.write_shape(&path_str, &shape).unwrap();
        let read_back = provider.read_shape(&path_str).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(read_back.vertices, shape.vertices);
        assert_eq!(read_back.edges, shape.edges);
        assert_eq!(read_back.faces, shape.faces);
    }

    #[test]
    fn test_write_shape_rejects_out_of_range_indices() {
        let provider = DexcafProvider::new();
        let mut shape = Shape::new();
        shape.add_vertex(0.0, 0.0, 0.0);
        shape.add_face(0, 1, 2); // out of range

        let path = temp_path("badshape");
        let result = provider.write_shape(&path.to_string_lossy(), &shape);
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
    }
}
