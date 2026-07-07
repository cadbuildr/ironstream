// FILE: step_kinematics_low_order_kinematic_pair_with_range.rs
// occt: StepKinematics_LowOrderKinematicPairWithRange

pub struct LowOrderKinematicPairWithRange {
    lower_limit_actual_rotation_x: Option<f64>,
    upper_limit_actual_rotation_x: Option<f64>,
    lower_limit_actual_rotation_y: Option<f64>,
    upper_limit_actual_rotation_y: Option<f64>,
    lower_limit_actual_rotation_z: Option<f64>,
    upper_limit_actual_rotation_z: Option<f64>,
    lower_limit_actual_translation_x: Option<f64>,
    upper_limit_actual_translation_x: Option<f64>,
    lower_limit_actual_translation_y: Option<f64>,
    upper_limit_actual_translation_y: Option<f64>,
    lower_limit_actual_translation_z: Option<f64>,
    upper_limit_actual_translation_z: Option<f64>,
}

impl LowOrderKinematicPairWithRange {
    pub fn new() -> Self {
        LowOrderKinematicPairWithRange {
            lower_limit_actual_rotation_x: None,
            upper_limit_actual_rotation_x: None,
            lower_limit_actual_rotation_y: None,
            upper_limit_actual_rotation_y: None,
            lower_limit_actual_rotation_z: None,
            upper_limit_actual_rotation_z: None,
            lower_limit_actual_translation_x: None,
            upper_limit_actual_translation_x: None,
            lower_limit_actual_translation_y: None,
            upper_limit_actual_translation_y: None,
            lower_limit_actual_translation_z: None,
            upper_limit_actual_translation_z: None,
        }
    }

    pub fn lower_limit_actual_rotation_x(&self) -> Option<f64> {
        self.lower_limit_actual_rotation_x
    }

    pub fn set_lower_limit_actual_rotation_x(&mut self, value: Option<f64>) {
        self.lower_limit_actual_rotation_x = value;
    }

    pub fn has_lower_limit_actual_rotation_x(&self) -> bool {
        self.lower_limit_actual_rotation_x.is_some()
    }

    pub fn upper_limit_actual_rotation_x(&self) -> Option<f64> {
        self.upper_limit_actual_rotation_x
    }

    pub fn set_upper_limit_actual_rotation_x(&mut self, value: Option<f64>) {
        self.upper_limit_actual_rotation_x = value;
    }

    pub fn has_upper_limit_actual_rotation_x(&self) -> bool {
        self.upper_limit_actual_rotation_x.is_some()
    }

    pub fn lower_limit_actual_rotation_y(&self) -> Option<f64> {
        self.lower_limit_actual_rotation_y
    }

    pub fn set_lower_limit_actual_rotation_y(&mut self, value: Option<f64>) {
        self.lower_limit_actual_rotation_y = value;
    }

    pub fn has_lower_limit_actual_rotation_y(&self) -> bool {
        self.lower_limit_actual_rotation_y.is_some()
    }

    pub fn upper_limit_actual_rotation_y(&self) -> Option<f64> {
        self.upper_limit_actual_rotation_y
    }

    pub fn set_upper_limit_actual_rotation_y(&mut self, value: Option<f64>) {
        self.upper_limit_actual_rotation_y = value;
    }

    pub fn has_upper_limit_actual_rotation_y(&self) -> bool {
        self.upper_limit_actual_rotation_y.is_some()
    }

    pub fn lower_limit_actual_rotation_z(&self) -> Option<f64> {
        self.lower_limit_actual_rotation_z
    }

    pub fn set_lower_limit_actual_rotation_z(&mut self, value: Option<f64>) {
        self.lower_limit_actual_rotation_z = value;
    }

    pub fn has_lower_limit_actual_rotation_z(&self) -> bool {
        self.lower_limit_actual_rotation_z.is_some()
    }

    pub fn upper_limit_actual_rotation_z(&self) -> Option<f64> {
        self.upper_limit_actual_rotation_z
    }

