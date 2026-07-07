// FILE: step_basic_product_definition_with_associated_documents.rs
// occt: StepBasic_ProductDefinitionWithAssociatedDocuments

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
#[derive(Debug, PartialEq)]
pub struct StepBasicDocument;
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

/// Represents a ProductDefinitionWithAssociatedDocuments in the STEP AP standard.
///
/// Extends ProductDefinition to include an array of associated documents.
pub struct StepBasicProductDefinitionWithAssociatedDocuments {
    base: StepBasicProductDefinition,
    doc_ids: Vec<Rc<RefCell<StepBasicDocument>>>,
}

impl StepBasicProductDefinitionWithAssociatedDocuments {
    /// Creates a new, uninitialized ProductDefinitionWithAssociatedDocuments
    pub fn new() -> Self {
        StepBasicProductDefinitionWithAssociatedDocuments {
            base: StepBasicProductDefinition::new(),
            doc_ids: Vec::new(),
        }
    }

    /// Initializes with all required attributes
    pub fn init(
        &mut self,
        id: String,
        description: String,
        formation: Rc<RefCell<StepBasicProductDefinitionFormation>>,
        frame_of_reference: Rc<RefCell<StepBasicProductDefinitionContext>>,
        doc_ids: Vec<Rc<RefCell<StepBasicDocument>>>,
    ) {
        self.base.init(id, description, formation, frame_of_reference);
        self.doc_ids = doc_ids;
    }

    /// Returns the document IDs array
    pub fn doc_ids(&self) -> Vec<Rc<RefCell<StepBasicDocument>>> {
        self.doc_ids.clone()
    }

    /// Sets the document IDs array
    pub fn set_doc_ids(&mut self, doc_ids: Vec<Rc<RefCell<StepBasicDocument>>>) {
        self.doc_ids = doc_ids;
    }

    /// Returns the number of document IDs
    pub fn nb_doc_ids(&self) -> usize {
        self.doc_ids.len()
    }

    /// Returns a specific document ID by index (1-based)
    pub fn doc_ids_value(&self, index: usize) -> Option<Rc<RefCell<StepBasicDocument>>> {
        if index > 0 && index <= self.doc_ids.len() {
            Some(self.doc_ids[index - 1].clone())
        } else {
            None
        }
    }

    /// Sets a specific document ID by index (1-based)
    pub fn set_doc_ids_value(&mut self, index: usize, doc: Rc<RefCell<StepBasicDocument>>) {
        if index > 0 && index <= self.doc_ids.len() {
            self.doc_ids[index - 1] = doc;
        }
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

impl Default for StepBasicProductDefinitionWithAssociatedDocuments {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let pd = StepBasicProductDefinitionWithAssociatedDocuments::new();
        assert_eq!(pd.id(), None);
        assert_eq!(pd.nb_doc_ids(), 0);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut pd = StepBasicProductDefinitionWithAssociatedDocuments::new();
        pd.set_id("PD001".to_string());
        assert_eq!(pd.id(), Some("PD001"));
    }

    #[test]
    fn test_set_doc_ids() {
        let mut pd = StepBasicProductDefinitionWithAssociatedDocuments::new();
        let docs = vec![Rc::new(RefCell::new(StepBasicDocument))];
        pd.set_doc_ids(docs);
        assert_eq!(pd.nb_doc_ids(), 1);
    }

    #[test]
    fn test_default() {
        let pd = StepBasicProductDefinitionWithAssociatedDocuments::default();
        assert_eq!(pd.id(), None);
    }
}
