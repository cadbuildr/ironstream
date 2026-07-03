// FILE: mesh_vs_sensitive_quad.rs
// occt: MeshVS_SensitiveQuad

//! Sensitive quad element for mesh selection.

#[derive(Clone, Debug)]
pub struct SensitiveQuad {
    pub entity_id: u32,
    pub quad_id: u32,
    pub vertex_indices: [u32; 4],
}

impl SensitiveQuad {
    pub fn new(entity_id: u32, quad_id: u32) -> Self {
        SensitiveQuad {
            entity_id,
            quad_id,
            vertex_indices: [0, 0, 0, 0],
        }
    }

    pub fn with_vertices(mut self, v0: u32, v1: u32, v2: u32, v3: u32) -> Self {
        self.vertex_indices = [v0, v1, v2, v3];
        self
    }

    pub fn vertex_at(&self, i: usize) -> Option<u32> {
        if i < 4 {
            Some(self.vertex_indices[i])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_quad_creation() {
        let quad = SensitiveQuad::new(1, 10);
        assert_eq!(quad.entity_id, 1);
        assert_eq!(quad.quad_id, 10);
    }

    #[test]
    fn sensitive_quad_vertices() {
        let quad = SensitiveQuad::new(1, 10)
            .with_vertices(0, 1, 2, 3);

        assert_eq!(quad.vertex_at(0), Some(0));
        assert_eq!(quad.vertex_at(3), Some(3));
        assert_eq!(quad.vertex_at(4), None);
    }
}
