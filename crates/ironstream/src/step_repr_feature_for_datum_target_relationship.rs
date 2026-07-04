// FILE: step_repr_feature_for_datum_target_relationship.rs
// occt: StepRepr_FeatureForDatumTargetRelationship

/// StepRepr_FeatureForDatumTargetRelationship:
/// Representation of STEP entity FeatureForDatumTargetRelationship
/// Inherits from StepRepr_ShapeAspectRelationship
#[derive(Clone, Debug)]
pub struct StepReprFeatureForDatumTargetRelationship {
    name: String,
}

impl StepReprFeatureForDatumTargetRelationship {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprFeatureForDatumTargetRelationship {
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

impl Default for StepReprFeatureForDatumTargetRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let fdtr = StepReprFeatureForDatumTargetRelationship::new();
        assert_eq!(fdtr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut fdtr = StepReprFeatureForDatumTargetRelationship::new();
        fdtr.set_name("feature_rel".to_string());
        assert_eq!(fdtr.name(), "feature_rel");
    }
}
