// FILE: step_visual_mechanical_design_geometric_presentation_representation.rs
// occt: StepVisual_MechanicalDesignGeometricPresentationRepresentation

/// A mechanical design geometric presentation representation in STEP.
///
/// This represents a presentation of mechanical design 3D geometry.
pub struct MechanicalDesignGeometricPresentationRepresentation {
    name: String,
    description: String,
}

impl MechanicalDesignGeometricPresentationRepresentation {
    /// Creates a new mechanical design geometric presentation representation.
    pub fn new(name: String) -> Self {
        MechanicalDesignGeometricPresentationRepresentation {
            name,
            description: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the description.
    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    /// Returns the description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical_design_geometric_presentation_representation_new() {
        let repr = MechanicalDesignGeometricPresentationRepresentation::new("Repr1".to_string());
        assert_eq!(repr.name(), "Repr1");
        assert_eq!(repr.description(), "");
    }

    #[test]
    fn test_set_description() {
        let mut repr =
            MechanicalDesignGeometricPresentationRepresentation::new("Representation".to_string());
        repr.set_description("3D Geometry".to_string());
        assert_eq!(repr.description(), "3D Geometry");
    }
}
