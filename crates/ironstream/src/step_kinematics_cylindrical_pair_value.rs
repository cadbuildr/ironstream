// FILE: step_kinematics_cylindrical_pair_value.rs
// occt: StepKinematics_CylindricalPairValue

pub struct CylindricalPairValue {
    actual_translation: f64,
    actual_rotation: f64,
}

impl CylindricalPairValue {
    pub fn new() -> Self {
        CylindricalPairValue {
            actual_translation: 0.0,
            actual_rotation: 0.0,
        }
    }

    pub fn init(&mut self, actual_translation: f64, actual_rotation: f64) {
        self.actual_translation = actual_translation;
        self.actual_rotation = actual_rotation;
    }

    pub fn actual_translation(&self) -> f64 {
        self.actual_translation
    }

    pub fn set_actual_translation(&mut self, value: f64) {
        self.actual_translation = value;
    }

    pub fn actual_rotation(&self) -> f64 {
        self.actual_rotation
    }

    pub fn set_actual_rotation(&mut self, value: f64) {
        self.actual_rotation = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylindrical_pair_value_creation() {
        let value = CylindricalPairValue::new();
        assert_eq!(value.actual_translation(), 0.0);
        assert_eq!(value.actual_rotation(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut value = CylindricalPairValue::new();
        value.init(1.5, 2.5);
        assert_eq!(value.actual_translation(), 1.5);
        assert_eq!(value.actual_rotation(), 2.5);
    }

    #[test]
    fn test_setters() {
        let mut value = CylindricalPairValue::new();
        value.set_actual_translation(3.0);
        value.set_actual_rotation(4.0);
        assert_eq!(value.actual_translation(), 3.0);
        assert_eq!(value.actual_rotation(), 4.0);
    }
}
