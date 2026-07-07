// FILE: step_kinematics_spherical_pair.rs
// occt: StepKinematics_SphericalPair

/// Representation of STEP entity SphericalPair.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsSphericalPair;

impl StepKinematicsSphericalPair {
    pub fn new() -> Self {
        StepKinematicsSphericalPair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsSphericalPair::new();
    }
}
