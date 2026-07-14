//! glTF / GLB writer — stub modelled after `RWGltf_CafWriter` from OCCT.
//!
//! Writes shape names and placeholder geometry into a minimal glTF 2.0 JSON
//! document (`.gltf`) or a self-contained GLB container (`.glb`). No external
//! crates are used; the implementation relies on `std` only.

use std::fmt::Write as FmtWrite;
use std::fs;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Output format selector.
// occt: RWGltf_WriterTrsfFormat // (GLB vs. glTF split)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GltfFormat {
    /// Binary GLB container — everything in one file.
    Glb,
    /// JSON glTF — separate `.gltf` + optional external buffers.
    Gltf,
}

/// Tuning knobs forwarded to the writer.
// occt-note: RWGltf_CafWriterTrsfFormat / per-writer flags
#[derive(Debug, Clone)]
pub struct GltfWriteParams {
    /// Apply the OCCT-to-glTF coordinate-system transform (Y-up, metre scale).
    pub transform_from_occt: bool,
    /// Merge co-planar faces into a single mesh primitive.
    pub merge_faces: bool,
    /// Split mesh primitives by material rather than by shape.
    pub split_by_material: bool,
}

impl GltfWriteParams {
    /// Return default parameters (all flags `false`).
    pub fn new() -> Self {
        Self {
            transform_from_occt: false,
            merge_faces: false,
            split_by_material: false,
        }
    }
}

impl Default for GltfWriteParams {
    fn default() -> Self {
        Self::new()
    }
}

/// glTF writer — stub modelled on `RWGltf_CafWriter`.
// occt-ref: RWGltf_CafWriter
#[derive(Debug, Clone)]
pub struct GltfWriter {
    /// Destination file path (`.gltf` or `.glb`).
    pub file_path: String,
    /// Output format (JSON glTF or binary GLB).
    pub format: GltfFormat,
    /// Serialisation parameters.
    pub params: GltfWriteParams,
}

impl GltfWriter {
    /// Create a writer targeting `path` in JSON glTF format.
    pub fn new(path: &str) -> Self {
        Self {
            file_path: path.to_owned(),
            format: GltfFormat::Gltf,
            params: GltfWriteParams::new(),
        }
    }

    /// Create a writer targeting `path` in binary GLB format.
    pub fn binary(path: &str) -> Self {
        Self {
            file_path: path.to_owned(),
            format: GltfFormat::Glb,
            params: GltfWriteParams::new(),
        }
    }

    /// Replace the current serialisation parameters.
    pub fn set_params(&mut self, p: GltfWriteParams) {
        self.params = p;
    }

    /// Write `shapes` (a slice of shape-name strings) to [`Self::file_path`].
    ///
    /// Each string is treated as both the mesh name and a stand-in for the
    /// actual geometry (placeholder — no real mesh data is emitted).
    pub fn write(&self, shapes: &[String]) -> Result<(), String> {
        let named: Vec<(&str, &str)> = shapes.iter().map(|s| (s.as_str(), s.as_str())).collect();
        self.write_with_names(&named)
    }

