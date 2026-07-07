// FILE: step_kinematics_rigid_placement.rs
// occt: StepKinematics_RigidPlacement

/// Representation of STEP SELECT type RigidPlacement.
/// Can be one of: Axis2Placement3d or SuParameters.
#[derive(Clone, Debug)]
pub enum StepKinematicsRigidPlacement {
    Axis2Placement3d,
    SuParameters,
}

impl Default for StepKinematicsRigidPlacement {
    fn default() -> Self {
        StepKinematicsRigidPlacement::Axis2Placement3d
    }
}

impl StepKinematicsRigidPlacement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_axis2_placement_3d(&self) -> bool {
        matches!(self, StepKinematicsRigidPlacement::Axis2Placement3d)
    }

    pub fn is_su_parameters(&self) -> bool {
        matches!(self, StepKinematicsRigidPlacement::SuParameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let placement = StepKinematicsRigidPlacement::new();
        assert!(placement.is_axis2_placement_3d());
    }

    #[test]
    fn test_variants() {
        let p1 = StepKinematicsRigidPlacement::Axis2Placement3d;
        assert!(p1.is_axis2_placement_3d());

        let p2 = StepKinematicsRigidPlacement::SuParameters;
        assert!(p2.is_su_parameters());
    }
}
