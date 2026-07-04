// FILE: step_kinematics_spherical_pair_with_pin.rs
// occt: StepKinematics_SphericalPairWithPin

/// Representation of STEP entity SphericalPairWithPin.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsSphericalPairWithPin;

impl StepKinematicsSphericalPairWithPin {
    pub fn new() -> Self {
        StepKinematicsSphericalPairWithPin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsSphericalPairWithPin::new();
    }
}