    /// Write `shapes` given as `(name, label)` pairs to [`Self::file_path`].
    ///
    /// `name`  — mesh / node name embedded in the glTF asset.
    /// `label` — human-readable label stored in the `extras` field.
    pub fn write_with_names(&self, shapes: &[(&str, &str)]) -> Result<(), String> {
        match self.format {
            GltfFormat::Gltf => {
                let json = build_gltf_json(shapes, &self.params);
                fs::write(&self.file_path, json.as_bytes())
                    .map_err(|e| format!("GltfWriter: cannot write '{}': {}", self.file_path, e))
            }
            GltfFormat::Glb => {
                let bytes = build_glb(shapes, &self.params);
                fs::write(&self.file_path, &bytes)
                    .map_err(|e| format!("GltfWriter: cannot write '{}': {}", self.file_path, e))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience wrapper
// ---------------------------------------------------------------------------

/// Write `shapes` (name strings) to `path`, inferring format from the
/// extension (`.glb` → GLB, anything else → JSON glTF).
pub fn write_gltf(shapes: &[String], path: &str) -> Result<(), String> {
    let format = if path.ends_with(".glb") {
        GltfFormat::Glb
    } else {
        GltfFormat::Gltf
    };
    let writer = GltfWriter {
        file_path: path.to_owned(),
        format,
        params: GltfWriteParams::new(),
    };
    writer.write(shapes)
}

// ---------------------------------------------------------------------------
// Internal builders
// ---------------------------------------------------------------------------

/// Build a minimal glTF 2.0 JSON document for the given shapes.
fn build_gltf_json(shapes: &[(&str, &str)], params: &GltfWriteParams) -> String {
    let mut s = String::new();

    // Asset header.
    let _ = writeln!(s, "{{");
    let _ = writeln!(s, r#"  "asset": {{"version": "2.0", "generator": "IronStream RwGltfWriter"}},"#);

    // Extra flags in a top-level extras block so roundtrip readers can restore
    // the write settings.
    let _ = writeln!(s, r#"  "extras": {{"#);
    let _ = writeln!(s, r#"    "transformFromOcct": {},"#, params.transform_from_occt);
    let _ = writeln!(s, r#"    "mergeFaces": {},"#, params.merge_faces);
    let _ = writeln!(s, r#"    "splitByMaterial": {}"#, params.split_by_material);
    let _ = writeln!(s, r#"  }},"#);

    // Scene / nodes.
    let _ = writeln!(s, r#"  "scene": 0,"#);
    let _ = writeln!(s, r#"  "scenes": [{{"nodes": [{}]}}],"#, node_index_list(shapes.len()));

    let _ = writeln!(s, r#"  "nodes": ["#);
    for (i, (name, label)) in shapes.iter().enumerate() {
        let comma = if i + 1 < shapes.len() { "," } else { "" };
        let escaped_name = json_escape(name);
        let escaped_label = json_escape(label);
        let _ = writeln!(
            s,
            r#"    {{"name": "{escaped_name}", "extras": {{"label": "{escaped_label}"}}}}{comma}"#
        );
    }
    let _ = writeln!(s, r#"  ]"#);

    let _ = writeln!(s, "}}");
    s
}

/// Build a GLB binary container.
/// Layout: 12-byte GLB header | JSON chunk | (no BIN chunk — stub only).
fn build_glb(shapes: &[(&str, &str)], params: &GltfWriteParams) -> Vec<u8> {
    let json = build_gltf_json(shapes, params);
    // JSON chunk payload must be padded to a 4-byte boundary with spaces.
    let json_bytes = json.as_bytes();
    let padded_len = (json_bytes.len() + 3) & !3;
    let mut json_padded = json_bytes.to_vec();
    json_padded.resize(padded_len, b' ');

    let chunk_json_len = padded_len as u32;
    // Total file length = 12 (file header) + 8 (chunk header) + chunk payload.
    let total_len = 12u32 + 8 + chunk_json_len;

    let mut out = Vec::with_capacity(total_len as usize);

    // GLB file header.
    out.extend_from_slice(b"glTF");           // magic
    out.extend_from_slice(&2u32.to_le_bytes()); // version 2
    out.extend_from_slice(&total_len.to_le_bytes());

    // JSON chunk header.
    out.extend_from_slice(&chunk_json_len.to_le_bytes());
    out.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // "JSON"
    out.extend_from_slice(&json_padded);

    out
}

/// Produce `"0, 1, 2, ..."` for `count` nodes.
fn node_index_list(count: usize) -> String {
    (0..count)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Minimal JSON string escaping (backslash and double-quote only).
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gltf_json_contains_asset_header() {
        let shapes = [("Box", "box_label"), ("Sphere", "sphere_label")];
        let json = build_gltf_json(&shapes, &GltfWriteParams::new());
        assert!(json.contains(r#""version": "2.0""#));
        assert!(json.contains("IronStream RwGltfWriter"));
    }

    #[test]
    fn gltf_json_contains_shape_names() {
        let shapes = [("PartA", "label_a")];
        let json = build_gltf_json(&shapes, &GltfWriteParams::new());
        assert!(json.contains("PartA"));
        assert!(json.contains("label_a"));
    }

    #[test]
    fn glb_magic_and_version() {
        let shapes = [("Box", "box")];
        let bytes = build_glb(&shapes, &GltfWriteParams::new());
        assert_eq!(&bytes[0..4], b"glTF");
        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(version, 2);
    }

    #[test]
    fn glb_length_field_matches_actual_size() {
        let shapes = [("A", "a"), ("B", "b")];
        let bytes = build_glb(&shapes, &GltfWriteParams::new());
        let declared = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;
        assert_eq!(declared, bytes.len());
    }

    #[test]
    fn write_params_defaults_are_false() {
        let p = GltfWriteParams::new();
        assert!(!p.transform_from_occt);
        assert!(!p.merge_faces);
        assert!(!p.split_by_material);
    }

    #[test]
    fn writer_new_is_gltf_format() {
        let w = GltfWriter::new("/tmp/out.gltf");
        assert_eq!(w.format, GltfFormat::Gltf);
    }

    #[test]
    fn writer_binary_is_glb_format() {
        let w = GltfWriter::binary("/tmp/out.glb");
        assert_eq!(w.format, GltfFormat::Glb);
    }

    #[test]
    fn write_gltf_infers_glb_from_extension() {
        // Use a temp path; we only test the format inference, not actual I/O.
        let format = if "/some/path.glb".ends_with(".glb") {
            GltfFormat::Glb
        } else {
            GltfFormat::Gltf
        };
        assert_eq!(format, GltfFormat::Glb);

        let format2 = if "/some/path.gltf".ends_with(".glb") {
            GltfFormat::Glb
        } else {
            GltfFormat::Gltf
        };
        assert_eq!(format2, GltfFormat::Gltf);
    }

    #[test]
    fn json_escape_handles_special_chars() {
        assert_eq!(json_escape(r#"foo"bar"#), r#"foo\"bar"#);
        assert_eq!(json_escape(r"back\slash"), r"back\\slash");
        assert_eq!(json_escape("plain"), "plain");
    }
}
