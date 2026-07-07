// FILE: step_repr_constructive_geometry_representation_relationship.rs
// occt: StepRepr_ConstructiveGeometryRepresentationRelationship

/// StepRepr_ConstructiveGeometryRepresentationRelationship:
/// Representation relationship for constructive geometry
/// Inherits from StepRepr_RepresentationRelationship
#[derive(Clone, Debug)]
pub struct StepReprConstructiveGeometryRepresentationRelationship {
    name: String,
}

impl StepReprConstructiveGeometryRepresentationRelationship {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprConstructiveGeometryRepresentationRelationship {
            name: String::new(),
        }
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepReprConstructiveGeometryRepresentationRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let cgrr = StepReprConstructiveGeometryRepresentationRelationship::new();
        assert_eq!(cgrr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut cgrr = StepReprConstructiveGeometryRepresentationRelationship::new();
        cgrr.set_name("test_rel".to_string());
        assert_eq!(cgrr.name(), "test_rel");
    }
}
