// FILE: step_basic_product_definition_formation_relationship.rs
// occt: StepBasic_ProductDefinitionFormationRelationship

use std::rc::Rc;
use std::cell::RefCell;

// Local placeholder for the referenced StepBasic_ProductDefinitionFormation entity
// (external plumbing; only referenced through shared handles here).
#[derive(Debug, PartialEq)]
pub struct StepBasicProductDefinitionFormation;

/// Represents a ProductDefinitionFormationRelationship in the STEP AP standard.
///
/// Specifies a relationship between product definition formations, including
/// an identifier, name, description, and references to related formations.
pub struct StepBasicProductDefinitionFormationRelationship {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    relating_product_definition_formation: Option<Rc<RefCell<StepBasicProductDefinitionFormation>>>,
    related_product_definition_formation: Option<Rc<RefCell<StepBasicProductDefinitionFormation>>>,
}

impl StepBasicProductDefinitionFormationRelationship {
    /// Creates a new, uninitialized ProductDefinitionFormationRelationship
    pub fn new() -> Self {
        StepBasicProductDefinitionFormationRelationship {
            id: None,
            name: None,
            description: None,
            relating_product_definition_formation: None,
            related_product_definition_formation: None,
        }
    }

    /// Initializes the ProductDefinitionFormationRelationship with all required attributes
    pub fn init(
        &mut self,
        id: String,
        name: String,
        description: String,
        relating_product_definition_formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
        related_product_definition_formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
    ) {
        self.id = Some(id);
        self.name = Some(name);
        self.description = Some(description);
        self.relating_product_definition_formation = Some(relating_product_definition_formation);
        self.related_product_definition_formation = Some(related_product_definition_formation);
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
    }

    /// Returns the relating product definition formation
    pub fn relating_product_definition_formation(
        &self,
    ) -> Option<Rc<RefCell<StepBasicProductDefinitionFormation>>> {
        self.relating_product_definition_formation.clone()
    }

    /// Sets the relating product definition formation
    pub fn set_relating_product_definition_formation(
        &mut self,
        formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
    ) {
        self.relating_product_definition_formation = Some(formation);
    }

    /// Returns the related product definition formation
    pub fn related_product_definition_formation(
        &self,
    ) -> Option<Rc<RefCell<StepBasicProductDefinitionFormation>>> {
        self.related_product_definition_formation.clone()
    }

    /// Sets the related product definition formation
    pub fn set_related_product_definition_formation(
        &mut self,
        formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
    ) {
        self.related_product_definition_formation = Some(formation);
    }
}

impl Default for StepBasicProductDefinitionFormationRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_relationship() {
        let rel = StepBasicProductDefinitionFormationRelationship::new();
        assert_eq!(rel.id(), None);
        assert_eq!(rel.name(), None);
        assert_eq!(rel.description(), None);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut rel = StepBasicProductDefinitionFormationRelationship::new();
        rel.set_id("REL001".to_string());
        assert_eq!(rel.id(), Some("REL001"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rel = StepBasicProductDefinitionFormationRelationship::new();
        rel.set_name("relationship_name".to_string());
        assert_eq!(rel.name(), Some("relationship_name"));
    }

    #[test]
    fn test_set_and_get_description() {
        let mut rel = StepBasicProductDefinitionFormationRelationship::new();
        rel.set_description("A relationship".to_string());
        assert_eq!(rel.description(), Some("A relationship"));
    }

    #[test]
    fn test_default() {
        let rel = StepBasicProductDefinitionFormationRelationship::default();
        assert_eq!(rel.id(), None);
    }
}
