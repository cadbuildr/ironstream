// FILE: step_basic_solid_angle_unit.rs
// occt: StepBasic_SolidAngleUnit

#[derive(Clone, Debug)]
pub struct StepBasicSolidAngleUnit {
    name: String,
}

impl StepBasicSolidAngleUnit {
    pub fn new() -> Self { Self { name: String::new() } }
    pub fn init(&mut self, name: String) { self.name = name; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
}

impl Default for StepBasicSolidAngleUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicSolidAngleUnit::new();
        u.init("steradian".into());
        assert_eq!(u.name(), "steradian");
    }
}
