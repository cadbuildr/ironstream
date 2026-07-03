// FILE: mesh_vs_sensitive_mesh.rs
// occt: MeshVS_SensitiveMesh

//! Sensitive mesh entity for selection.

#[derive(Clone, Debug)]
pub struct SensitiveMesh {
    pub entity_id: u32,
    pub mesh_id: u32,
    pub entity_count: usize,
}

impl SensitiveMesh {
    pub fn new(entity_id: u32, mesh_id: u32) -> Self {
        SensitiveMesh {
            entity_id,
            mesh_id,
            entity_count: 0,
        }
    }

    pub fn with_entity_count(mut self, count: usize) -> Self {
        self.entity_count = count;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.entity_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_mesh_creation() {
        let mesh = SensitiveMesh::new(1, 10);
        assert_eq!(mesh.entity_id, 1);
        assert_eq!(mesh.mesh_id, 10);
        assert!(mesh.is_empty());
    }

    #[test]
    fn sensitive_mesh_builder() {
        let mesh = SensitiveMesh::new(1, 10)
            .with_entity_count(100);

        assert_eq!(mesh.entity_count, 100);
        assert!(!mesh.is_empty());
    }
}
