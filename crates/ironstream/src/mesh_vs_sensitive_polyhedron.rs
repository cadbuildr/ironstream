// FILE: mesh_vs_sensitive_polyhedron.rs
// occt: MeshVS_SensitivePolyhedron

//! Sensitive polyhedron for mesh selection.

#[derive(Clone, Debug)]
pub struct SensitivePolyhedron {
    pub entity_id: u32,
    pub vertex_count: usize,
    pub face_count: usize,
}

impl SensitivePolyhedron {
    pub fn new(entity_id: u32) -> Self {
        SensitivePolyhedron {
            entity_id,
            vertex_count: 0,
            face_count: 0,
        }
    }

    pub fn with_geometry(mut self, vertices: usize, faces: usize) -> Self {
        self.vertex_count = vertices;
        self.face_count = faces;
        self
    }

    pub fn is_valid(&self) -> bool {
        self.vertex_count >= 4 && self.face_count >= 4
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_count == 0 || self.face_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_polyhedron_creation() {
        let poly = SensitivePolyhedron::new(1);
        assert_eq!(poly.entity_id, 1);
        assert!(poly.is_empty());
    }

    #[test]
    fn sensitive_polyhedron_validity() {
        let valid = SensitivePolyhedron::new(1)
            .with_geometry(8, 6);

        assert!(valid.is_valid());
        assert!(!valid.is_empty());

        let invalid = SensitivePolyhedron::new(2)
            .with_geometry(2, 2);

        assert!(!invalid.is_valid());
    }
}
