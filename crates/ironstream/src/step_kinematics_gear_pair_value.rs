// FILE: step_kinematics_gear_pair_value.rs
// occt: StepKinematics_GearPairValue

pub struct GearPairValue {
    actual_rotation1: f64,
}

impl GearPairValue {
    pub fn new() -> Self {
        GearPairValue {
            actual_rotation1: 0.0,
        }
    }

    pub fn init(&mut self, actual_rotation1: f64) {
        self.actual_rotation1 = actual_rotation1;
    }

    pub fn actual_rotation1(&self) -> f64 {
        self.actual_rotation1
    }

    pub fn set_actual_rotation1(&mut self, value: f64) {
        self.actual_rotation1 = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_pair_value_creation() {
        let value = GearPairValue::new();
        assert_eq!(value.actual_rotation1(), 0.0);
    }

    #[test]
    fn test_gear_pair_value_init() {
        let mut value = GearPairValue::new();
        value.init(1.57);
        assert_eq!(value.actual_rotation1(), 1.57);
    }

    #[test]
    fn test_gear_pair_value_setter() {
        let mut value = GearPairValue::new();
        value.set_actual_rotation1(3.14);
        assert_eq!(value.actual_rotation1(), 3.14);
    }
}
