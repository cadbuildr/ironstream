// FILE: step_kinematics_rack_and_pinion_pair_value.rs
// occt: StepKinematics_RackAndPinionPairValue

/// Representation of STEP entity RackAndPinionPairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsRackAndPinionPairValue {
    actual_displacement: f64,
}

impl Default for StepKinematicsRackAndPinionPairValue {
    fn default() -> Self {
        StepKinematicsRackAndPinionPairValue {
            actual_displacement: 0.0,
        }
    }
}

impl StepKinematicsRackAndPinionPairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_displacement(&self) -> f64 {
        self.actual_displacement
    }

    pub fn set_actual_displacement(&mut self, value: f64) {
        self.actual_displacement = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let value = StepKinematicsRackAndPinionPairValue::new();
        assert_eq!(value.actual_displacement(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut value = StepKinematicsRackAndPinionPairValue::new();
        value.set_actual_displacement(1.5);
        assert_eq!(value.actual_displacement(), 1.5);
    }
}
