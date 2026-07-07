// FILE: step_shape_advanced_brep_shape_representation.rs
// occt: StepShape_AdvancedBrepShapeRepresentation

/// Represents an advanced BREP (boundary representation) shape in STEP
pub struct AdvancedBrepShapeRepresentation {
    name: Option<String>,
    representation_type: String,
}

impl AdvancedBrepShapeRepresentation {
    /// Create a new AdvancedBrepShapeRepresentation
    pub fn new() -> Self {
        AdvancedBrepShapeRepresentation {
            name: None,
            representation_type: "ADVANCED_BREP_SHAPE_REPRESENTATION".to_string(),
        }
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the representation type
    pub fn representation_type(&self) -> &str {
        &self.representation_type
    }
}

impl Default for AdvancedBrepShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rep = AdvancedBrepShapeRepresentation::new();
        assert_eq!(rep.name(), None);
        assert_eq!(
            rep.representation_type(),
            "ADVANCED_BREP_SHAPE_REPRESENTATION"
        );
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rep = AdvancedBrepShapeRepresentation::new();
        rep.set_name("BrepRep1".to_string());
        assert_eq!(rep.name(), Some("BrepRep1"));
    }

    #[test]
    fn test_representation_type() {
        let rep = AdvancedBrepShapeRepresentation::new();
        assert!(rep.representation_type().contains("ADVANCED"));
        assert!(rep.representation_type().contains("BREP"));
    }
}
