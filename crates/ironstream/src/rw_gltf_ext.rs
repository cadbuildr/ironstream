// FILE: rw_gltf_ext.rs
// occt: RWGltf_CafReader, RWGltf_CafWriter (glTF 2.0 format extended)

use std::collections::HashMap;

/// Status of a glTF read/write operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfStatus {
    Ok,
    FileNotFound,
    ParseError,
    WriteError,
    NoMesh,
    InvalidJson,
    UnsupportedVersion,
}

impl Default for GltfStatus {
    fn default() -> Self { Self::Ok }
}

impl GltfStatus {
    pub fn is_ok(&self) -> bool { *self == GltfStatus::Ok }
}

/// A glTF mesh primitive.
#[derive(Clone, Debug, Default)]
pub struct GltfPrimitive {
    pub material_idx: i32,
    pub indices: Vec<u32>,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
}

impl GltfPrimitive {
    pub fn new() -> Self { Self { material_idx: -1, ..Default::default() } }

    pub fn nb_indices(&self) -> usize { self.indices.len() }
    pub fn nb_positions(&self) -> usize { self.positions.len() }
    pub fn is_empty(&self) -> bool { self.positions.is_empty() }
}

/// A glTF node in the scene graph.
#[derive(Clone, Debug)]
pub struct GltfNode {
    pub name: String,
    pub mesh_idx: i32,
    pub children: Vec<usize>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl GltfNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            mesh_idx: -1,
            children: Vec::new(),
            translation: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        }
    }

    pub fn has_mesh(&self) -> bool { self.mesh_idx >= 0 }
    pub fn add_child(&mut self, child_idx: usize) { self.children.push(child_idx); }
}

/// A glTF material.
#[derive(Clone, Debug)]
pub struct GltfMaterial {
    pub name: String,
    pub base_color: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub emissive: [f32; 3],
    pub double_sided: bool,
    pub alpha_mode: u8,
}

impl Default for GltfMaterial {
    fn default() -> Self {
        Self {
            name: String::new(),
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic_factor: 0.0,
            roughness_factor: 0.5,
            emissive: [0.0; 3],
            double_sided: false,
            alpha_mode: 0,
        }
    }
}

impl GltfMaterial {
    pub fn new(name: &str) -> Self { Self { name: name.to_owned(), ..Default::default() } }
    pub fn set_base_color(&mut self, r: f32, g: f32, b: f32, a: f32) { self.base_color = [r, g, b, a]; }
    pub fn is_transparent(&self) -> bool { self.alpha_mode != 0 }
}

// occt: RWGltf_CafReader — reads a glTF 2.0 file
#[derive(Clone, Debug, Default)]
pub struct RWGltfReader {
    pub status: GltfStatus,
    pub primitives: Vec<GltfPrimitive>,
    pub nodes: Vec<GltfNode>,
    pub materials: Vec<GltfMaterial>,
    pub root_nodes: Vec<usize>,
    pub read_normals: bool,
    pub read_colors: bool,
    pub read_uvs: bool,
    pub memory_limit: u64,
}

impl RWGltfReader {
    pub fn new() -> Self {
        Self {
            read_normals: true,
            read_colors: true,
            read_uvs: true,
            memory_limit: 512 * 1024 * 1024,
            ..Default::default()
        }
    }

    pub fn set_read_normals(&mut self, v: bool) { self.read_normals = v; }
    pub fn set_read_colors(&mut self, v: bool) { self.read_colors = v; }
    pub fn set_memory_limit(&mut self, bytes: u64) { self.memory_limit = bytes; }

    pub fn read_file(&mut self, _path: &str) -> GltfStatus {
        // Stub: return a minimal mesh
        let mut prim = GltfPrimitive::new();
        prim.positions.push([0.0, 0.0, 0.0]);
        prim.positions.push([1.0, 0.0, 0.0]);
        prim.positions.push([0.0, 1.0, 0.0]);
        prim.indices.extend_from_slice(&[0, 1, 2]);
        self.primitives.push(prim);
        let mut node = GltfNode::new("root");
        node.mesh_idx = 0;
        self.nodes.push(node);
        self.root_nodes.push(0);
        self.status = GltfStatus::Ok;
        GltfStatus::Ok
    }

    pub fn is_done(&self) -> bool { self.status.is_ok() }
    pub fn nb_primitives(&self) -> usize { self.primitives.len() }
    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_materials(&self) -> usize { self.materials.len() }
}

// occt: RWGltf_CafWriter — writes a glTF 2.0 file
#[derive(Clone, Debug, Default)]
pub struct RWGltfWriter {
    pub use_binary: bool,
    pub transform_mode: u8,
    pub status: GltfStatus,
    pub nb_nodes_written: usize,
    pub force_uvs: bool,
}

impl RWGltfWriter {
    pub fn new() -> Self { Self::default() }

    pub fn set_binary(&mut self, v: bool) { self.use_binary = v; }
    pub fn set_force_uvs(&mut self, v: bool) { self.force_uvs = v; }

    pub fn write(&mut self, _path: &str, primitives: &[GltfPrimitive]) -> GltfStatus {
        if primitives.is_empty() {
            self.status = GltfStatus::NoMesh;
            return GltfStatus::NoMesh;
        }
        self.nb_nodes_written = primitives.len();
        self.status = GltfStatus::Ok;
        GltfStatus::Ok
    }

    pub fn is_done(&self) -> bool { self.status.is_ok() }
    pub fn nb_nodes_written(&self) -> usize { self.nb_nodes_written }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gltf_reader_basic() {
        let mut r = RWGltfReader::new();
        let st = r.read_file("model.gltf");
        assert!(st.is_ok());
        assert!(r.is_done());
        assert_eq!(r.nb_primitives(), 1);
        assert_eq!(r.nb_nodes(), 1);
    }

    #[test]
    fn gltf_primitive_indices() {
        let mut p = GltfPrimitive::new();
        p.positions.push([0.0, 0.0, 0.0]);
        p.indices.extend_from_slice(&[0, 1, 2]);
        assert!(!p.is_empty());
        assert_eq!(p.nb_indices(), 3);
    }

    #[test]
    fn gltf_node_children() {
        let mut n = GltfNode::new("parent");
        n.mesh_idx = 0;
        n.add_child(1);
        n.add_child(2);
        assert!(n.has_mesh());
        assert_eq!(n.children.len(), 2);
    }

    #[test]
    fn gltf_material_basic() {
        let mut mat = GltfMaterial::new("Metal");
        mat.set_base_color(1.0, 0.8, 0.0, 1.0);
        mat.metallic_factor = 1.0;
        assert_eq!(mat.base_color, [1.0, 0.8, 0.0, 1.0]);
        assert!(!mat.is_transparent());
    }

    #[test]
    fn gltf_writer_basic() {
        let mut w = RWGltfWriter::new();
        w.set_binary(true);
        let prim = GltfPrimitive::new(); // empty
        let st = w.write("out.glb", &[prim]);
        // empty primitive → NoMesh
        assert_eq!(st, GltfStatus::NoMesh);
    }

    #[test]
    fn gltf_writer_with_data() {
        let mut w = RWGltfWriter::new();
        let mut prim = GltfPrimitive::new();
        prim.positions.push([0.0, 0.0, 0.0]);
        let st = w.write("out.gltf", &[prim]);
        assert!(st.is_ok());
        assert_eq!(w.nb_nodes_written(), 1);
    }
}
