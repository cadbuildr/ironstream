// FILE: step_kinematics_universal_pair_with_range.rs
// occt: StepKinematics_UniversalPairWithRange

/// Representation of STEP entity UniversalPairWithRange.
#[derive(Clone, Debug)]
pub struct StepKinematicsUniversalPairWithRange {
    lower_limit_first_rotation: Option<f64>,
    upper_limit_first_rotation: Option<f64>,
    lower_limit_second_rotation: Option<f64>,
    upper_limit_second_rotation: Option<f64>,
}

impl Default for StepKinematicsUniversalPairWithRange {
    fn default() -> Self {
        StepKinematicsUniversalPairWithRange {
            lower_limit_first_rotation: None,
            upper_limit_first_rotation: None,
            lower_limit_second_rotation: None,
            upper_limit_second_rotation: None,
        }
    }
}

impl StepKinematicsUniversalPairWithRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_limit_first_rotation(&self) -> Option<f64> {
        self.lower_limit_first_rotation
    }

    pub fn set_lower_limit_first_rotation(&mut self, value: f64) {
        self.lower_limit_first_rotation = Some(value);
    }

    pub fn has_lower_limit_first_rotation(&self) -> bool {
        self.lower_limit_first_rotation.is_some()
    }

    pub fn upper_limit_first_rotation(&self) -> Option<f64> {
        self.upper_limit_first_rotation
    }

    pub fn set_upper_limit_first_rotation(&mut self, value: f64) {
        self.upper_limit_first_rotation = Some(value);
    }

    pub fn has_upper_limit_first_rotation(&self) -> bool {
        self.upper_limit_first_rotation.is_some()
    }

    pub fn lower_limit_second_rotation(&self) -> Option<f64> {
        self.lower_limit_second_rotation
    }

    pub fn set_lower_limit_second_rotation(&mut self, value: f64) {
        self.lower_limit_second_rotation = Some(value);
    }

    pub fn has_lower_limit_second_rotation(&self) -> bool {
        self.lower_limit_second_rotation.is_some()
    }

    pub fn upper_limit_second_rotation(&self) -> Option<f64> {
        self.upper_limit_second_rotation
    }

    pub fn set_upper_limit_second_rotation(&mut self, value: f64) {
        self.upper_limit_second_rotation = Some(value);
    }

    pub fn has_upper_limit_second_rotation(&self) -> bool {
        self.upper_limit_second_rotation.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsUniversalPairWithRange::new();
        assert!(!pair.has_lower_limit_first_rotation());
    }

    #[test]
    fn test_all_limits() {
        let mut pair = StepKinematicsUniversalPairWithRange::new();
        pair.set_lower_limit_first_rotation(-1.5);
        pair.set_upper_limit_first_rotation(1.5);
        pair.set_lower_limit_second_rotation(-1.0);
        pair.set_upper_limit_second_rotation(1.0);

        assert!(pair.has_lower_limit_first_rotation());
        assert!(pair.has_upper_limit_first_rotation());
        assert!(pair.has_lower_limit_second_rotation());
        assert!(pair.has_upper_limit_second_rotation());
    }
}
