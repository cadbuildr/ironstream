// FILE: step_repr_data_environment.rs
// occt: StepRepr_DataEnvironment

/// StepRepr_DataEnvironment: Representation of STEP entity DataEnvironment
#[derive(Clone, Debug)]
pub struct StepReprDataEnvironment {
    name: String,
    description: String,
    elements: Vec<String>, // Simplified: storing identifiers of PropertyDefinitionRepresentation
}

impl StepReprDataEnvironment {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprDataEnvironment {
            name: String::new(),
            description: String::new(),
            elements: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, description: String, elements: Vec<String>) {
        self.name = name;
        self.description = description;
        self.elements = elements;
    }

    /// Returns field Name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field Name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns field Description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set field Description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns field Elements
    pub fn elements(&self) -> &[String] {
        &self.elements
    }

    /// Set field Elements
    pub fn set_elements(&mut self, elements: Vec<String>) {
        self.elements = elements;
    }
}

impl Default for StepReprDataEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let de = StepReprDataEnvironment::new();
        assert_eq!(de.name(), "");
        assert_eq!(de.description(), "");
        assert_eq!(de.elements().len(), 0);
    }

    #[test]
    fn test_init() {
        let mut de = StepReprDataEnvironment::new();
        let elements = vec!["elem1".to_string(), "elem2".to_string()];
        de.init("env_name".to_string(), "env_desc".to_string(), elements);
        assert_eq!(de.name(), "env_name");
        assert_eq!(de.description(), "env_desc");
        assert_eq!(de.elements().len(), 2);
    }

    #[test]
    fn test_set_name() {
        let mut de = StepReprDataEnvironment::new();
        de.set_name("new_name".to_string());
        assert_eq!(de.name(), "new_name");
    }

    #[test]
    fn test_set_description() {
        let mut de = StepReprDataEnvironment::new();
        de.set_description("new_desc".to_string());
        assert_eq!(de.description(), "new_desc");
    }

    #[test]
    fn test_set_elements() {
        let mut de = StepReprDataEnvironment::new();
        let elements = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        de.set_elements(elements);
        assert_eq!(de.elements().len(), 3);
    }
}
