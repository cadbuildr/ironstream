// FILE: step_shape_dimensional_size.rs
// occt: StepShape_DimensionalSize

//! Representation of STEP entity DimensionalSize

#[derive(Clone, Debug)]
pub struct DimensionalSize {
    applies_to: Option<String>,
    name: String,
}

impl DimensionalSize {
    /// Empty constructor
    pub fn new() -> Self {
        DimensionalSize {
            applies_to: None,
            name: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, applies_to: Option<String>, name: String) {
        self.applies_to = applies_to;
        self.name = name;
    }

    /// Returns field AppliesTo
    pub fn applies_to(&self) -> &Option<String> {
        &self.applies_to
    }

    /// Set field AppliesTo
    pub fn set_applies_to(&mut self, applies_to: Option<String>) {
        self.applies_to = applies_to;
    }

    /// Returns field Name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field Name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for DimensionalSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ds = DimensionalSize::new();
        assert!(ds.applies_to().is_none());
        assert_eq!(ds.name(), "");
    }

    #[test]
    fn test_init() {
        let mut ds = DimensionalSize::new();
        ds.init(Some("aspect1".to_string()), "Size1".to_string());
        assert_eq!(ds.applies_to(), &Some("aspect1".to_string()));
        assert_eq!(ds.name(), "Size1");
    }

    #[test]
    fn test_set_applies_to() {
        let mut ds = DimensionalSize::new();
        ds.set_applies_to(Some("aspect2".to_string()));
        assert_eq!(ds.applies_to(), &Some("aspect2".to_string()));
    }

    #[test]
    fn test_set_name() {
        let mut ds = DimensionalSize::new();
        ds.set_name("NewSize".to_string());
        assert_eq!(ds.name(), "NewSize");
    }
}
