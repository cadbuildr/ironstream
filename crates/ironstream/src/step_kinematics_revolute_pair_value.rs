// FILE: step_kinematics_revolute_pair_value.rs
// occt: StepKinematics_RevolutePairValue

/// Representation of STEP entity RevolutePairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsRevolutePairValue {
    actual_rotation: f64,
}

impl Default for StepKinematicsRevolutePairValue {
    fn default() -> Self {
        StepKinematicsRevolutePairValue {
            actual_rotation: 0.0,
        }
    }
}

impl StepKinematicsRevolutePairValue {
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
        let value = StepKinematicsRevolutePairValue::new();
        assert_eq!(value.actual_rotation(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut value = StepKinematicsRevolutePairValue::new();
        value.set_actual_rotation(1.57);
        assert_eq!(value.actual_rotation(), 1.57);
    }
}
