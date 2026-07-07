// FILE: step_basic_si_unit_and_thermodynamic_temperature_unit.rs
// occt: StepBasic_SiUnitAndThermodynamicTemperatureUnit

#[derive(Clone, Debug)]
pub struct StepBasicSiUnitAndThermodynamicTemperatureUnit {
    si_unit_name: String,
}

impl StepBasicSiUnitAndThermodynamicTemperatureUnit {
    pub fn new() -> Self { Self { si_unit_name: String::new() } }
    pub fn init(&mut self, name: String) { self.si_unit_name = name; }
    pub fn name(&self) -> &str { &self.si_unit_name }
    pub fn set_name(&mut self, name: String) { self.si_unit_name = name; }
}

impl Default for StepBasicSiUnitAndThermodynamicTemperatureUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicSiUnitAndThermodynamicTemperatureUnit::new();
        u.init("kelvin".into());
        assert_eq!(u.name(), "kelvin");
    }
}
