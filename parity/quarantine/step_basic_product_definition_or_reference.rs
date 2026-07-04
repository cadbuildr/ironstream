// FILE: step_basic_product_definition_or_reference.rs
// occt: StepBasic_ProductDefinitionOrReference

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicProductDefinition;
pub struct StepBasicProductDefinitionReference;
pub struct StepBasicProductDefinitionReferenceWithLocalRepresentation;

/// Represents a ProductDefinitionOrReference in the STEP AP standard.
///
/// A select type that can hold one of:
/// 1. ProductDefinition
/// 2. ProductDefinitionReference
/// 3. ProductDefinitionReferenceWithLocalRepresentation
#[derive(Clone)]
pub enum StepBasicProductDefinitionOrReference {
    ProductDefinition(Rc<RefCell<StepBasicProductDefinition>>),
    ProductDefinitionReference(Rc<RefCell<StepBasicProductDefinitionReference>>),
    ProductDefinitionReferenceWithLocalRepresentation(
        Rc<RefCell<StepBasicProductDefinitionReferenceWithLocalRepresentation>>,
    ),
}

impl StepBasicProductDefinitionOrReference {
    /// Creates a new ProductDefinitionOrReference
    pub fn new() -> Self {
        // Return a default variant - need to pick one. Default to None-like state.
        // This is unusual for a select type but we need a default constructor.
        // In OCCT this would be an uninitialized state.
        // We'll use ProductDefinition as the default, but document that it should be set.
        StepBasicProductDefinitionOrReference::ProductDefinition(Rc::new(RefCell::new(
            StepBasicProductDefinition,
        )))
    }

    /// Returns the case number for the current type:
    /// 1 -> ProductDefinition
    /// 2 -> ProductDefinitionReference
    /// 3 -> ProductDefinitionReferenceWithLocalRepresentation
    /// 0 -> unknown
    pub fn case_num(&self) -> i32 {
        match self {
            StepBasicProductDefinitionOrReference::ProductDefinition(_) => 1,
            StepBasicProductDefinitionOrReference::ProductDefinitionReference(_) => 2,
            StepBasicProductDefinitionOrReference::ProductDefinitionReferenceWithLocalRepresentation(_) => 3,
        }
    }

    /// Returns the value as a ProductDefinition (None if another type)
    pub fn product_definition(
        &self,
    ) -> Option<Rc<RefCell<StepBasicProductDefinition>>> {
        match self {
            StepBasicProductDefinitionOrReference::ProductDefinition(pd) => Some(pd.clone()),
            _ => None,
        }
    }

    /// Returns the value as a ProductDefinitionReference (None if another type)
    pub fn product_definition_reference(
        &self,
    ) -> Option<Rc<RefCell<StepBasicProductDefinitionReference>>> {
        match self {
            StepBasicProductDefinitionOrReference::ProductDefinitionReference(pdr) => Some(pdr.clone()),
            _ => None,
        }
    }

    /// Returns the value as a ProductDefinitionReferenceWithLocalRepresentation (None if another type)
    pub fn product_definition_reference_with_local_representation(
        &self,
    ) -> Option<Rc<RefCell<StepBasicProductDefinitionReferenceWithLocalRepresentation>>> {
        match self {
            StepBasicProductDefinitionOrReference::ProductDefinitionReferenceWithLocalRepresentation(
                pdrlr,
            ) => Some(pdrlr.clone()),
            _ => None,
        }
    }
}

impl Default for StepBasicProductDefinitionOrReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let sel = StepBasicProductDefinitionOrReference::new();
        assert_eq!(sel.case_num(), 1);
    }

    #[test]
    fn test_case_num_product_definition() {
        let sel = StepBasicProductDefinitionOrReference::ProductDefinition(Rc::new(
            RefCell::new(StepBasicProductDefinition),
        ));
        assert_eq!(sel.case_num(), 1);
        assert!(sel.product_definition().is_some());
    }

    #[test]
    fn test_case_num_product_definition_reference() {
        let sel = StepBasicProductDefinitionOrReference::ProductDefinitionReference(Rc::new(
            RefCell::new(StepBasicProductDefinitionReference),
        ));
        assert_eq!(sel.case_num(), 2);
        assert!(sel.product_definition_reference().is_some());
    }

    #[test]
    fn test_case_num_product_definition_reference_with_local_representation() {
        let sel =
            StepBasicProductDefinitionOrReference::ProductDefinitionReferenceWithLocalRepresentation(
                Rc::new(RefCell::new(
                    StepBasicProductDefinitionReferenceWithLocalRepresentation,
                )),
            );
        assert_eq!(sel.case_num(), 3);
        assert!(sel
            .product_definition_reference_with_local_representation()
            .is_some());
    }

    #[test]
    fn test_default() {
        let sel = StepBasicProductDefinitionOrReference::default();
        assert_eq!(sel.case_num(), 1);
    }
}
