// FILE: step_shape_type_qualifier.rs
// occt: StepShape_TypeQualifier

use std::sync::Arc;

/// Represents a type qualifier for dimensional tolerances in STEP format.
pub struct TypeQualifier {
    name: Arc<str>,
}

impl TypeQualifier {
    /// Create a new TypeQualifier
    pub fn new() -> Self {
        TypeQualifier {
            name: Arc::from(""),
        }
    }

    /// Initialize with a name
    pub fn init(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_qualifier_creation() {
        let tq = TypeQualifier::new();
        assert_eq!(tq.name(), "");
    }

    #[test]
    fn test_init_method() {
        let mut tq = TypeQualifier::new();
        let name: Arc<str> = Arc::from("qualifier_name");

        tq.init(name.clone());

        assert_eq!(tq.name(), "qualifier_name");
    }

    #[test]
    fn test_set_name() {
        let mut tq = TypeQualifier::new();
        let name: Arc<str> = Arc::from("new_qualifier");

        tq.set_name(name);

        assert_eq!(tq.name(), "new_qualifier");
    }

    #[test]
    fn test_name_changes() {
        let mut tq = TypeQualifier::new();

        tq.set_name(Arc::from("first"));
        assert_eq!(tq.name(), "first");

        tq.set_name(Arc::from("second"));
        assert_eq!(tq.name(), "second");
    }
}
