// FILE: step_kinematics_prismatic_pair_with_range.rs
// occt: StepKinematics_PrismaticPairWithRange

/// Representation of STEP entity PrismaticPairWithRange.
#[derive(Clone, Debug)]
pub struct StepKinematicsPrismaticPairWithRange {
    lower_limit_actual_translation: Option<f64>,
    upper_limit_actual_translation: Option<f64>,
}

impl Default for StepKinematicsPrismaticPairWithRange {
    fn default() -> Self {
        StepKinematicsPrismaticPairWithRange {
            lower_limit_actual_translation: None,
            upper_limit_actual_translation: None,
        }
    }
}

impl StepKinematicsPrismaticPairWithRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_limit_actual_translation(&self) -> Option<f64> {
        self.lower_limit_actual_translation
    }

    pub fn set_lower_limit_actual_translation(&mut self, value: f64) {
        self.lower_limit_actual_translation = Some(value);
    }

    pub fn has_lower_limit_actual_translation(&self) -> bool {
        self.lower_limit_actual_translation.is_some()
    }

    pub fn upper_limit_actual_translation(&self) -> Option<f64> {
        self.upper_limit_actual_translation
    }

    pub fn set_upper_limit_actual_translation(&mut self, value: f64) {
        self.upper_limit_actual_translation = Some(value);
    }

    pub fn has_upper_limit_actual_translation(&self) -> bool {
        self.upper_limit_actual_translation.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsPrismaticPairWithRange::new();
        assert!(!pair.has_lower_limit_actual_translation());
        assert!(!pair.has_upper_limit_actual_translation());
    }

    #[test]
    fn test_limits() {
        let mut pair = StepKinematicsPrismaticPairWithRange::new();
        pair.set_lower_limit_actual_translation(-10.0);
        pair.set_upper_limit_actual_translation(10.0);

        assert_eq!(pair.lower_limit_actual_translation(), Some(-10.0));
        assert_eq!(pair.upper_limit_actual_translation(), Some(10.0));
    }
}
