// FILE: step_repr_item_defined_transformation.rs
// occt: StepRepr_ItemDefinedTransformation

/// StepRepr_ItemDefinedTransformation: Added from StepRepr Rev2 to Rev4
#[derive(Clone, Debug)]
pub struct StepReprItemDefinedTransformation {
    name: String,
    description: Option<String>,
    transform_item1: String, // Simplified: storing identifier
    transform_item2: String, // Simplified: storing identifier
}

impl StepReprItemDefinedTransformation {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprItemDefinedTransformation {
            name: String::new(),
            description: None,
            transform_item1: String::new(),
            transform_item2: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        transform_item1: String,
        transform_item2: String,
    ) {
        self.name = name;
        self.description = if !description.is_empty() {
            Some(description)
        } else {
            None
        };
        self.transform_item1 = transform_item1;
        self.transform_item2 = transform_item2;
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if description exists
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        if !description.is_empty() {
            self.description = Some(description);
        } else {
            self.description = None;
        }
    }

    /// Get description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set transform item 1
    pub fn set_transform_item1(&mut self, item: String) {
        self.transform_item1 = item;
    }

    /// Get transform item 1
    pub fn transform_item1(&self) -> &str {
        &self.transform_item1
    }

    /// Set transform item 2
    pub fn set_transform_item2(&mut self, item: String) {
        self.transform_item2 = item;
    }

    /// Get transform item 2
    pub fn transform_item2(&self) -> &str {
        &self.transform_item2
    }
}

impl Default for StepReprItemDefinedTransformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let idt = StepReprItemDefinedTransformation::new();
        assert_eq!(idt.name(), "");
        assert!(!idt.has_description());
        assert_eq!(idt.transform_item1(), "");
        assert_eq!(idt.transform_item2(), "");
    }

    #[test]
    fn test_init() {
        let mut idt = StepReprItemDefinedTransformation::new();
        idt.init(
            "transform".to_string(),
            "a transformation".to_string(),
            "item1".to_string(),
            "item2".to_string(),
        );
        assert_eq!(idt.name(), "transform");
        assert!(idt.has_description());
        assert_eq!(idt.description(), Some("a transformation"));
        assert_eq!(idt.transform_item1(), "item1");
        assert_eq!(idt.transform_item2(), "item2");
    }

    #[test]
    fn test_init_without_description() {
        let mut idt = StepReprItemDefinedTransformation::new();
        idt.init(
            "transform".to_string(),
            "".to_string(),
            "item1".to_string(),
            "item2".to_string(),
        );
        assert_eq!(idt.name(), "transform");
        assert!(!idt.has_description());
        assert_eq!(idt.description(), None);
    }

    #[test]
    fn test_set_description() {
        let mut idt = StepReprItemDefinedTransformation::new();
        assert!(!idt.has_description());
        idt.set_description("new_desc".to_string());
        assert!(idt.has_description());
        assert_eq!(idt.description(), Some("new_desc"));
    }

    #[test]
    fn test_set_items() {
        let mut idt = StepReprItemDefinedTransformation::new();
        idt.set_transform_item1("first".to_string());
        idt.set_transform_item2("second".to_string());
        assert_eq!(idt.transform_item1(), "first");
        assert_eq!(idt.transform_item2(), "second");
    }
}
