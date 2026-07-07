// FILE: step_kinematics_cylindrical_pair_with_range.rs
// occt: StepKinematics_CylindricalPairWithRange

pub struct CylindricalPairWithRange {
    lower_limit_actual_translation: Option<f64>,
    upper_limit_actual_translation: Option<f64>,
    lower_limit_actual_rotation: Option<f64>,
    upper_limit_actual_rotation: Option<f64>,
}

impl CylindricalPairWithRange {
    pub fn new() -> Self {
        CylindricalPairWithRange {
            lower_limit_actual_translation: None,
            upper_limit_actual_translation: None,
            lower_limit_actual_rotation: None,
            upper_limit_actual_rotation: None,
        }
    }

    pub fn lower_limit_actual_translation(&self) -> Option<f64> {
        self.lower_limit_actual_translation
    }

    pub fn set_lower_limit_actual_translation(&mut self, value: Option<f64>) {
        self.lower_limit_actual_translation = value;
    }

    pub fn has_lower_limit_actual_translation(&self) -> bool {
        self.lower_limit_actual_translation.is_some()
    }

    pub fn upper_limit_actual_translation(&self) -> Option<f64> {
        self.upper_limit_actual_translation
    }

    pub fn set_upper_limit_actual_translation(&mut self, value: Option<f64>) {
        self.upper_limit_actual_translation = value;
    }

    pub fn has_upper_limit_actual_translation(&self) -> bool {
        self.upper_limit_actual_translation.is_some()
    }

    pub fn lower_limit_actual_rotation(&self) -> Option<f64> {
        self.lower_limit_actual_rotation
    }

    pub fn set_lower_limit_actual_rotation(&mut self, value: Option<f64>) {
        self.lower_limit_actual_rotation = value;
    }

    pub fn has_lower_limit_actual_rotation(&self) -> bool {
        self.lower_limit_actual_rotation.is_some()
    }

    pub fn upper_limit_actual_rotation(&self) -> Option<f64> {
        self.upper_limit_actual_rotation
    }

    pub fn set_upper_limit_actual_rotation(&mut self, value: Option<f64>) {
        self.upper_limit_actual_rotation = value;
    }

    pub fn has_upper_limit_actual_rotation(&self) -> bool {
        self.upper_limit_actual_rotation.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylindrical_pair_with_range_creation() {
        let pair = CylindricalPairWithRange::new();
        assert_eq!(pair.has_lower_limit_actual_translation(), false);
        assert_eq!(pair.has_upper_limit_actual_translation(), false);
        assert_eq!(pair.has_lower_limit_actual_rotation(), false);
        assert_eq!(pair.has_upper_limit_actual_rotation(), false);
    }

    #[test]
    fn test_set_limits() {
        let mut pair = CylindricalPairWithRange::new();
        pair.set_lower_limit_actual_translation(Some(1.0));
        pair.set_upper_limit_actual_translation(Some(2.0));
        pair.set_lower_limit_actual_rotation(Some(0.5));
        pair.set_upper_limit_actual_rotation(Some(1.5));

        assert_eq!(pair.has_lower_limit_actual_translation(), true);
        assert_eq!(pair.lower_limit_actual_translation(), Some(1.0));
        assert_eq!(pair.has_upper_limit_actual_translation(), true);
        assert_eq!(pair.upper_limit_actual_translation(), Some(2.0));
        assert_eq!(pair.has_lower_limit_actual_rotation(), true);
        assert_eq!(pair.lower_limit_actual_rotation(), Some(0.5));
        assert_eq!(pair.has_upper_limit_actual_rotation(), true);
        assert_eq!(pair.upper_limit_actual_rotation(), Some(1.5));
    }
}
