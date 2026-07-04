// FILE: step_kinematics_universal_pair.rs
// occt: StepKinematics_UniversalPair

/// Representation of STEP entity UniversalPair.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsUniversalPair;

impl StepKinematicsUniversalPair {
    pub fn new() -> Self {
        StepKinematicsUniversalPair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsUniversalPair::new();
    }
}
