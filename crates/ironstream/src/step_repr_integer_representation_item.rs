// FILE: step_repr_integer_representation_item.rs
// occt: StepRepr_IntegerRepresentationItem

/// StepRepr_IntegerRepresentationItem: A representation item with an integer value
/// Inherits from StepRepr_RepresentationItem
#[derive(Clone, Debug)]
pub struct StepReprIntegerRepresentationItem {
    name: String,
    value: i32,
}

impl StepReprIntegerRepresentationItem {
    /// Returns an IntegerRepresentationItem
    pub fn new() -> Self {
        StepReprIntegerRepresentationItem {
            name: String::new(),
            value: 0,
        }
    }

    /// Initialize with name and value
    pub fn init(&mut self, name: String, value: i32) {
        self.name = name;
        self.value = value;
    }

    /// Set value
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    /// Get value
    pub fn value(&self) -> i32 {
        self.value
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

impl Default for StepReprIntegerRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprIntegerRepresentationItem::new();
        assert_eq!(item.name(), "");
        assert_eq!(item.value(), 0);
    }

    #[test]
    fn test_init() {
        let mut item = StepReprIntegerRepresentationItem::new();
        item.init("test_int".to_string(), 42);
        assert_eq!(item.name(), "test_int");
        assert_eq!(item.value(), 42);
    }

    #[test]
    fn test_set_value() {
        let mut item = StepReprIntegerRepresentationItem::new();
        assert_eq!(item.value(), 0);
        item.set_value(100);
        assert_eq!(item.value(), 100);
        item.set_value(-50);
        assert_eq!(item.value(), -50);
    }

    #[test]
    fn test_set_name() {
        let mut item = StepReprIntegerRepresentationItem::new();
        item.set_name("new_name".to_string());
        assert_eq!(item.name(), "new_name");
    }
}
