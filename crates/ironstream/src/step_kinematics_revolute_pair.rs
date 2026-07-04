// FILE: step_kinematics_revolute_pair.rs
// occt: StepKinematics_RevolutePair

/// Representation of STEP entity RevolutePair.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsRevolutePair;

impl StepKinematicsRevolutePair {
    pub fn new() -> Self {
        StepKinematicsRevolutePair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsRevolutePair::new();
    }
}
