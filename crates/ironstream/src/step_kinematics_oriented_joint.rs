// FILE: step_kinematics_oriented_joint.rs
// occt: StepKinematics_OrientedJoint

pub struct OrientedJoint;

impl OrientedJoint {
    pub fn new() -> Self {
        OrientedJoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oriented_joint_creation() {
        let _joint = OrientedJoint::new();
    }
}
