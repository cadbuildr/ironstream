// FILE: step_basic_product_or_formation_or_definition.rs
// occt: StepBasic_ProductOrFormationOrDefinition

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicProduct;
pub struct StepBasicProductDefinitionFormation;
pub struct StepBasicProductDefinition;

/// Represents a ProductOrFormationOrDefinition in the STEP AP standard.
///
/// A select type that can hold one of:
/// 1. Product
/// 2. ProductDefinitionFormation
/// 3. ProductDefinition
#[derive(Clone)]
pub enum StepBasicProductOrFormationOrDefinition {
    Product(Rc<RefCell<StepBasicProduct>>),
    ProductDefinitionFormation(Rc<RefCell<StepBasicProductDefinitionFormation>>),
    ProductDefinition(Rc<RefCell<StepBasicProductDefinition>>),
}

impl StepBasicProductOrFormationOrDefinition {
    /// Creates a new ProductOrFormationOrDefinition
    pub fn new() -> Self {
        StepBasicProductOrFormationOrDefinition::Product(Rc::new(RefCell::new(
            StepBasicProduct,
        )))
    }

    /// Returns the case number for the current type:
    /// 1 -> Product
    /// 2 -> ProductDefinitionFormation
    /// 3 -> ProductDefinition
    /// 0 -> unknown
    pub fn case_num(&self) -> i32 {
        match self {
            StepBasicProductOrFormationOrDefinition::Product(_) => 1,
            StepBasicProductOrFormationOrDefinition::ProductDefinitionFormation(_) => 2,
            StepBasicProductOrFormationOrDefinition::ProductDefinition(_) => 3,
        }
    }

    /// Returns the value as a Product (None if another type)
    pub fn product(&self) -> Option<Rc<RefCell<StepBasicProduct>>> {
        match self {
            StepBasicProductOrFormationOrDefinition::Product(p) => Some(p.clone()),
            _ => None,
        }
    }

    /// Returns the value as a ProductDefinitionFormation (None if another type)
    pub fn product_definition_formation(
        &self,
    ) -> Option<Rc<RefCell<StepBasicProductDefinitionFormation>>> {
        match self {
            StepBasicProductOrFormationOrDefinition::ProductDefinitionFormation(pdf) => {
                Some(pdf.clone())
            }
            _ => None,
        }
    }

    /// Returns the value as a ProductDefinition (None if another type)
    pub fn product_definition(&self) -> Option<Rc<RefCell<StepBasicProductDefinition>>> {
        match self {
            StepBasicProductOrFormationOrDefinition::ProductDefinition(pd) => Some(pd.clone()),
            _ => None,
        }
    }
}

impl Default for StepBasicProductOrFormationOrDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let sel = StepBasicProductOrFormationOrDefinition::new();
        assert_eq!(sel.case_num(), 1);
    }

    #[test]
    fn test_case_num_product() {
        let sel = StepBasicProductOrFormationOrDefinition::Product(Rc::new(RefCell::new(
            StepBasicProduct,
        )));
        assert_eq!(sel.case_num(), 1);
        assert!(sel.product().is_some());
    }

    #[test]
    fn test_case_num_product_definition_formation() {
        let sel = StepBasicProductOrFormationOrDefinition::ProductDefinitionFormation(Rc::new(
            RefCell::new(StepBasicProductDefinitionFormation),
        ));
        assert_eq!(sel.case_num(), 2);
        assert!(sel.product_definition_formation().is_some());
    }

    #[test]
    fn test_case_num_product_definition() {
        let sel = StepBasicProductOrFormationOrDefinition::ProductDefinition(Rc::new(
            RefCell::new(StepBasicProductDefinition),
        ));
        assert_eq!(sel.case_num(), 3);
        assert!(sel.product_definition().is_some());
    }

    #[test]
    fn test_default() {
        let sel = StepBasicProductOrFormationOrDefinition::default();
        assert_eq!(sel.case_num(), 1);
    }
}
