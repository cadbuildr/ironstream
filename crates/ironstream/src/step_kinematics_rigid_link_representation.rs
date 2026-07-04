// FILE: step_kinematics_rigid_link_representation.rs
// occt: StepKinematics_RigidLinkRepresentation

/// Representation of STEP entity RigidLinkRepresentation.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsRigidLinkRepresentation;

impl StepKinematicsRigidLinkRepresentation {
    pub fn new() -> Self {
        StepKinematicsRigidLinkRepresentation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _obj = StepKinematicsRigidLinkRepresentation::new();
    }
}
