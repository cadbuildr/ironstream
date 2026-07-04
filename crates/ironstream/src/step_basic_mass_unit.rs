// FILE: step_basic_mass_unit.rs
// occt: StepBasic_MassUnit

/// Representation of STEP entity MassUnit
/// Extends NamedUnit with specific semantics for mass units
#[derive(Clone, Debug)]
pub struct MassUnit {
    name: Option<String>,
}

impl MassUnit {
    /// Empty constructor
    pub fn new() -> Self {
        Self { name: None }
    }

    /// Initialize with name
    pub fn init(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the unit name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the unit name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

impl Default for MassUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let unit = MassUnit::new();
        assert!(unit.name().is_none());
    }

    #[test]
    fn test_init() {
        let mut unit = MassUnit::new();
        unit.init("kilogram".to_string());
        assert_eq!(unit.name(), Some("kilogram"));
    }

    #[test]
    fn test_set_name() {
        let mut unit = MassUnit::new();
        unit.set_name("gram".to_string());
        assert_eq!(unit.name(), Some("gram"));
    }

    #[test]
    fn test_common_units() {
        let mut unit = MassUnit::new();
        unit.init("tonne".to_string());
        assert_eq!(unit.name(), Some("tonne"));
    }

    #[test]
    fn test_default() {
        let unit = MassUnit::default();
        assert!(unit.name().is_none());
    }
}
