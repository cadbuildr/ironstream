// FILE: step_basic_named_unit.rs
// occt: StepBasic_NamedUnit

/// Representation of STEP entity NamedUnit
#[derive(Clone, Debug)]
pub struct NamedUnit {
    dimensions: Option<String>,
}

impl NamedUnit {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            dimensions: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, dimensions: String) {
        self.dimensions = Some(dimensions);
    }

    /// Set dimensions
    pub fn set_dimensions(&mut self, dimensions: String) {
        self.dimensions = Some(dimensions);
    }

    /// Get dimensions
    pub fn dimensions(&self) -> Option<&str> {
        self.dimensions.as_deref()
    }
}

impl Default for NamedUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let unit = NamedUnit::new();
        assert!(unit.dimensions().is_none());
    }

    #[test]
    fn test_init() {
        let mut unit = NamedUnit::new();
        unit.init("length".to_string());
        assert_eq!(unit.dimensions(), Some("length"));
    }

    #[test]
    fn test_set_dimensions() {
        let mut unit = NamedUnit::new();
        unit.set_dimensions("mass".to_string());
        assert_eq!(unit.dimensions(), Some("mass"));
    }

    #[test]
    fn test_default() {
        let unit = NamedUnit::default();
        assert!(unit.dimensions().is_none());
    }
}
