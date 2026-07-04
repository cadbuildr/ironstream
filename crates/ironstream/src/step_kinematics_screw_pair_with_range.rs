// FILE: step_kinematics_screw_pair_with_range.rs
// occt: StepKinematics_ScrewPairWithRange

/// Representation of STEP entity ScrewPairWithRange.
#[derive(Clone, Debug)]
pub struct StepKinematicsScrewPairWithRange {
    lower_limit_actual_rotation: Option<f64>,
    upper_limit_actual_rotation: Option<f64>,
}

impl Default for StepKinematicsScrewPairWithRange {
    fn default() -> Self {
        StepKinematicsScrewPairWithRange {
            lower_limit_actual_rotation: None,
            upper_limit_actual_rotation: None,
        }
    }
}

impl StepKinematicsScrewPairWithRange {
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
        let pair = StepKinematicsScrewPairWithRange::new();
        assert!(!pair.has_lower_limit_actual_rotation());
        assert!(!pair.has_upper_limit_actual_rotation());
    }

    #[test]
    fn test_limits() {
        let mut pair = StepKinematicsScrewPairWithRange::new();
        pair.set_lower_limit_actual_rotation(-1.0);
        pair.set_upper_limit_actual_rotation(1.0);

        assert_eq!(pair.lower_limit_actual_rotation(), Some(-1.0));
        assert_eq!(pair.upper_limit_actual_rotation(), Some(1.0));
    }
}
