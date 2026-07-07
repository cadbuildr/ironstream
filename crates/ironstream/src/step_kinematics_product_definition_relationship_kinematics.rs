// FILE: step_kinematics_product_definition_relationship_kinematics.rs
// occt: StepKinematics_ProductDefinitionRelationshipKinematics

/// Representation of STEP entity ProductDefinitionRelationshipKinematics.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsProductDefinitionRelationshipKinematics;

impl StepKinematicsProductDefinitionRelationshipKinematics {
    pub fn new() -> Self {
        StepKinematicsProductDefinitionRelationshipKinematics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _obj = StepKinematicsProductDefinitionRelationshipKinematics::new();
    }
}
