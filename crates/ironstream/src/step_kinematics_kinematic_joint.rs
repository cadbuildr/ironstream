// FILE: step_kinematics_kinematic_joint.rs
// occt: StepKinematics_KinematicJoint

pub struct KinematicJoint;

impl KinematicJoint {
    pub fn new() -> Self {
        KinematicJoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_joint_creation() {
        let _joint = KinematicJoint::new();
    }
}
