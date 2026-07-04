// FILE: step_kinematics_screw_pair_value.rs
// occt: StepKinematics_ScrewPairValue

/// Representation of STEP entity ScrewPairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsScrewPairValue {
    actual_rotation: f64,
}

impl Default for StepKinematicsScrewPairValue {
    fn default() -> Self {
        StepKinematicsScrewPairValue {
            actual_rotation: 0.0,
        }
    }
}

impl StepKinematicsScrewPairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_rotation(&self) -> f64 {
        self.actual_rotation
    }

    pub fn set_actual_rotation(&mut self, value: f64) {
        self.actual_rotation = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let value = StepKinematicsScrewPairValue::new();
        assert_eq!(value.actual_rotation(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut value = StepKinematicsScrewPairValue::new();
        value.set_actual_rotation(0.5);
        assert_eq!(value.actual_rotation(), 0.5);
    }
}
