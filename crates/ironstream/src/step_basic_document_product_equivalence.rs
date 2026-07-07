// FILE: step_basic_document_product_equivalence.rs
// occt: StepBasic_DocumentProductEquivalence

/// Representation of STEP entity DocumentProductEquivalence.
/// This is a marker type that extends DocumentProductAssociation with no additional fields.
#[derive(Clone, Debug)]
pub struct DocumentProductEquivalence {
    // Inherited from DocumentProductAssociation
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
    // relating_document: Handle<Document>,
    // related_product: ProductOrFormationOrDefinition,
}

impl DocumentProductEquivalence {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            has_description: false,
        }
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.has_description = true;
    }

    /// Check if description is defined
    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for DocumentProductEquivalence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_constructor() {
        let equiv = DocumentProductEquivalence::new();
        assert!(equiv.name().is_none());
        assert!(equiv.description().is_none());
        assert!(!equiv.has_description());
    }

    #[test]
    fn test_set_name() {
        let mut equiv = DocumentProductEquivalence::new();
        equiv.set_name("TestName".to_string());
        assert_eq!(equiv.name(), Some("TestName"));
    }

    #[test]
    fn test_set_description() {
        let mut equiv = DocumentProductEquivalence::new();
        equiv.set_description("TestDesc".to_string());
        assert_eq!(equiv.description(), Some("TestDesc"));
        assert!(equiv.has_description());
    }

    #[test]
    fn test_default() {
        let equiv = DocumentProductEquivalence::default();
        assert!(equiv.name().is_none());
    }
}
