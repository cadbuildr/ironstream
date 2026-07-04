// FILE: step_data_simple.rs
// occt: StepData_Simple

//! Represents a simple entity in STEP
pub struct StepDataSimple {
    entity_type: String,
}

impl StepDataSimple {
    //! Creates a Simple entity
    pub fn new(entity_type: &str) -> Self {
        StepDataSimple {
            entity_type: entity_type.to_string(),
        }
    }

    //! Returns the entity type
    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_new() {
        let simple = StepDataSimple::new("MyType");
        assert_eq!(simple.entity_type(), "MyType");
    }
}
