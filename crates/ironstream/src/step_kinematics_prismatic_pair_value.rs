// FILE: step_kinematics_prismatic_pair_value.rs
// occt: StepKinematics_PrismaticPairValue

/// Representation of STEP entity PrismaticPairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsPrismaticPairValue {
    actual_translation: f64,
}

impl Default for StepKinematicsPrismaticPairValue {
    fn default() -> Self {
        StepKinematicsPrismaticPairValue {
            actual_translation: 0.0,
        }
    }
}

impl StepKinematicsPrismaticPairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_translation(&self) -> f64 {
        self.actual_translation
    }

    pub fn set_actual_translation(&mut self, value: f64) {
        self.actual_translation = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let value = StepKinematicsPrismaticPairValue::new();
        assert_eq!(value.actual_translation(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut value = StepKinematicsPrismaticPairValue::new();
        value.set_actual_translation(5.5);
        assert_eq!(value.actual_translation(), 5.5);
    }
}
