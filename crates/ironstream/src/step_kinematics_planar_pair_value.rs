// FILE: step_kinematics_planar_pair_value.rs
// occt: StepKinematics_PlanarPairValue

pub struct PlanarPairValue {
    actual_rotation: f64,
    actual_translation_x: f64,
    actual_translation_y: f64,
}

impl PlanarPairValue {
    pub fn new() -> Self {
        PlanarPairValue {
            actual_rotation: 0.0,
            actual_translation_x: 0.0,
            actual_translation_y: 0.0,
        }
    }

    pub fn init(
        &mut self,
        actual_rotation: f64,
        actual_translation_x: f64,
        actual_translation_y: f64,
    ) {
        self.actual_rotation = actual_rotation;
        self.actual_translation_x = actual_translation_x;
        self.actual_translation_y = actual_translation_y;
    }

    pub fn actual_rotation(&self) -> f64 {
        self.actual_rotation
    }

    pub fn set_actual_rotation(&mut self, value: f64) {
        self.actual_rotation = value;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planar_pair_value_creation() {
        let value = PlanarPairValue::new();
        assert_eq!(value.actual_rotation(), 0.0);
        assert_eq!(value.actual_translation_x(), 0.0);
        assert_eq!(value.actual_translation_y(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut value = PlanarPairValue::new();
        value.init(0.5, 1.0, 2.0);
        assert_eq!(value.actual_rotation(), 0.5);
        assert_eq!(value.actual_translation_x(), 1.0);
        assert_eq!(value.actual_translation_y(), 2.0);
    }

    #[test]
    fn test_setters() {
        let mut value = PlanarPairValue::new();
        value.set_actual_rotation(0.3);
        value.set_actual_translation_x(1.5);
        value.set_actual_translation_y(2.5);

        assert_eq!(value.actual_rotation(), 0.3);
        assert_eq!(value.actual_translation_x(), 1.5);
        assert_eq!(value.actual_translation_y(), 2.5);
    }
}