    pub fn set_upper_limit_actual_rotation_z(&mut self, value: Option<f64>) {
        self.upper_limit_actual_rotation_z = value;
    }

    pub fn has_upper_limit_actual_rotation_z(&self) -> bool {
        self.upper_limit_actual_rotation_z.is_some()
    }

    pub fn lower_limit_actual_translation_x(&self) -> Option<f64> {
        self.lower_limit_actual_translation_x
    }

    pub fn set_lower_limit_actual_translation_x(&mut self, value: Option<f64>) {
        self.lower_limit_actual_translation_x = value;
    }

    pub fn has_lower_limit_actual_translation_x(&self) -> bool {
        self.lower_limit_actual_translation_x.is_some()
    }

    pub fn upper_limit_actual_translation_x(&self) -> Option<f64> {
        self.upper_limit_actual_translation_x
    }

    pub fn set_upper_limit_actual_translation_x(&mut self, value: Option<f64>) {
        self.upper_limit_actual_translation_x = value;
    }

    pub fn has_upper_limit_actual_translation_x(&self) -> bool {
        self.upper_limit_actual_translation_x.is_some()
    }

    pub fn lower_limit_actual_translation_y(&self) -> Option<f64> {
        self.lower_limit_actual_translation_y
    }

    pub fn set_lower_limit_actual_translation_y(&mut self, value: Option<f64>) {
        self.lower_limit_actual_translation_y = value;
    }

    pub fn has_lower_limit_actual_translation_y(&self) -> bool {
        self.lower_limit_actual_translation_y.is_some()
    }

    pub fn upper_limit_actual_translation_y(&self) -> Option<f64> {
        self.upper_limit_actual_translation_y
    }

    pub fn set_upper_limit_actual_translation_y(&mut self, value: Option<f64>) {
        self.upper_limit_actual_translation_y = value;
    }

    pub fn has_upper_limit_actual_translation_y(&self) -> bool {
        self.upper_limit_actual_translation_y.is_some()
    }

    pub fn lower_limit_actual_translation_z(&self) -> Option<f64> {
        self.lower_limit_actual_translation_z
    }

    pub fn set_lower_limit_actual_translation_z(&mut self, value: Option<f64>) {
        self.lower_limit_actual_translation_z = value;
    }

    pub fn has_lower_limit_actual_translation_z(&self) -> bool {
        self.lower_limit_actual_translation_z.is_some()
    }

    pub fn upper_limit_actual_translation_z(&self) -> Option<f64> {
        self.upper_limit_actual_translation_z
    }

    pub fn set_upper_limit_actual_translation_z(&mut self, value: Option<f64>) {
        self.upper_limit_actual_translation_z = value;
    }

    pub fn has_upper_limit_actual_translation_z(&self) -> bool {
        self.upper_limit_actual_translation_z.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_order_kinematic_pair_with_range_creation() {
        let pair = LowOrderKinematicPairWithRange::new();
        assert_eq!(pair.has_lower_limit_actual_rotation_x(), false);
        assert_eq!(pair.has_upper_limit_actual_rotation_x(), false);
    }

    #[test]
    fn test_set_rotation_limits() {
        let mut pair = LowOrderKinematicPairWithRange::new();
        pair.set_lower_limit_actual_rotation_x(Some(0.0));
        pair.set_upper_limit_actual_rotation_x(Some(3.14));

        assert_eq!(pair.has_lower_limit_actual_rotation_x(), true);
        assert_eq!(pair.lower_limit_actual_rotation_x(), Some(0.0));
    }

    #[test]
    fn test_set_translation_limits() {
        let mut pair = LowOrderKinematicPairWithRange::new();
        pair.set_lower_limit_actual_translation_y(Some(-1.0));
        pair.set_upper_limit_actual_translation_y(Some(1.0));

        assert_eq!(pair.has_lower_limit_actual_translation_y(), true);
        assert_eq!(pair.lower_limit_actual_translation_y(), Some(-1.0));
    }
}
