// FILE: step_basic_product_definition.rs
// occt: StepBasic_ProductDefinition

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a ProductDefinition in the STEP AP standard.
///
/// A product definition specifies what the product is, including its identity,
/// description, formation information, and context/frame of reference.
pub struct StepBasicProductDefinition {
    id: Option<String>,
    description: Option<String>,
    formation: Option<Rc<RefCell<StepBasicProductDefinitionFormation>>>,
    frame_of_reference: Option<Rc<RefCell<StepBasicProductDefinitionContext>>>,
}

// Placeholder types for the referenced classes
pub struct StepBasicProductDefinitionFormation;
pub struct StepBasicProductDefinitionContext;

impl StepBasicProductDefinition {
    /// Creates a new, uninitialized ProductDefinition
    pub fn new() -> Self {
        StepBasicProductDefinition {
            id: None,
            description: None,
            formation: None,
            frame_of_reference: None,
        }
    }

    /// Initializes the ProductDefinition with all required attributes
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

    /// Sets the product identifier
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Returns the product identifier
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the product description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Returns the product description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the product formation reference
    pub fn set_formation(&mut self, formation: Rc<RefCell<StepBasicProductDefinitionFormation>>) {
        self.formation = Some(formation);
    }

    /// Returns the product formation reference
    pub fn formation(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionFormation>>> {
        self.formation.clone()
    }

    /// Sets the frame of reference (context)
    pub fn set_frame_of_reference(
        &mut self,
        frame_of_reference: Rc<RefCell<StepBasicProductDefinitionContext>>,
    ) {
        self.frame_of_reference = Some(frame_of_reference);
    }

    /// Returns the frame of reference (context)
    pub fn frame_of_reference(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionContext>>> {
        self.frame_of_reference.clone()
    }
}

impl Default for StepBasicProductDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_product_definition() {
        let pd = StepBasicProductDefinition::new();
        assert_eq!(pd.id(), None);
        assert_eq!(pd.description(), None);
        assert_eq!(pd.formation(), None);
        assert_eq!(pd.frame_of_reference(), None);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut pd = StepBasicProductDefinition::new();
        pd.set_id("PROD001".to_string());
        assert_eq!(pd.id(), Some("PROD001"));
    }

    #[test]
    fn test_set_and_get_description() {
        let mut pd = StepBasicProductDefinition::new();
        pd.set_description("A test product".to_string());
        assert_eq!(pd.description(), Some("A test product"));
    }

    #[test]
    fn test_default() {
        let pd = StepBasicProductDefinition::default();
        assert_eq!(pd.id(), None);
        assert_eq!(pd.description(), None);
    }
}
