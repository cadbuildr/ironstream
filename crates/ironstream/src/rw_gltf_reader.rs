// FILE: rw_gltf_reader.rs
// occt: RWGltf_CafReader, RWGltf_GltfJsonParser, RWGltf_PrimitiveArrayReader

// occt: RWGltf_GltfAccessorLayout
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfAccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl GltfAccessorType {
    pub fn nb_components(&self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
            Self::Mat2 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}

// occt: RWGltf_GltfAccessor
#[derive(Clone, Debug)]
pub struct GltfAccessor {
    pub buffer_view: usize,
    pub byte_offset: usize,
    pub count: usize,
    pub accessor_type: GltfAccessorType,
    pub component_type: u32,
}

impl GltfAccessor {
    pub fn new(buffer_view: usize, count: usize, accessor_type: GltfAccessorType) -> Self {
        Self { buffer_view, byte_offset: 0, count, accessor_type, component_type: 5126 } // FLOAT
    }
}

// occt: RWGltf_GltfNode
#[derive(Clone, Debug)]
pub struct GltfNode {
    pub name: String,
    pub mesh_index: Option<usize>,
    pub children: Vec<usize>,
    pub translation: [f64; 3],
    pub rotation: [f64; 4],
    pub scale: [f64; 3],
}

impl GltfNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mesh_index: None,
            children: Vec::new(),
            translation: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

// occt: RWGltf_GltfMesh
#[derive(Clone, Debug)]
pub struct GltfMesh {
    pub name: String,
    pub primitives: Vec<GltfPrimitive>,
}

// occt: RWGltf_GltfPrimitive
#[derive(Clone, Debug)]
pub struct GltfPrimitive {
    pub position_accessor: Option<usize>,
    pub normal_accessor: Option<usize>,
    pub texcoord_accessor: Option<usize>,
    pub index_accessor: Option<usize>,
    pub material_index: Option<usize>,
}

// occt: RWGltf_CafReader
/// Reads a glTF 2.0 file into an in-memory model.
#[derive(Clone, Debug, Default)]
pub struct GltfReader {
    pub nodes: Vec<GltfNode>,
    pub meshes: Vec<GltfMesh>,
    pub accessors: Vec<GltfAccessor>,
    pub scene_roots: Vec<usize>,
    pub done: bool,
    pub nb_warnings: u32,
    pub nb_errors: u32,
}

impl GltfReader {
    pub fn new() -> Self { Self::default() }

    pub fn read_file(&mut self, path: &str) -> bool {
        // Stub: record the attempt
        self.nb_errors += 1;
        let _ = path;
        false
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_meshes(&self) -> usize { self.meshes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gltf_accessor_type_components() {
        assert_eq!(GltfAccessorType::Vec3.nb_components(), 3);
        assert_eq!(GltfAccessorType::Mat4.nb_components(), 16);
    }

    #[test]
    fn gltf_reader_stub() {
        let mut r = GltfReader::new();
        assert!(!r.read_file("model.gltf"));
        assert_eq!(r.nb_errors, 1);
    }

    #[test]
    fn gltf_node_default_transform() {
        let n = GltfNode::new("Root");
        assert_eq!(n.translation, [0.0; 3]);
        assert_eq!(n.scale, [1.0, 1.0, 1.0]);
        assert_eq!(n.rotation[3], 1.0);
    }
}
