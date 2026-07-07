// FILE: step_kinematics_unconstrained_pair.rs
// occt: StepKinematics_UnconstrainedPair

/// Representation of STEP entity UnconstrainedPair.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsUnconstrainedPair;

impl StepKinematicsUnconstrainedPair {
    pub fn new() -> Self {
        StepKinematicsUnconstrainedPair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsUnconstrainedPair::new();
    }
}
