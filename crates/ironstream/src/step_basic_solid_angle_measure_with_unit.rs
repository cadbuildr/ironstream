// FILE: step_basic_solid_angle_measure_with_unit.rs
// occt: StepBasic_SolidAngleMeasureWithUnit

#[derive(Clone, Debug)]
pub struct StepBasicSolidAngleMeasureWithUnit {
    value_component: f64,
    unit_component: String,
}

impl StepBasicSolidAngleMeasureWithUnit {
    pub fn new() -> Self {
        Self {
            value_component: 0.0,
            unit_component: String::new(),
        }
    }

    pub fn init(&mut self, value: f64, unit: String) {
        self.value_component = value;
        self.unit_component = unit;
    }

    pub fn value_component(&self) -> f64 { self.value_component }
    pub fn set_value_component(&mut self, v: f64) { self.value_component = v; }
    pub fn unit_component(&self) -> &str { &self.unit_component }
    pub fn set_unit_component(&mut self, u: String) { self.unit_component = u; }
}

impl Default for StepBasicSolidAngleMeasureWithUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut m = StepBasicSolidAngleMeasureWithUnit::new();
        m.init(2.0, "steradian".into());
        assert_eq!(m.value_component(), 2.0);
        assert_eq!(m.unit_component(), "steradian");
    }
}
