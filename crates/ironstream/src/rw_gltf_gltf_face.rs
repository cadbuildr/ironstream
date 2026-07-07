// FILE: rw_gltf_gltf_face.rs
// occt: RWGltf_GltfFace

//! Triangle face representation for glTF.

#[derive(Debug, Clone, Copy)]
pub struct Face {
    vertices: [u32; 3],
}

impl Face {
    pub fn new(v1: u32, v2: u32, v3: u32) -> Self {
        Self {
            vertices: [v1, v2, v3],
        }
    }

    pub fn vertex(&self, i: usize) -> Option<u32> {
        if i < 3 { Some(self.vertices[i]) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face() {
        let f = Face::new(0, 1, 2);
        assert_eq!(f.vertex(0), Some(0));
    }
}
