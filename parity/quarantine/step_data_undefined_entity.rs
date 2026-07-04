// FILE: step_data_undefined_entity.rs
// occt: StepData_UndefinedEntity

//! Represents an undefined entity in STEP
pub struct StepDataUndefinedEntity {
    entity_name: String,
}

impl StepDataUndefinedEntity {
    //! Creates an UndefinedEntity
    pub fn new(name: &str) -> Self {
        StepDataUndefinedEntity {
            entity_name: name.to_string(),
        }
    }

    //! Returns the entity name
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_entity_new() {
        let entity = StepDataUndefinedEntity::new("unknown");
        assert_eq!(entity.entity_name(), "unknown");
    }
}
