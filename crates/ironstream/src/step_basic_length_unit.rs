// FILE: step_basic_length_unit.rs
// occt: StepBasic_LengthUnit

/// Representation of STEP entity LengthUnit
/// Extends NamedUnit with specific semantics for length units
#[derive(Clone, Debug)]
pub struct LengthUnit {
    name: Option<String>,
}

impl LengthUnit {
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

impl Default for LengthUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let unit = LengthUnit::new();
        assert!(unit.name().is_none());
    }

    #[test]
    fn test_init() {
        let mut unit = LengthUnit::new();
        unit.init("millimetre".to_string());
        assert_eq!(unit.name(), Some("millimetre"));
    }

    #[test]
    fn test_set_name() {
        let mut unit = LengthUnit::new();
        unit.set_name("metre".to_string());
        assert_eq!(unit.name(), Some("metre"));
    }

    #[test]
    fn test_common_units() {
        let mut unit = LengthUnit::new();
        unit.init("centimetre".to_string());
        assert_eq!(unit.name(), Some("centimetre"));
    }

    #[test]
    fn test_default() {
        let unit = LengthUnit::default();
        assert!(unit.name().is_none());
    }
}
