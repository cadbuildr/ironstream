// FILE: step_basic_product_definition_reference_with_local_representation.rs
// occt: StepBasic_ProductDefinitionReferenceWithLocalRepresentation

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
#[derive(Debug, PartialEq)]
pub struct StepBasicExternalSource;
#[derive(Debug, PartialEq)]
pub struct StepBasicProductDefinitionFormation;
#[derive(Debug, PartialEq)]
pub struct StepBasicProductDefinitionContext;

/// Local mirror of the StepBasic_ProductDefinition base class
/// (id, description, formation, frame_of_reference), used as the inherited part.
pub struct StepBasicProductDefinition {
    id: Option<String>,
    description: Option<String>,
    formation: Option<Rc<RefCell<StepBasicProductDefinitionFormation>>>,
    frame_of_reference: Option<Rc<RefCell<StepBasicProductDefinitionContext>>>,
}

impl StepBasicProductDefinition {
    pub fn new() -> Self {
        StepBasicProductDefinition {
            id: None,
            description: None,
            formation: None,
            frame_of_reference: None,
        }
    }

    pub fn init(
        &mut self,
        id: String,
        description: String,
        formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
        frame_of_reference: Rc<RefCell<StepBasicProductDefinitionContext>>,
    ) {
        self.id = Some(id);
        self.description = Some(description);
        self.formation = Some(formation);
        self.frame_of_reference = Some(frame_of_reference);
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_formation(&mut self, formation: Rc<RefCell<StepBasicProductDefinitionFormation>>) {
        self.formation = Some(formation);
    }

    pub fn formation(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionFormation>>> {
        self.formation.clone()
    }

    pub fn set_frame_of_reference(
        &mut self,
        frame_of_reference: Rc<RefCell<StepBasicProductDefinitionContext>>,
    ) {
        self.frame_of_reference = Some(frame_of_reference);
    }

    pub fn frame_of_reference(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionContext>>> {
        self.frame_of_reference.clone()
    }
}

/// Represents a ProductDefinitionReferenceWithLocalRepresentation in the STEP AP standard.
///
/// Extends ProductDefinition to include an external source reference.
pub struct StepBasicProductDefinitionReferenceWithLocalRepresentation {
    base: StepBasicProductDefinition,
    source: Option<Rc<RefCell<StepBasicExternalSource>>>,
}

impl StepBasicProductDefinitionReferenceWithLocalRepresentation {
    /// Creates a new, uninitialized ProductDefinitionReferenceWithLocalRepresentation
    pub fn new() -> Self {
        StepBasicProductDefinitionReferenceWithLocalRepresentation {
            base: StepBasicProductDefinition::new(),
            source: None,
        }
    }

    /// Initializes with all required attributes
    pub fn init(
        &mut self,
        source: Rc<RefCell<StepBasicExternalSource>>,
        id: String,
        description: String,
        formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
        frame_of_reference: Rc<RefCell<StepBasicProductDefinitionContext>>,
    ) {
        self.base.init(id, description, formation, frame_of_reference);
        self.source = Some(source);
    }

    /// Returns the external source
    pub fn source(&self) -> Option<Rc<RefCell<StepBasicExternalSource>>> {
        self.source.clone()
    }

    /// Sets the external source
    pub fn set_source(&mut self, source: Rc<RefCell<StepBasicExternalSource>>) {
        self.source = Some(source);
    }

    // Delegate to base class
    pub fn set_id(&mut self, id: String) {
        self.base.set_id(id);
    }

    pub fn id(&self) -> Option<&str> {
        self.base.id()
    }

    pub fn set_description(&mut self, description: String) {
        self.base.set_description(description);
    }

    pub fn description(&self) -> Option<&str> {
        self.base.description()
    }

    pub fn set_formation(&mut self, formation: Rc<RefCell<StepBasicProductDefinitionFormation>>) {
        self.base.set_formation(formation);
    }

    pub fn formation(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionFormation>>> {
        self.base.formation()
    }

    pub fn set_frame_of_reference(
        &mut self,
        frame_of_reference: Rc<RefCell<StepBasicProductDefinitionContext>>,
    ) {
        self.base.set_frame_of_reference(frame_of_reference);
    }

    pub fn frame_of_reference(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionContext>>> {
        self.base.frame_of_reference()
    }
}

impl Default for StepBasicProductDefinitionReferenceWithLocalRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let pdr = StepBasicProductDefinitionReferenceWithLocalRepresentation::new();
        assert_eq!(pdr.id(), None);
        assert_eq!(pdr.source(), None);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut pdr = StepBasicProductDefinitionReferenceWithLocalRepresentation::new();
        pdr.set_id("PDR001".to_string());
        assert_eq!(pdr.id(), Some("PDR001"));
    }

    #[test]
    fn test_default() {
        let pdr = StepBasicProductDefinitionReferenceWithLocalRepresentation::default();
        assert_eq!(pdr.id(), None);
    }
}
