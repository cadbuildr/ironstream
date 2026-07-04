// FILE: step_repr_functionally_defined_transformation.rs
// occt: StepRepr_FunctionallyDefinedTransformation

/// StepRepr_FunctionallyDefinedTransformation: A transformation defined functionally
#[derive(Clone, Debug)]
pub struct StepReprFunctionallyDefinedTransformation {
    name: String,
    description: String,
}

impl StepReprFunctionallyDefinedTransformation {
    /// Returns a FunctionallyDefinedTransformation
    pub fn new() -> Self {
        StepReprFunctionallyDefinedTransformation {
            name: String::new(),
            description: String::new(),
        }
    }

    /// Initialize with name and description
    pub fn init(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Get description
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Default for StepReprFunctionallyDefinedTransformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let fdt = StepReprFunctionallyDefinedTransformation::new();
        assert_eq!(fdt.name(), "");
        assert_eq!(fdt.description(), "");
    }

    #[test]
    fn test_init() {
        let mut fdt = StepReprFunctionallyDefinedTransformation::new();
        fdt.init("transform1".to_string(), "a transformation".to_string());
        assert_eq!(fdt.name(), "transform1");
        assert_eq!(fdt.description(), "a transformation");
    }

    #[test]
    fn test_set_name() {
        let mut fdt = StepReprFunctionallyDefinedTransformation::new();
        fdt.set_name("new_transform".to_string());
        assert_eq!(fdt.name(), "new_transform");
    }

    #[test]
    fn test_set_description() {
        let mut fdt = StepReprFunctionallyDefinedTransformation::new();
        fdt.set_description("new_desc".to_string());
        assert_eq!(fdt.description(), "new_desc");
    }
}
