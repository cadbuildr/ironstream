// FILE: step_basic_si_unit_and_volume_unit.rs
// occt: StepBasic_SiUnitAndVolumeUnit

#[derive(Clone, Debug)]
pub struct StepBasicSiUnitAndVolumeUnit {
    si_unit_name: String,
}

impl StepBasicSiUnitAndVolumeUnit {
    pub fn new() -> Self { Self { si_unit_name: String::new() } }
    pub fn init(&mut self, name: String) { self.si_unit_name = name; }
    pub fn name(&self) -> &str { &self.si_unit_name }
    pub fn set_name(&mut self, name: String) { self.si_unit_name = name; }
}

impl Default for StepBasicSiUnitAndVolumeUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicSiUnitAndVolumeUnit::new();
        u.init("cubic_metre".into());
        assert_eq!(u.name(), "cubic_metre");
    }
}
