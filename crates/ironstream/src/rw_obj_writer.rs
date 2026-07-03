//! OBJ / MTL writer — stub modelled after `RWObj_CafWriter` from OCCT.
//!
//! Serialises triangle-mesh geometry into Wavefront OBJ (`.obj`) and the
//! companion material library format (`.mtl`).  No external crates are used;
//! the implementation relies on `std` only.
//!
//! # OCCT correspondence
//!
//! | OCCT type          | IronStream type  |
//! |--------------------|------------------|
//! | `RWObj_CafWriter`  | [`ObjWriter`]    |
//! | *(mtl helper)*     | [`MtlWriter`]    |

use std::fmt::Write as FmtWrite;
use std::fs;

// ---------------------------------------------------------------------------
// ObjWriter
// ---------------------------------------------------------------------------

/// OBJ mesh writer — stub modelled on `RWObj_CafWriter`.
// occt: RWObj_CafWriter
#[derive(Debug, Clone)]
pub struct ObjWriter {
    /// Destination `.obj` file path.
    pub file_path: String,
    /// Coordinate-system convention identifier (0 = default / OCCT Z-up).
    pub coordinate_system: u8,
}

impl ObjWriter {
    /// Create a writer targeting `path` with the default coordinate system (0).
    pub fn new(path: &str) -> Self {
        Self {
            file_path: path.to_owned(),
            coordinate_system: 0,
        }
    }

    /// Return a copy of the writer with the coordinate system set to `cs`.
    pub fn with_coord_system(mut self, cs: u8) -> Self {
        self.coordinate_system = cs;
        self
    }

    /// Write a triangle mesh to [`Self::file_path`].
    ///
    /// `nodes`   — vertex positions (one `v` statement each).
    /// `tris`    — triangle index triples, 0-based (one `f` statement each).
    /// `normals` — optional per-vertex normals; when `Some` a matching `vn`
    ///             statement is emitted for each vertex and face indices gain
    ///             the `v//vn` form.
    ///
    /// Indices in `tris` must be valid (< `nodes.len()`); no bounds check is
    /// performed at this level.
    pub fn write(
        &self,
        nodes: &[[f64; 3]],
        tris: &[[u32; 3]],
        normals: Option<&[[f64; 3]]>,
    ) -> Result<(), String> {
        let content = build_obj(nodes, tris, normals, None);
        fs::write(&self.file_path, content.as_bytes()).map_err(|e| {
            format!(
                "ObjWriter: cannot write '{}': {}",
                self.file_path, e
            )
        })
    }

    /// Write a triangle mesh to [`Self::file_path`] with a material library
    /// reference.
    ///
    /// `mtl` is the base filename of the `.mtl` file (e.g. `"model.mtl"`).  A
    /// `mtllib` directive is prepended so OBJ viewers can locate the materials.
    /// Triangle indices reference the single unnamed material; callers should
    /// write a matching MTL file via [`MtlWriter`].
    pub fn write_with_materials(
        &self,
        nodes: &[[f64; 3]],
        tris: &[[u32; 3]],
        mtl: &str,
    ) -> Result<(), String> {
        let content = build_obj(nodes, tris, None, Some(mtl));
        fs::write(&self.file_path, content.as_bytes()).map_err(|e| {
            format!(
                "ObjWriter: cannot write '{}': {}",
                self.file_path, e
            )
        })
    }
}

// ---------------------------------------------------------------------------
// MtlWriter
// ---------------------------------------------------------------------------

/// MTL material-library writer.
///
/// Produces Wavefront `.mtl` files consumed by [`ObjWriter::write_with_materials`].
#[derive(Debug, Clone)]
pub struct MtlWriter {
    /// Destination `.mtl` file path.
    pub file_path: String,
}

impl MtlWriter {
    /// Create a writer targeting `path`.
    pub fn new(path: &str) -> Self {
        Self {
            file_path: path.to_owned(),
        }
    }

