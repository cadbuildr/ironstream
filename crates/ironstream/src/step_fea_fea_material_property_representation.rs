// FILE: step_fea_fea_material_property_representation.rs
// occt: StepFEA_FeaMaterialPropertyRepresentation

/// Representation of STEP entity FeaMaterialPropertyRepresentation
#[derive(Debug, Clone)]
pub struct StepFeaFeaMaterialPropertyRepresentation;

impl StepFeaFeaMaterialPropertyRepresentation {
    /// Creates a new FeaMaterialPropertyRepresentation
    pub fn new() -> Self {
        StepFeaFeaMaterialPropertyRepresentation
    }
}

impl Default for StepFeaFeaMaterialPropertyRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_material_property_representation_creation() {
        let repr = StepFeaFeaMaterialPropertyRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_fea_material_property_representation_default() {
        let repr = StepFeaFeaMaterialPropertyRepresentation::default();
        let _ = repr;
    }
}
