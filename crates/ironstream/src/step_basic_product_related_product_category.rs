// FILE: step_basic_product_related_product_category.rs
// occt: StepBasic_ProductRelatedProductCategory

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicProduct;

/// Base class: ProductCategory
pub struct StepBasicProductCategory {
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
}

impl StepBasicProductCategory {
    pub fn new() -> Self {
        StepBasicProductCategory {
            name: None,
            description: None,
            has_description: false,
        }
    }

    pub fn init(&mut self, name: String, has_description: bool, description: Option<String>) {
        self.name = Some(name);
        self.has_description = has_description;
        self.description = description;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.has_description = true;
    }

    pub fn unset_description(&mut self) {
        self.description = None;
        self.has_description = false;
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for StepBasicProductCategory {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a ProductRelatedProductCategory in the STEP AP standard.
///
/// Extends ProductCategory to include an array of related products.
pub struct StepBasicProductRelatedProductCategory {
    base: StepBasicProductCategory,
    products: Vec<Rc<RefCell<StepBasicProduct>>>,
}

impl StepBasicProductRelatedProductCategory {
    /// Creates a new, uninitialized ProductRelatedProductCategory
    pub fn new() -> Self {
        StepBasicProductRelatedProductCategory {
            base: StepBasicProductCategory::new(),
            products: Vec::new(),
        }
    }

    /// Initializes with all required attributes
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        products: Vec<Rc<RefCell<StepBasicProduct>>>,
    ) {
        self.base.init(name, has_description, description);
        self.products = products;
    }

    /// Sets the products array
    pub fn set_products(&mut self, products: Vec<Rc<RefCell<StepBasicProduct>>>) {
        self.products = products;
    }

    /// Returns the products array
    pub fn products(&self) -> Vec<Rc<RefCell<StepBasicProduct>>> {
        self.products.clone()
    }

    /// Returns a specific product by index (1-based)
    pub fn products_value(&self, index: usize) -> Option<Rc<RefCell<StepBasicProduct>>> {
        if index > 0 && index <= self.products.len() {
            Some(self.products[index - 1].clone())
        } else {
            None
        }
    }

    /// Returns the number of products
    pub fn nb_products(&self) -> usize {
        self.products.len()
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
}

impl Default for StepBasicProductRelatedProductCategory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let prpc = StepBasicProductRelatedProductCategory::new();
        assert_eq!(prpc.name(), None);
        assert_eq!(prpc.nb_products(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut prpc = StepBasicProductRelatedProductCategory::new();
        prpc.set_name("category".to_string());
        assert_eq!(prpc.name(), Some("category"));
    }

    #[test]
    fn test_set_products() {
        let mut prpc = StepBasicProductRelatedProductCategory::new();
        let products = vec![Rc::new(RefCell::new(StepBasicProduct))];
        prpc.set_products(products);
        assert_eq!(prpc.nb_products(), 1);
    }

    #[test]
    fn test_default() {
        let prpc = StepBasicProductRelatedProductCategory::default();
        assert_eq!(prpc.name(), None);
    }
}
