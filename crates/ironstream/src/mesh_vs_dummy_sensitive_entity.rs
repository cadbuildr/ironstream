// FILE: mesh_vs_dummy_sensitive_entity.rs
// occt: MeshVS_DummySensitiveEntity

//! Dummy sensitive entity used as placeholder in mesh selection.

#[derive(Clone, Debug)]
pub struct DummySensitiveEntity {
    pub entity_id: u32,
}

impl DummySensitiveEntity {
    pub fn new(entity_id: u32) -> Self {
        DummySensitiveEntity { entity_id }
    }

    pub fn is_dummy() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_sensitive_entity_creation() {
        let ent = DummySensitiveEntity::new(1);
        assert_eq!(ent.entity_id, 1);
        assert!(DummySensitiveEntity::is_dummy());
    }
}
