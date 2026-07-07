// FILE: step_basic_thermodynamic_temperature_unit.rs
// occt: StepBasic_ThermodynamicTemperatureUnit

#[derive(Clone, Debug)]
pub struct StepBasicThermodynamicTemperatureUnit {
    name: String,
}

impl StepBasicThermodynamicTemperatureUnit {
    pub fn new() -> Self { Self { name: String::new() } }
    pub fn init(&mut self, name: String) { self.name = name; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
}

impl Default for StepBasicThermodynamicTemperatureUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicThermodynamicTemperatureUnit::new();
        u.init("kelvin".into());
        assert_eq!(u.name(), "kelvin");
    }
}
