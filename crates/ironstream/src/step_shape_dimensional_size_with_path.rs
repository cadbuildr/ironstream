// FILE: step_shape_dimensional_size_with_path.rs
// occt: StepShape_DimensionalSizeWithPath

//! Representation of STEP entity DimensionalSizeWithPath

#[derive(Clone, Debug)]
pub struct DimensionalSizeWithPath {
    applies_to: Option<String>,
    name: String,
    path: Option<String>,
}

impl DimensionalSizeWithPath {
    /// Empty constructor
    pub fn new() -> Self {
        DimensionalSizeWithPath {
            applies_to: None,
            name: String::new(),
            path: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, applies_to: Option<String>, name: String, path: Option<String>) {
        self.applies_to = applies_to;
        self.name = name;
        self.path = path;
    }

    /// Returns field Path
    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    /// Set field Path
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// Returns field AppliesTo (inherited)
    pub fn applies_to(&self) -> &Option<String> {
        &self.applies_to
    }

    /// Set field AppliesTo (inherited)
    pub fn set_applies_to(&mut self, applies_to: Option<String>) {
        self.applies_to = applies_to;
    }

    /// Returns field Name (inherited)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field Name (inherited)
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for DimensionalSizeWithPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dsp = DimensionalSizeWithPath::new();
        assert!(dsp.applies_to().is_none());
        assert_eq!(dsp.name(), "");
        assert!(dsp.path().is_none());
    }

    #[test]
    fn test_init() {
        let mut dsp = DimensionalSizeWithPath::new();
        dsp.init(
            Some("aspect1".to_string()),
            "Size1".to_string(),
            Some("path1".to_string()),
        );
        assert_eq!(dsp.applies_to(), &Some("aspect1".to_string()));
        assert_eq!(dsp.name(), "Size1");
        assert_eq!(dsp.path(), &Some("path1".to_string()));
    }

    #[test]
    fn test_set_path() {
        let mut dsp = DimensionalSizeWithPath::new();
        dsp.set_path(Some("new_path".to_string()));
        assert_eq!(dsp.path(), &Some("new_path".to_string()));
    }
}