    /// Write a single Phong material entry to [`Self::file_path`].
    ///
    /// `name` — material name (`newmtl` value).
    /// `kd`   — diffuse colour RGB in \[0, 1\].
    /// `ks`   — specular colour RGB in \[0, 1\].
    /// `ns`   — specular exponent (`Ns`), typically in \[0, 1000\].
    ///
    /// The file is overwritten on each call; call once per MTL file.
    pub fn write_material(
        &self,
        name: &str,
        kd: [f64; 3],
        ks: [f64; 3],
        ns: f64,
    ) -> Result<(), String> {
        let content = build_mtl(name, kd, ks, ns);
        fs::write(&self.file_path, content.as_bytes()).map_err(|e| {
            format!(
                "MtlWriter: cannot write '{}': {}",
                self.file_path, e
            )
        })
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience wrapper
// ---------------------------------------------------------------------------

/// Write a triangle mesh to `path` (Wavefront OBJ, no normals, no materials).
///
/// This is a thin convenience wrapper around [`ObjWriter::write`].
pub fn write_obj(nodes: &[[f64; 3]], tris: &[[u32; 3]], path: &str) -> Result<(), String> {
    ObjWriter::new(path).write(nodes, tris, None)
}

// ---------------------------------------------------------------------------
// Internal builders
// ---------------------------------------------------------------------------

/// Build the complete text content of a `.obj` file.
fn build_obj(
    nodes: &[[f64; 3]],
    tris: &[[u32; 3]],
    normals: Option<&[[f64; 3]]>,
    mtl_file: Option<&str>,
) -> String {
    let mut s = String::new();

    let _ = writeln!(s, "# Generated by IronStream ObjWriter");

    if let Some(mtl) = mtl_file {
        let _ = writeln!(s, "mtllib {}", mtl);
    }

    // Vertex positions.
    for v in nodes {
        let _ = writeln!(s, "v {} {} {}", v[0], v[1], v[2]);
    }

    // Vertex normals (optional).
    if let Some(ns) = normals {
        for n in ns {
            let _ = writeln!(s, "vn {} {} {}", n[0], n[1], n[2]);
        }
    }

    // Faces — OBJ indices are 1-based.
    if normals.is_some() {
        // v//vn form when normals are present.
        for tri in tris {
            let (a, b, c) = (tri[0] + 1, tri[1] + 1, tri[2] + 1);
            let _ = writeln!(s, "f {a}//{a} {b}//{b} {c}//{c}");
        }
    } else {
        for tri in tris {
            let _ = writeln!(s, "f {} {} {}", tri[0] + 1, tri[1] + 1, tri[2] + 1);
        }
    }

    s
}

/// Build the complete text content of a `.mtl` file for a single material.
fn build_mtl(name: &str, kd: [f64; 3], ks: [f64; 3], ns: f64) -> String {
    let mut s = String::new();

    let _ = writeln!(s, "# Generated by IronStream MtlWriter");
    let _ = writeln!(s, "newmtl {}", name);
    let _ = writeln!(s, "Kd {} {} {}", kd[0], kd[1], kd[2]);
    let _ = writeln!(s, "Ks {} {} {}", ks[0], ks[1], ks[2]);
    let _ = writeln!(s, "Ns {}", ns);

    s
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- build_obj ---

    #[test]
    fn obj_header_comment() {
        let s = build_obj(&[], &[], None, None);
        assert!(s.contains("IronStream ObjWriter"));
    }

    #[test]
    fn obj_vertex_statements() {
        let nodes = [[1.0_f64, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let s = build_obj(&nodes, &[], None, None);
        assert!(s.contains("v 1 2 3"));
        assert!(s.contains("v 4 5 6"));
    }

    #[test]
    fn obj_face_one_based_indices() {
        let nodes = [[0.0_f64; 3]; 3];
        let tris = [[0u32, 1, 2]];
        let s = build_obj(&nodes, &tris, None, None);
        assert!(s.contains("f 1 2 3"));
    }

    #[test]
    fn obj_face_with_normals_uses_double_slash() {
        let nodes = [[0.0_f64; 3]; 3];
        let tris = [[0u32, 1, 2]];
        let normals = [[0.0_f64, 0.0, 1.0]; 3];
        let s = build_obj(&nodes, &tris, Some(&normals), None);
        assert!(s.contains("f 1//1 2//2 3//3"));
    }

    #[test]
    fn obj_normal_statements_present() {
        let nodes = [[0.0_f64; 3]; 1];
        let normals = [[0.0_f64, 0.0, 1.0]];
        let s = build_obj(&nodes, &[], Some(&normals), None);
        assert!(s.contains("vn 0 0 1"));
    }

    #[test]
    fn obj_mtllib_directive() {
        let s = build_obj(&[], &[], None, Some("model.mtl"));
        assert!(s.contains("mtllib model.mtl"));
    }

    #[test]
    fn obj_no_mtllib_when_none() {
        let s = build_obj(&[], &[], None, None);
        assert!(!s.contains("mtllib"));
    }

    // --- build_mtl ---

    #[test]
    fn mtl_newmtl_line() {
        let s = build_mtl("Steel", [0.8, 0.8, 0.8], [0.5, 0.5, 0.5], 96.0);
        assert!(s.contains("newmtl Steel"));
    }

    #[test]
    fn mtl_kd_ks_ns() {
        let s = build_mtl("Mat", [0.1, 0.2, 0.3], [0.4, 0.5, 0.6], 32.0);
        assert!(s.contains("Kd 0.1 0.2 0.3"));
        assert!(s.contains("Ks 0.4 0.5 0.6"));
        assert!(s.contains("Ns 32"));
    }

    // --- ObjWriter ---

    #[test]
    fn obj_writer_new_defaults() {
        let w = ObjWriter::new("/tmp/out.obj");
        assert_eq!(w.file_path, "/tmp/out.obj");
        assert_eq!(w.coordinate_system, 0);
    }

    #[test]
    fn obj_writer_with_coord_system() {
        let w = ObjWriter::new("/tmp/out.obj").with_coord_system(2);
        assert_eq!(w.coordinate_system, 2);
    }

    // --- MtlWriter ---

    #[test]
    fn mtl_writer_new_stores_path() {
        let w = MtlWriter::new("/tmp/model.mtl");
        assert_eq!(w.file_path, "/tmp/model.mtl");
    }

    // --- write_obj free function ---

    #[test]
    fn write_obj_roundtrip() {
        use std::io::Read;

        let dir = std::env::temp_dir();
        let path = dir.join("ironstream_test_write_obj.obj");
        let path_str = path.to_str().unwrap();

        let nodes = [[0.0_f64, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let tris = [[0u32, 1, 2]];
        write_obj(&nodes, &tris, path_str).expect("write_obj failed");

        let mut content = String::new();
        std::fs::File::open(&path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("v 0 0 0"));
        assert!(content.contains("v 1 0 0"));
        assert!(content.contains("v 0 1 0"));
        assert!(content.contains("f 1 2 3"));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn write_obj_error_on_bad_path() {
        let result = write_obj(&[], &[], "/nonexistent_dir/out.obj");
        assert!(result.is_err());
    }
}
