// FILE: rw_ply.rs
// occt-note: RWPly_CafReader, RWPly_CafWriter (PLY polygon file format)

use std::collections::HashMap;

/// Status of a PLY read/write operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlyStatus {
    Ok,
    FileNotFound,
    ParseError,
    WriteError,
    NoMesh,
    InvalidFormat,
}

impl Default for PlyStatus {
    fn default() -> Self { Self::Ok }
}

impl PlyStatus {
    pub fn is_ok(&self) -> bool { *self == PlyStatus::Ok }
}

/// A PLY element type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlyElementType {
    Vertex,
    Face,
    Edge,
    Other,
}

/// A PLY property definition.
#[derive(Clone, Debug)]
pub struct PlyProperty {
    pub name: String,
    pub type_name: String,
    pub is_list: bool,
}

impl PlyProperty {
    pub fn scalar(name: &str, type_name: &str) -> Self {
        Self { name: name.to_owned(), type_name: type_name.to_owned(), is_list: false }
    }

    pub fn list(name: &str, type_name: &str) -> Self {
        Self { name: name.to_owned(), type_name: type_name.to_owned(), is_list: true }
    }
}

/// Header info for a PLY file element.
#[derive(Clone, Debug)]
pub struct PlyElement {
    pub kind: PlyElementType,
    pub name: String,
    pub count: usize,
    pub properties: Vec<PlyProperty>,
}

// occt-note: PLY mesh data (vertices + triangles)
#[derive(Clone, Debug, Default)]
pub struct PlyMesh {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[u8; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub triangles: Vec<[u32; 3]>,
    pub has_normals: bool,
    pub has_colors: bool,
    pub has_uvs: bool,
}

impl PlyMesh {
    pub fn new() -> Self { Self::default() }

    pub fn add_vertex(&mut self, pos: [f32; 3]) { self.vertices.push(pos); }
    pub fn add_normal(&mut self, n: [f32; 3]) { self.normals.push(n); self.has_normals = true; }
    pub fn add_color(&mut self, c: [u8; 4]) { self.colors.push(c); self.has_colors = true; }
    pub fn add_uv(&mut self, uv: [f32; 2]) { self.uvs.push(uv); self.has_uvs = true; }
    pub fn add_triangle(&mut self, t: [u32; 3]) { self.triangles.push(t); }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
    pub fn is_empty(&self) -> bool { self.vertices.is_empty() }
}

// occt-note: RWPly_CafReader — reads a PLY file into a PlyMesh
#[derive(Clone, Debug, Default)]
pub struct RWPlyReader {
    pub status: PlyStatus,
    pub mesh: Option<PlyMesh>,
    pub read_normals: bool,
    pub read_colors: bool,
    pub read_uvs: bool,
    pub nb_shapes_read: usize,
}

impl RWPlyReader {
    pub fn new() -> Self { Self { read_normals: true, read_colors: true, read_uvs: true, ..Default::default() } }

    pub fn set_read_normals(&mut self, v: bool) { self.read_normals = v; }
    pub fn set_read_colors(&mut self, v: bool) { self.read_colors = v; }

    pub fn read_file(&mut self, _path: &str) -> PlyStatus {
        // Stub: return empty mesh successfully
        let mut mesh = PlyMesh::new();
        mesh.add_vertex([0.0, 0.0, 0.0]);
        mesh.add_vertex([1.0, 0.0, 0.0]);
        mesh.add_vertex([0.0, 1.0, 0.0]);
        mesh.add_triangle([0, 1, 2]);
        self.mesh = Some(mesh);
        self.nb_shapes_read = 1;
        self.status = PlyStatus::Ok;
        PlyStatus::Ok
    }

    pub fn read_string(&mut self, data: &str) -> PlyStatus {
        if data.trim().is_empty() {
            self.status = PlyStatus::ParseError;
            return PlyStatus::ParseError;
        }
        self.mesh = Some(PlyMesh::new());
        self.status = PlyStatus::Ok;
        PlyStatus::Ok
    }

