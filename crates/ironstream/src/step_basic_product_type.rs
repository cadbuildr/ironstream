// FILE: step_basic_product_type.rs
// occt: StepBasic_ProductType

use crate::step_basic_product_related_product_category::StepBasicProductRelatedProductCategory;

/// Represents a ProductType in the STEP AP standard.
///
/// Extends ProductRelatedProductCategory to represent a specific type of product.
pub struct StepBasicProductType {
    base: StepBasicProductRelatedProductCategory,
}

impl StepBasicProductType {
    /// Creates a new, uninitialized ProductType
    pub fn new() -> Self {
        StepBasicProductType {
            base: StepBasicProductRelatedProductCategory::new(),
        }
    }

    // Delegate to base class
    pub fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }

    pub fn set_description(&mut self, description: String) {
        self.base.set_description(description);
    }

    pub fn description(&self) -> Option<&str> {
        self.base.description()
    }

    pub fn has_description(&self) -> bool {
        self.base.has_description()
    }

    pub fn nb_products(&self) -> usize {
        self.base.nb_products()
    }
}

impl Default for StepBasicProductType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let pt = StepBasicProductType::new();
        assert_eq!(pt.name(), None);
        assert_eq!(pt.nb_products(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut pt = StepBasicProductType::new();
        pt.set_name("product_type".to_string());
        assert_eq!(pt.name(), Some("product_type"));
    }

    #[test]
    fn test_default() {
        let pt = StepBasicProductType::default();
        assert_eq!(pt.name(), None);
    }
}
