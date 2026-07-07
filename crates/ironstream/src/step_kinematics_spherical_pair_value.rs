// FILE: step_kinematics_spherical_pair_value.rs
// occt: StepKinematics_SphericalPairValue

/// Representation of STEP entity SphericalPairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsSphericalPairValue {
    input_orientation: StepKinematicsSpatialRotationRef,
}

#[derive(Clone, Debug)]
pub struct StepKinematicsSpatialRotationRef;

impl Default for StepKinematicsSphericalPairValue {
    fn default() -> Self {
        StepKinematicsSphericalPairValue {
            input_orientation: StepKinematicsSpatialRotationRef,
        }
    }
}

impl StepKinematicsSphericalPairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_orientation(&self) -> &StepKinematicsSpatialRotationRef {
        &self.input_orientation
    }

    pub fn set_input_orientation(&mut self, orientation: StepKinematicsSpatialRotationRef) {
        self.input_orientation = orientation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _value = StepKinematicsSphericalPairValue::new();
    }
}