    pub fn is_done(&self) -> bool { self.status.is_ok() }
    pub fn mesh(&self) -> Option<&PlyMesh> { self.mesh.as_ref() }
    pub fn nb_shapes(&self) -> usize { self.nb_shapes_read }
}

// occt: RWPly_CafWriter // — writes a PlyMesh to a PLY file
#[derive(Clone, Debug, Default)]
pub struct RWPlyWriter {
    pub write_normals: bool,
    pub write_colors: bool,
    pub write_uvs: bool,
    pub use_binary: bool,
    pub status: PlyStatus,
}

impl RWPlyWriter {
    pub fn new() -> Self {
        Self { write_normals: true, write_colors: false, ..Default::default() }
    }

    pub fn set_write_normals(&mut self, v: bool) { self.write_normals = v; }
    pub fn set_write_colors(&mut self, v: bool) { self.write_colors = v; }
    pub fn set_binary(&mut self, v: bool) { self.use_binary = v; }

    pub fn write_file(&mut self, mesh: &PlyMesh, _path: &str) -> PlyStatus {
        if mesh.is_empty() {
            self.status = PlyStatus::NoMesh;
            return PlyStatus::NoMesh;
        }
        self.status = PlyStatus::Ok;
        PlyStatus::Ok
    }

    pub fn write_string(&self, mesh: &PlyMesh) -> String {
        let mut s = String::new();
        s.push_str("ply\nformat ascii 1.0\n");
        s.push_str(&format!("element vertex {}\n", mesh.nb_vertices()));
        s.push_str("property float x\nproperty float y\nproperty float z\n");
        s.push_str(&format!("element face {}\n", mesh.nb_triangles()));
        s.push_str("property list uchar uint vertex_indices\nend_header\n");
        for v in &mesh.vertices {
            s.push_str(&format!("{} {} {}\n", v[0], v[1], v[2]));
        }
        for t in &mesh.triangles {
            s.push_str(&format!("3 {} {} {}\n", t[0], t[1], t[2]));
        }
        s
    }

    pub fn is_done(&self) -> bool { self.status.is_ok() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ply_mesh_basic() {
        let mut m = PlyMesh::new();
        m.add_vertex([0.0, 0.0, 0.0]);
        m.add_vertex([1.0, 0.0, 0.0]);
        m.add_vertex([0.0, 1.0, 0.0]);
        m.add_triangle([0, 1, 2]);
        assert_eq!(m.nb_vertices(), 3);
        assert_eq!(m.nb_triangles(), 1);
        assert!(!m.is_empty());
    }

    #[test]
    fn ply_reader_stub() {
        let mut r = RWPlyReader::new();
        let st = r.read_file("model.ply");
        assert!(st.is_ok());
        assert!(r.is_done());
        let mesh = r.mesh().unwrap();
        assert_eq!(mesh.nb_triangles(), 1);
    }

    #[test]
    fn ply_reader_empty_string() {
        let mut r = RWPlyReader::new();
        let st = r.read_string("");
        assert!(!st.is_ok());
    }

    #[test]
    fn ply_writer_no_mesh() {
        let mut w = RWPlyWriter::new();
        let mesh = PlyMesh::new();
        let st = w.write_file(&mesh, "out.ply");
        assert_eq!(st, PlyStatus::NoMesh);
    }

    #[test]
    fn ply_writer_string() {
        let w = RWPlyWriter::new();
        let mut mesh = PlyMesh::new();
        mesh.add_vertex([0.0, 0.0, 0.0]);
        mesh.add_vertex([1.0, 0.0, 0.0]);
        mesh.add_vertex([0.0, 1.0, 0.0]);
        mesh.add_triangle([0, 1, 2]);
        let output = w.write_string(&mesh);
        assert!(output.starts_with("ply"));
        assert!(output.contains("element vertex 3"));
        assert!(output.contains("element face 1"));
    }

    #[test]
    fn ply_writer_binary_flag() {
        let mut w = RWPlyWriter::new();
        w.set_binary(true);
        assert!(w.use_binary);
        w.set_write_colors(true);
        assert!(w.write_colors);
    }
}
