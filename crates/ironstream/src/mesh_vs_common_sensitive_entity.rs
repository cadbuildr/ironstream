// FILE: mesh_vs_common_sensitive_entity.rs
// occt: MeshVS_CommonSensitiveEntity

//! Common base for sensitive mesh entities.

#[derive(Clone, Debug)]
pub struct CommonSensitiveEntity {
    pub entity_id: u32,
    pub entity_type: i32,
    pub is_selectable: bool,
}

impl CommonSensitiveEntity {
    pub fn new(entity_id: u32, entity_type: i32) -> Self {
        CommonSensitiveEntity {
            entity_id,
            entity_type,
            is_selectable: true,
        }
    }

    pub fn set_selectable(&mut self, selectable: bool) {
        self.is_selectable = selectable;
    }

    pub fn is_selectable(&self) -> bool {
        self.is_selectable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_sensitive_entity_creation() {
        let ent = CommonSensitiveEntity::new(1, 0);
        assert_eq!(ent.entity_id, 1);
        assert!(ent.is_selectable);
    }

    #[test]
    fn common_sensitive_entity_selectability() {
        let mut ent = CommonSensitiveEntity::new(1, 0);
        ent.set_selectable(false);
        assert!(!ent.is_selectable());
    }
}
