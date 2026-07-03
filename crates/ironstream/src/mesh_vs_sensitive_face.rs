// FILE: mesh_vs_sensitive_face.rs
// occt: MeshVS_SensitiveFace

//! Sensitive face for mesh selection.

#[derive(Clone, Debug)]
pub struct SensitiveFace {
    pub entity_id: u32,
    pub face_id: u32,
    pub num_triangles: i32,
}

impl SensitiveFace {
    pub fn new(entity_id: u32, face_id: u32) -> Self {
        SensitiveFace {
            entity_id,
            face_id,
            num_triangles: 0,
        }
    }

    pub fn with_triangles(mut self, count: i32) -> Self {
        self.num_triangles = count.max(0);
        self
    }

    pub fn is_triangle(&self) -> bool {
        self.num_triangles == 1
    }

    pub fn is_quad(&self) -> bool {
        self.num_triangles == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_face_creation() {
        let face = SensitiveFace::new(1, 10);
        assert_eq!(face.entity_id, 1);
        assert_eq!(face.face_id, 10);
    }

    #[test]
    fn sensitive_face_predicates() {
        let tri = SensitiveFace::new(1, 10).with_triangles(1);
        assert!(tri.is_triangle());
        assert!(!tri.is_quad());

        let quad = SensitiveFace::new(2, 11).with_triangles(2);
        assert!(quad.is_quad());
        assert!(!quad.is_triangle());
    }
}
