// FILE: step_basic_si_unit_and_solid_angle_unit.rs
// occt: StepBasic_SiUnitAndSolidAngleUnit

#[derive(Clone, Debug)]
pub struct StepBasicSiUnitAndSolidAngleUnit {
    si_unit_name: String,
}

impl StepBasicSiUnitAndSolidAngleUnit {
    pub fn new() -> Self { Self { si_unit_name: String::new() } }
    pub fn init(&mut self, name: String) { self.si_unit_name = name; }
    pub fn name(&self) -> &str { &self.si_unit_name }
    pub fn set_name(&mut self, name: String) { self.si_unit_name = name; }
}

impl Default for StepBasicSiUnitAndSolidAngleUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicSiUnitAndSolidAngleUnit::new();
        u.init("steradian".into());
        assert_eq!(u.name(), "steradian");
    }
}
