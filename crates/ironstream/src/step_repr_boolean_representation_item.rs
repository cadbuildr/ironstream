// FILE: step_repr_boolean_representation_item.rs
// occt: StepRepr_BooleanRepresentationItem

use std::fmt;

/// StepRepr_BooleanRepresentationItem: A representation item with a boolean value.
/// Inherits from StepRepr_RepresentationItem.
#[derive(Clone, Debug)]
pub struct StepReprBooleanRepresentationItem {
    name: String,
    value: bool,
}

impl StepReprBooleanRepresentationItem {
    /// Create a new StepReprBooleanRepresentationItem
    pub fn new() -> Self {
        StepReprBooleanRepresentationItem {
            name: String::new(),
            value: false,
        }
    }

    /// Initialize with name and value
    pub fn init(&mut self, name: String, value: bool) {
        self.name = name;
        self.value = value;
    }

    /// Set the value
    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }

    /// Get the value
    pub fn value(&self) -> bool {
        self.value
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for StepReprBooleanRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StepReprBooleanRepresentationItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanRepresentationItem(name={}, value={})", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprBooleanRepresentationItem::new();
        assert_eq!(item.name(), "");
        assert!(!item.value());
    }

    #[test]
    fn test_init() {
        let mut item = StepReprBooleanRepresentationItem::new();
        item.init("test_name".to_string(), true);
        assert_eq!(item.name(), "test_name");
        assert!(item.value());
    }

    #[test]
    fn test_set_value() {
        let mut item = StepReprBooleanRepresentationItem::new();
        assert!(!item.value());
        item.set_value(true);
        assert!(item.value());
        item.set_value(false);
        assert!(!item.value());
    }

    #[test]
    fn test_set_name() {
        let mut item = StepReprBooleanRepresentationItem::new();
        item.set_name("new_name".to_string());
        assert_eq!(item.name(), "new_name");
    }
}
