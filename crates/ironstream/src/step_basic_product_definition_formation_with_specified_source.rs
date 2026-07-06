// FILE: step_basic_product_definition_formation_with_specified_source.rs
// occt: StepBasic_ProductDefinitionFormationWithSpecifiedSource

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
#[derive(Debug, PartialEq)]
pub struct StepBasicProduct;

/// Local mirror of the StepBasic_Source enumeration
/// (StepBasic_sMade, StepBasic_sBought, StepBasic_sNotKnown).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasicSource {
    Made,
    Bought,
    NotKnown,
}

/// Local mirror of the StepBasic_ProductDefinitionFormation base class
/// (id, description, of_product), used as the inherited part of this class.
pub struct StepBasicProductDefinitionFormation {
    id: Option<String>,
    description: Option<String>,
    of_product: Option<Rc<RefCell<StepBasicProduct>>>,
}

impl StepBasicProductDefinitionFormation {
    pub fn new() -> Self {
        StepBasicProductDefinitionFormation {
            id: None,
            description: None,
            of_product: None,
        }
    }

    pub fn init(
        &mut self,
        id: String,
        description: String,
        of_product: Rc<RefCell<StepBasicProduct>>,
    ) {
        self.id = Some(id);
        self.description = Some(description);
        self.of_product = Some(of_product);
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

    pub fn set_of_product(&mut self, of_product: Rc<RefCell<StepBasicProduct>>) {
        self.of_product = Some(of_product);
    }

    pub fn of_product(&self) -> Option<Rc<RefCell<StepBasicProduct>>> {
        self.of_product.clone()
    }
}

/// Represents a ProductDefinitionFormationWithSpecifiedSource in the STEP AP standard.
///
/// Extends ProductDefinitionFormation to specify whether a product is made or bought.
pub struct StepBasicProductDefinitionFormationWithSpecifiedSource {
    base: StepBasicProductDefinitionFormation,
    make_or_buy: StepBasicSource,
}

impl StepBasicProductDefinitionFormationWithSpecifiedSource {
    /// Creates a new, uninitialized ProductDefinitionFormationWithSpecifiedSource
    pub fn new() -> Self {
        StepBasicProductDefinitionFormationWithSpecifiedSource {
            base: StepBasicProductDefinitionFormation::new(),
            make_or_buy: StepBasicSource::NotKnown,
        }
    }

    /// Initializes the ProductDefinitionFormationWithSpecifiedSource with all required attributes
    pub fn init(
        &mut self,
        id: String,
        description: String,
        of_product: Rc<RefCell<StepBasicProduct>>,
        make_or_buy: StepBasicSource,
    ) {
        self.base.init(id, description, of_product);
        self.make_or_buy = make_or_buy;
    }

    /// Sets the make or buy source
    pub fn set_make_or_buy(&mut self, make_or_buy: StepBasicSource) {
        self.make_or_buy = make_or_buy;
    }

    /// Returns the make or buy source
    pub fn make_or_buy(&self) -> StepBasicSource {
        self.make_or_buy.clone()
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

    pub fn set_of_product(&mut self, of_product: Rc<RefCell<StepBasicProduct>>) {
        self.base.set_of_product(of_product);
    }

    pub fn of_product(&self) -> Option<Rc<RefCell<StepBasicProduct>>> {
        self.base.of_product()
    }
}

impl Default for StepBasicProductDefinitionFormationWithSpecifiedSource {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let fmt = StepBasicProductDefinitionFormationWithSpecifiedSource::new();
        assert_eq!(fmt.id(), None);
        assert_eq!(fmt.make_or_buy(), StepBasicSource::NotKnown);
    }

    #[test]
    fn test_set_and_get_make_or_buy() {
        let mut fmt = StepBasicProductDefinitionFormationWithSpecifiedSource::new();
        fmt.set_make_or_buy(StepBasicSource::Made);
        assert_eq!(fmt.make_or_buy(), StepBasicSource::Made);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut fmt = StepBasicProductDefinitionFormationWithSpecifiedSource::new();
        fmt.set_id("FMT001".to_string());
        assert_eq!(fmt.id(), Some("FMT001"));
    }

    #[test]
    fn test_default() {
        let fmt = StepBasicProductDefinitionFormationWithSpecifiedSource::default();
        assert_eq!(fmt.id(), None);
    }
}
