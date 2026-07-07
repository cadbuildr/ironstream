// FILE: iges_solid_general_module.rs
// occt: IGESSolid_GeneralModule

//! General module for IGESSolid entity operations.
//!
//! Provides generic services for IGESSolid entities (creation, copying, etc.).

pub struct IGESEntity {
    id: usize,
    type_number: i32,
}

impl IGESEntity {
    pub fn new(id: usize, type_num: i32) -> Self {
        IGESEntity {
            id,
            type_number: type_num,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn type_number(&self) -> i32 {
        self.type_number
    }
}

/// General module for IGESSolid operations
pub struct IGESSolidGeneralModule;

impl IGESSolidGeneralModule {
    /// Creates a new general module
    pub fn new() -> Self {
        IGESSolidGeneralModule
    }

    /// Create a new entity (factory method)
    pub fn create_entity(entity_type: i32) -> Option<IGESEntity> {
        match entity_type {
            150 => Some(IGESEntity::new(1, 150)), // Block
            155 => Some(IGESEntity::new(2, 155)), // Cylinder
            156 => Some(IGESEntity::new(3, 156)), // Cone Frustum
            158 => Some(IGESEntity::new(4, 158)), // Ellipsoid
            _ => None,
        }
    }

    /// Copy an entity
    pub fn copy_entity(entity: &IGESEntity) -> IGESEntity {
        IGESEntity::new(entity.id(), entity.type_number())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let _module = IGESSolidGeneralModule::new();
    }

    #[test]
    fn test_create_entity_block() {
        let ent = IGESSolidGeneralModule::create_entity(150);
        assert!(ent.is_some());
        assert_eq!(ent.unwrap().type_number(), 150);
    }

    #[test]
    fn test_create_entity_invalid() {
        let ent = IGESSolidGeneralModule::create_entity(999);
        assert!(ent.is_none());
    }

    #[test]
    fn test_copy_entity() {
        let original = IGESEntity::new(42, 155);
        let copy = IGESSolidGeneralModule::copy_entity(&original);

        assert_eq!(copy.id(), original.id());
        assert_eq!(copy.type_number(), original.type_number());
    }
}
