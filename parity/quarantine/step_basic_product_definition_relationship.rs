// FILE: step_basic_product_definition_relationship.rs
// occt: StepBasic_ProductDefinitionRelationship

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_product_definition_or_reference::StepBasicProductDefinitionOrReference;

// Placeholder types
pub struct StepBasicProductDefinition;

/// Represents a ProductDefinitionRelationship in the STEP AP standard.
///
/// Specifies a relationship between product definitions, including
/// identifier, name, optional description, and related definitions.
pub struct StepBasicProductDefinitionRelationship {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
    relating_product_definition: StepBasicProductDefinitionOrReference,
    related_product_definition: StepBasicProductDefinitionOrReference,
}

impl StepBasicProductDefinitionRelationship {
    /// Creates a new, uninitialized ProductDefinitionRelationship
    pub fn new() -> Self {
        StepBasicProductDefinitionRelationship {
            id: None,
            name: None,
            description: None,
            has_description: false,
            relating_product_definition: StepBasicProductDefinitionOrReference::default(),
            related_product_definition: StepBasicProductDefinitionOrReference::default(),
        }
    }

    /// Initializes with StepBasic_ProductDefinition handles
    pub fn init_with_handles(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_product_definition: Rc<RefCell<StepBasicProductDefinition>>,
        related_product_definition: Rc<RefCell<StepBasicProductDefinition>>,
    ) {
        self.id = Some(id);
        self.name = Some(name);
        self.has_description = has_description;
        self.description = description;
        self.relating_product_definition =
            StepBasicProductDefinitionOrReference::ProductDefinition(relating_product_definition);
        self.related_product_definition =
            StepBasicProductDefinitionOrReference::ProductDefinition(related_product_definition);
    }

    /// Initializes with StepBasic_ProductDefinitionOrReference (AP242)
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_product_definition: StepBasicProductDefinitionOrReference,
        related_product_definition: StepBasicProductDefinitionOrReference,
    ) {
        self.id = Some(id);
        self.name = Some(name);
        self.has_description = has_description;
        self.description = description;
        self.relating_product_definition = relating_product_definition;
        self.related_product_definition = related_product_definition;
    }

    /// Returns the identifier
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the identifier
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Returns the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Returns the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.has_description = true;
    }

    /// Returns whether the description field is defined
    pub fn has_description(&self) -> bool {
        self.has_description
    }

    /// Returns the relating product definition
    pub fn relating_product_definition(&self) -> StepBasicProductDefinitionOrReference {
        self.relating_product_definition.clone()
    }

    /// Returns the relating product definition (AP242)
    pub fn relating_product_definition_ap242(&self) -> StepBasicProductDefinitionOrReference {
        self.relating_product_definition.clone()
    }

    /// Sets the relating product definition
    pub fn set_relating_product_definition(
        &mut self,
        definition: Rc<RefCell<StepBasicProductDefinition>>,
    ) {
        self.relating_product_definition =
            StepBasicProductDefinitionOrReference::ProductDefinition(definition);
    }

    /// Sets the relating product definition (AP242)
    pub fn set_relating_product_definition_ap242(
        &mut self,
        definition: StepBasicProductDefinitionOrReference,
    ) {
        self.relating_product_definition = definition;
    }

    /// Returns the related product definition
    pub fn related_product_definition(&self) -> StepBasicProductDefinitionOrReference {
        self.related_product_definition.clone()
    }

    /// Returns the related product definition (AP242)
    pub fn related_product_definition_ap242(&self) -> StepBasicProductDefinitionOrReference {
        self.related_product_definition.clone()
    }

    /// Sets the related product definition
    pub fn set_related_product_definition(
        &mut self,
        definition: Rc<RefCell<StepBasicProductDefinition>>,
    ) {
        self.related_product_definition =
            StepBasicProductDefinitionOrReference::ProductDefinition(definition);
    }

    /// Sets the related product definition (AP242)
    pub fn set_related_product_definition_ap242(
        &mut self,
        definition: StepBasicProductDefinitionOrReference,
    ) {
        self.related_product_definition = definition;
    }
}

impl Default for StepBasicProductDefinitionRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let rel = StepBasicProductDefinitionRelationship::new();
        assert_eq!(rel.id(), None);
        assert_eq!(rel.name(), None);
        assert!(!rel.has_description());
    }

    #[test]
    fn test_set_and_get_id() {
        let mut rel = StepBasicProductDefinitionRelationship::new();
        rel.set_id("REL001".to_string());
        assert_eq!(rel.id(), Some("REL001"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rel = StepBasicProductDefinitionRelationship::new();
        rel.set_name("relationship".to_string());
        assert_eq!(rel.name(), Some("relationship"));
    }

    #[test]
    fn test_set_description() {
        let mut rel = StepBasicProductDefinitionRelationship::new();
        rel.set_description("A description".to_string());
        assert_eq!(rel.description(), Some("A description"));
        assert!(rel.has_description());
    }

    #[test]
    fn test_default() {
        let rel = StepBasicProductDefinitionRelationship::default();
        assert_eq!(rel.id(), None);
    }
}
