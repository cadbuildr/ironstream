// FILE: step_kinematics_product_definition_kinematics.rs
// occt: StepKinematics_ProductDefinitionKinematics

/// Representation of STEP entity ProductDefinitionKinematics.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsProductDefinitionKinematics;

impl StepKinematicsProductDefinitionKinematics {
    pub fn new() -> Self {
        StepKinematicsProductDefinitionKinematics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _obj = StepKinematicsProductDefinitionKinematics::new();
    }
}
