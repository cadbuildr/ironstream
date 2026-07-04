// FILE: step_kinematics_revolute_pair_with_range.rs
// occt: StepKinematics_RevolutePairWithRange

/// Representation of STEP entity RevolutePairWithRange.
#[derive(Clone, Debug)]
pub struct StepKinematicsRevolutePairWithRange {
    lower_limit_actual_rotation: Option<f64>,
    upper_limit_actual_rotation: Option<f64>,
}

impl Default for StepKinematicsRevolutePairWithRange {
    fn default() -> Self {
        StepKinematicsRevolutePairWithRange {
            lower_limit_actual_rotation: None,
            upper_limit_actual_rotation: None,
        }
    }
}

impl StepKinematicsRevolutePairWithRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_limit_actual_rotation(&self) -> Option<f64> {
        self.lower_limit_actual_rotation
    }

    pub fn set_lower_limit_actual_rotation(&mut self, value: f64) {
        self.lower_limit_actual_rotation = Some(value);
    }

    pub fn has_lower_limit_actual_rotation(&self) -> bool {
        self.lower_limit_actual_rotation.is_some()
    }

    pub fn upper_limit_actual_rotation(&self) -> Option<f64> {
        self.upper_limit_actual_rotation
    }

    pub fn set_upper_limit_actual_rotation(&mut self, value: f64) {
        self.upper_limit_actual_rotation = Some(value);
    }

    pub fn has_upper_limit_actual_rotation(&self) -> bool {
        self.upper_limit_actual_rotation.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsRevolutePairWithRange::new();
        assert!(!pair.has_lower_limit_actual_rotation());
        assert!(!pair.has_upper_limit_actual_rotation());
    }

    #[test]
    fn test_limits() {
        let mut pair = StepKinematicsRevolutePairWithRange::new();
        pair.set_lower_limit_actual_rotation(-3.14);
        pair.set_upper_limit_actual_rotation(3.14);

        assert_eq!(pair.lower_limit_actual_rotation(), Some(-3.14));
        assert_eq!(pair.upper_limit_actual_rotation(), Some(3.14));
    }
}
