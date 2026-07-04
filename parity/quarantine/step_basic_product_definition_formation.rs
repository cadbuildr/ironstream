// FILE: step_basic_product_definition_formation.rs
// occt: StepBasic_ProductDefinitionFormation

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicProduct;

/// Represents a ProductDefinitionFormation in the STEP AP standard.
///
/// A ProductDefinitionFormation specifies the formation of a product,
/// including its identifier, description, and the product it relates to.
pub struct StepBasicProductDefinitionFormation {
    id: Option<String>,
    description: Option<String>,
    of_product: Option<Rc<RefCell<StepBasicProduct>>>,
}

impl StepBasicProductDefinitionFormation {
    /// Creates a new, uninitialized ProductDefinitionFormation
    pub fn new() -> Self {
        StepBasicProductDefinitionFormation {
            id: None,
            description: None,
            of_product: None,
        }
    }

    /// Initializes the ProductDefinitionFormation with all required attributes
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

    /// Sets the formation identifier
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Returns the formation identifier
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the formation description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Returns the formation description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the product reference
    pub fn set_of_product(&mut self, of_product: Rc<RefCell<StepBasicProduct>>) {
        self.of_product = Some(of_product);
    }

    /// Returns the product reference
    pub fn of_product(&self) -> Option<Rc<RefCell<StepBasicProduct>>> {
        self.of_product.clone()
    }
}

impl Default for StepBasicProductDefinitionFormation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_formation() {
        let fmt = StepBasicProductDefinitionFormation::new();
        assert_eq!(fmt.id(), None);
        assert_eq!(fmt.description(), None);
        assert_eq!(fmt.of_product(), None);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut fmt = StepBasicProductDefinitionFormation::new();
        fmt.set_id("FMT001".to_string());
        assert_eq!(fmt.id(), Some("FMT001"));
    }

    #[test]
    fn test_set_and_get_description() {
        let mut fmt = StepBasicProductDefinitionFormation::new();
        fmt.set_description("Formation description".to_string());
        assert_eq!(fmt.description(), Some("Formation description"));
    }

    #[test]
    fn test_default() {
        let fmt = StepBasicProductDefinitionFormation::default();
        assert_eq!(fmt.id(), None);
    }
}
