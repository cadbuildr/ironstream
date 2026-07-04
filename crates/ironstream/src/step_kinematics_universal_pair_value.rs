// FILE: step_kinematics_universal_pair_value.rs
// occt: StepKinematics_UniversalPairValue

/// Representation of STEP entity UniversalPairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsUniversalPairValue {
    input_orientation: StepKinematicsSpatialRotationRef,
}

#[derive(Clone, Debug)]
pub struct StepKinematicsSpatialRotationRef;

impl Default for StepKinematicsUniversalPairValue {
    fn default() -> Self {
        StepKinematicsUniversalPairValue {
            input_orientation: StepKinematicsSpatialRotationRef,
        }
    }
}

impl StepKinematicsUniversalPairValue {
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
        let _value = StepKinematicsUniversalPairValue::new();
    }
}
