// FILE: step_basic_si_unit_and_ratio_unit.rs
// occt: StepBasic_SiUnitAndRatioUnit

#[derive(Clone, Debug)]
pub struct StepBasicSiUnitAndRatioUnit {
    si_unit_name: String,
}

impl StepBasicSiUnitAndRatioUnit {
    pub fn new() -> Self { Self { si_unit_name: String::new() } }
    pub fn init(&mut self, name: String) { self.si_unit_name = name; }
    pub fn name(&self) -> &str { &self.si_unit_name }
    pub fn set_name(&mut self, name: String) { self.si_unit_name = name; }
}

impl Default for StepBasicSiUnitAndRatioUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicSiUnitAndRatioUnit::new();
        u.init("metre".into());
        assert_eq!(u.name(), "metre");
    }
}
