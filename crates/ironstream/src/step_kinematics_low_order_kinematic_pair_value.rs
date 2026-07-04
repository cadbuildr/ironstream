// FILE: step_kinematics_low_order_kinematic_pair_value.rs
// occt: StepKinematics_LowOrderKinematicPairValue

pub struct LowOrderKinematicPairValue {
    actual_translation_x: f64,
    actual_translation_y: f64,
    actual_translation_z: f64,
    actual_rotation_x: f64,
    actual_rotation_y: f64,
    actual_rotation_z: f64,
}

impl LowOrderKinematicPairValue {
    pub fn new() -> Self {
        LowOrderKinematicPairValue {
            actual_translation_x: 0.0,
            actual_translation_y: 0.0,
            actual_translation_z: 0.0,
            actual_rotation_x: 0.0,
            actual_rotation_y: 0.0,
            actual_rotation_z: 0.0,
        }
    }

    pub fn init(
        &mut self,
        actual_translation_x: f64,
        actual_translation_y: f64,
        actual_translation_z: f64,
        actual_rotation_x: f64,
        actual_rotation_y: f64,
        actual_rotation_z: f64,
    ) {
        self.actual_translation_x = actual_translation_x;
        self.actual_translation_y = actual_translation_y;
        self.actual_translation_z = actual_translation_z;
        self.actual_rotation_x = actual_rotation_x;
        self.actual_rotation_y = actual_rotation_y;
        self.actual_rotation_z = actual_rotation_z;
    }

    pub fn actual_translation_x(&self) -> f64 {
        self.actual_translation_x
    }

    pub fn set_actual_translation_x(&mut self, value: f64) {
        self.actual_translation_x = value;
    }

    pub fn actual_translation_y(&self) -> f64 {
        self.actual_translation_y
    }

    pub fn set_actual_translation_y(&mut self, value: f64) {
        self.actual_translation_y = value;
    }

    pub fn actual_translation_z(&self) -> f64 {
        self.actual_translation_z
    }

    pub fn set_actual_translation_z(&mut self, value: f64) {
        self.actual_translation_z = value;
    }

    pub fn actual_rotation_x(&self) -> f64 {
        self.actual_rotation_x
    }

    pub fn set_actual_rotation_x(&mut self, value: f64) {
        self.actual_rotation_x = value;
    }

    pub fn actual_rotation_y(&self) -> f64 {
        self.actual_rotation_y
    }

    pub fn set_actual_rotation_y(&mut self, value: f64) {
        self.actual_rotation_y = value;
    }

    pub fn actual_rotation_z(&self) -> f64 {
        self.actual_rotation_z
    }

    pub fn set_actual_rotation_z(&mut self, value: f64) {
        self.actual_rotation_z = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_order_kinematic_pair_value_creation() {
        let value = LowOrderKinematicPairValue::new();
        assert_eq!(value.actual_translation_x(), 0.0);
        assert_eq!(value.actual_translation_y(), 0.0);
        assert_eq!(value.actual_translation_z(), 0.0);
        assert_eq!(value.actual_rotation_x(), 0.0);
        assert_eq!(value.actual_rotation_y(), 0.0);
        assert_eq!(value.actual_rotation_z(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut value = LowOrderKinematicPairValue::new();
        value.init(1.0, 2.0, 3.0, 0.1, 0.2, 0.3);
        assert_eq!(value.actual_translation_x(), 1.0);
        assert_eq!(value.actual_translation_y(), 2.0);
        assert_eq!(value.actual_translation_z(), 3.0);
        assert_eq!(value.actual_rotation_x(), 0.1);
        assert_eq!(value.actual_rotation_y(), 0.2);
        assert_eq!(value.actual_rotation_z(), 0.3);
    }

    #[test]
    fn test_setters() {
        let mut value = LowOrderKinematicPairValue::new();
        value.set_actual_translation_x(1.5);
        value.set_actual_rotation_z(0.5);
        assert_eq!(value.actual_translation_x(), 1.5);
        assert_eq!(value.actual_rotation_z(), 0.5);
    }
}
