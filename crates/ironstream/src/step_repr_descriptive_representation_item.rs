// FILE: step_repr_descriptive_representation_item.rs
// occt: StepRepr_DescriptiveRepresentationItem

/// StepRepr_DescriptiveRepresentationItem: A representation item with a description
/// Inherits from StepRepr_RepresentationItem
#[derive(Clone, Debug)]
pub struct StepReprDescriptiveRepresentationItem {
    name: String,
    description: String,
}

impl StepReprDescriptiveRepresentationItem {
    /// Returns a DescriptiveRepresentationItem
    pub fn new() -> Self {
        StepReprDescriptiveRepresentationItem {
            name: String::new(),
            description: String::new(),
        }
    }

    /// Initialize with name and description
    pub fn init(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Get description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for StepReprDescriptiveRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprDescriptiveRepresentationItem::new();
        assert_eq!(item.name(), "");
        assert_eq!(item.description(), "");
    }

    #[test]
    fn test_init() {
        let mut item = StepReprDescriptiveRepresentationItem::new();
        item.init("test_name".to_string(), "test_description".to_string());
        assert_eq!(item.name(), "test_name");
        assert_eq!(item.description(), "test_description");
    }

    #[test]
    fn test_set_description() {
        let mut item = StepReprDescriptiveRepresentationItem::new();
        item.set_description("new_desc".to_string());
        assert_eq!(item.description(), "new_desc");
    }

    #[test]
    fn test_set_name() {
        let mut item = StepReprDescriptiveRepresentationItem::new();
        item.set_name("new_name".to_string());
        assert_eq!(item.name(), "new_name");
    }
}
