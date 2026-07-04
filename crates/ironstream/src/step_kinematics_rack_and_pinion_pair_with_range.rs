// FILE: step_kinematics_rack_and_pinion_pair_with_range.rs
// occt: StepKinematics_RackAndPinionPairWithRange

/// Representation of STEP entity RackAndPinionPairWithRange.
#[derive(Clone, Debug)]
pub struct StepKinematicsRackAndPinionPairWithRange {
    lower_limit_rack_displacement: Option<f64>,
    upper_limit_rack_displacement: Option<f64>,
}

impl Default for StepKinematicsRackAndPinionPairWithRange {
    fn default() -> Self {
        StepKinematicsRackAndPinionPairWithRange {
            lower_limit_rack_displacement: None,
            upper_limit_rack_displacement: None,
        }
    }
}

impl StepKinematicsRackAndPinionPairWithRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_limit_rack_displacement(&self) -> Option<f64> {
        self.lower_limit_rack_displacement
    }

    pub fn set_lower_limit_rack_displacement(&mut self, value: f64) {
        self.lower_limit_rack_displacement = Some(value);
    }

    pub fn has_lower_limit_rack_displacement(&self) -> bool {
        self.lower_limit_rack_displacement.is_some()
    }

    pub fn upper_limit_rack_displacement(&self) -> Option<f64> {
        self.upper_limit_rack_displacement
    }

    pub fn set_upper_limit_rack_displacement(&mut self, value: f64) {
        self.upper_limit_rack_displacement = Some(value);
    }

    pub fn has_upper_limit_rack_displacement(&self) -> bool {
        self.upper_limit_rack_displacement.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsRackAndPinionPairWithRange::new();
        assert!(!pair.has_lower_limit_rack_displacement());
        assert!(!pair.has_upper_limit_rack_displacement());
    }

    #[test]
    fn test_limits() {
        let mut pair = StepKinematicsRackAndPinionPairWithRange::new();
        pair.set_lower_limit_rack_displacement(-5.0);
        pair.set_upper_limit_rack_displacement(5.0);

        assert_eq!(pair.lower_limit_rack_displacement(), Some(-5.0));
        assert_eq!(pair.upper_limit_rack_displacement(), Some(5.0));
    }
}
