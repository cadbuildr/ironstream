// FILE: step_basic_plane_angle_unit.rs
// occt: StepBasic_PlaneAngleUnit

#[derive(Clone, Debug)]
pub struct StepBasicPlaneAngleUnit {
    name: String,
}

impl StepBasicPlaneAngleUnit {
    pub fn new() -> Self { Self { name: String::new() } }
    pub fn init(&mut self, name: String) { self.name = name; }
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name: String) { self.name = name; }
}

impl Default for StepBasicPlaneAngleUnit {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut u = StepBasicPlaneAngleUnit::new();
        u.init("radian".into());
        assert_eq!(u.name(), "radian");
    }
}
