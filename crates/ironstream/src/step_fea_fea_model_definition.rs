// FILE: step_fea_fea_model_definition.rs
// occt: StepFEA_FeaModelDefinition

/// Representation of STEP entity FeaModelDefinition
#[derive(Debug, Clone)]
pub struct StepFeaFeaModelDefinition;

impl StepFeaFeaModelDefinition {
    /// Creates a new FeaModelDefinition
    pub fn new() -> Self {
        StepFeaFeaModelDefinition
    }
}

impl Default for StepFeaFeaModelDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_model_definition_creation() {
        let def = StepFeaFeaModelDefinition::new();
        let _ = def;
    }

    #[test]
    fn test_fea_model_definition_default() {
        let def = StepFeaFeaModelDefinition::default();
        let _ = def;
    }
}
