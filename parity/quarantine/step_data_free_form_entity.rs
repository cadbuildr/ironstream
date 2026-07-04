// FILE: step_data_free_form_entity.rs
// occt: StepData_FreeFormEntity

//! Represents a free-form entity in STEP
pub struct StepDataFreeFormEntity {
    name: String,
}

impl StepDataFreeFormEntity {
    //! Creates a FreeFormEntity
    pub fn new(name: &str) -> Self {
        StepDataFreeFormEntity {
            name: name.to_string(),
        }
    }

    //! Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_form_entity_new() {
        let entity = StepDataFreeFormEntity::new("test");
        assert_eq!(entity.name(), "test");
    }
}
