// FILE: step_kinematics_gear_pair_with_range.rs
// occt: StepKinematics_GearPairWithRange

pub struct GearPairWithRange {
    lower_limit_actual_rotation1: Option<f64>,
    upper_limit_actual_rotation1: Option<f64>,
}

impl GearPairWithRange {
    pub fn new() -> Self {
        GearPairWithRange {
            lower_limit_actual_rotation1: None,
            upper_limit_actual_rotation1: None,
        }
    }

    pub fn lower_limit_actual_rotation1(&self) -> Option<f64> {
        self.lower_limit_actual_rotation1
    }

    pub fn set_lower_limit_actual_rotation1(&mut self, value: Option<f64>) {
        self.lower_limit_actual_rotation1 = value;
    }

    pub fn has_lower_limit_actual_rotation1(&self) -> bool {
        self.lower_limit_actual_rotation1.is_some()
    }

    pub fn upper_limit_actual_rotation1(&self) -> Option<f64> {
        self.upper_limit_actual_rotation1
    }

    pub fn set_upper_limit_actual_rotation1(&mut self, value: Option<f64>) {
        self.upper_limit_actual_rotation1 = value;
    }

    pub fn has_upper_limit_actual_rotation1(&self) -> bool {
        self.upper_limit_actual_rotation1.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_pair_with_range_creation() {
        let pair = GearPairWithRange::new();
        assert_eq!(pair.has_lower_limit_actual_rotation1(), false);
        assert_eq!(pair.has_upper_limit_actual_rotation1(), false);
    }

    #[test]
    fn test_set_limits() {
        let mut pair = GearPairWithRange::new();
        pair.set_lower_limit_actual_rotation1(Some(0.5));
        pair.set_upper_limit_actual_rotation1(Some(3.14));

        assert_eq!(pair.has_lower_limit_actual_rotation1(), true);
        assert_eq!(pair.lower_limit_actual_rotation1(), Some(0.5));
        assert_eq!(pair.has_upper_limit_actual_rotation1(), true);
        assert_eq!(pair.upper_limit_actual_rotation1(), Some(3.14));
    }
}
