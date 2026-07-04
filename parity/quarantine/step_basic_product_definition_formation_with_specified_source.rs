// FILE: step_basic_product_definition_formation_with_specified_source.rs
// occt: StepBasic_ProductDefinitionFormationWithSpecifiedSource

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_product_definition_formation::StepBasicProductDefinitionFormation;
use crate::step_basic_source::StepBasicSource;

// Placeholder types
pub struct StepBasicProduct;

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
