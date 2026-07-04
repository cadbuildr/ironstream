// FILE: step_fea_node_representation.rs
// occt: StepFEA_NodeRepresentation

/// Representation of STEP entity NodeRepresentation
#[derive(Debug, Clone)]
pub struct StepFeaNodeRepresentation {
    name: String,
    model_ref: Option<i32>,
}

impl StepFeaNodeRepresentation {
    /// Creates a new empty NodeRepresentation
    pub fn new() -> Self {
        StepFeaNodeRepresentation {
            name: String::new(),
            model_ref: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, model_ref: Option<i32>) {
        self.name = name;
        self.model_ref = model_ref;
    }

    /// Returns field ModelRef
    pub fn model_ref(&self) -> Option<i32> {
        self.model_ref
    }

    /// Set field ModelRef
    pub fn set_model_ref(&mut self, model_ref: Option<i32>) {
        self.model_ref = model_ref;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepFeaNodeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_representation_creation() {
        let repr = StepFeaNodeRepresentation::new();
        assert_eq!(repr.name(), "");
        assert_eq!(repr.model_ref(), None);
    }

    #[test]
    fn test_node_representation_init() {
        let mut repr = StepFeaNodeRepresentation::new();
        repr.init("NodeRepr".to_string(), Some(1));

        assert_eq!(repr.name(), "NodeRepr");
        assert_eq!(repr.model_ref(), Some(1));
    }

    #[test]
    fn test_node_representation_setters() {
        let mut repr = StepFeaNodeRepresentation::new();
        repr.set_name("Test".to_string());
        repr.set_model_ref(Some(2));

        assert_eq!(repr.name(), "Test");
        assert_eq!(repr.model_ref(), Some(2));
    }
}
