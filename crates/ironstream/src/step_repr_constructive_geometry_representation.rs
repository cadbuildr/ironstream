// FILE: step_repr_constructive_geometry_representation.rs
// occt: StepRepr_ConstructiveGeometryRepresentation

/// StepRepr_ConstructiveGeometryRepresentation: Representation subclass for constructive geometry
/// Inherits from StepRepr_Representation
#[derive(Clone, Debug)]
pub struct StepReprConstructiveGeometryRepresentation {
    name: String,
}

impl StepReprConstructiveGeometryRepresentation {
    /// Returns a ConstructiveGeometryRepresentation
    pub fn new() -> Self {
        StepReprConstructiveGeometryRepresentation {
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

impl Default for StepReprConstructiveGeometryRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let cgr = StepReprConstructiveGeometryRepresentation::new();
        assert_eq!(cgr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut cgr = StepReprConstructiveGeometryRepresentation::new();
        cgr.set_name("test_geometry".to_string());
        assert_eq!(cgr.name(), "test_geometry");
    }
}
