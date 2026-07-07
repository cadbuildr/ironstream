// FILE: step_repr_real_representation_item.rs
// occt: StepRepr_RealRepresentationItem

/// StepRepr_RealRepresentationItem: A representation item with a double value
/// Inherits from StepRepr_RepresentationItem
#[derive(Clone, Debug)]
pub struct StepReprRealRepresentationItem {
    name: String,
    value: f64,
}

impl StepReprRealRepresentationItem {
    /// Returns a RealRepresentationItem
    pub fn new() -> Self {
        StepReprRealRepresentationItem {
            name: String::new(),
            value: 0.0,
        }
    }

    /// Initialize with name and value
    pub fn init(&mut self, name: String, value: f64) {
        self.name = name;
        self.value = value;
    }

    /// Set value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Get value
    pub fn value(&self) -> f64 {
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

impl Default for StepReprRealRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprRealRepresentationItem::new();
        assert_eq!(item.name(), "");
        assert_eq!(item.value(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut item = StepReprRealRepresentationItem::new();
        item.init("test_real".to_string(), 3.14);
        assert_eq!(item.name(), "test_real");
        assert_eq!(item.value(), 3.14);
    }

    #[test]
    fn test_set_value() {
        let mut item = StepReprRealRepresentationItem::new();
        assert_eq!(item.value(), 0.0);
        item.set_value(2.71828);
        assert_eq!(item.value(), 2.71828);
        item.set_value(-1.5);
        assert_eq!(item.value(), -1.5);
    }

    #[test]
    fn test_set_name() {
        let mut item = StepReprRealRepresentationItem::new();
        item.set_name("new_name".to_string());
        assert_eq!(item.name(), "new_name");
    }
}
