// FILE: step_basic_product_definition_reference.rs
// occt: StepBasic_ProductDefinitionReference

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicExternalSource;

/// Represents a ProductDefinitionReference in the STEP AP standard.
///
/// Specifies a reference to a product definition from an external source,
/// including product ID, formation ID, definition ID, and optionally
/// the owning organization name.
pub struct StepBasicProductDefinitionReference {
    source: Option<Rc<RefCell<StepBasicExternalSource>>>,
    product_id: Option<String>,
    product_definition_formation_id: Option<String>,
    product_definition_id: Option<String>,
    id_owning_organization_name: Option<String>,
    has_id_owning_organization_name: bool,
}

impl StepBasicProductDefinitionReference {
    /// Creates a new, uninitialized ProductDefinitionReference
    pub fn new() -> Self {
        StepBasicProductDefinitionReference {
            source: None,
            product_id: None,
            product_definition_formation_id: None,
            product_definition_id: None,
            id_owning_organization_name: None,
            has_id_owning_organization_name: false,
        }
    }

    /// Initializes with all five fields including optional organization name
    pub fn init_with_org_name(
        &mut self,
        source: Rc<RefCell<StepBasicExternalSource>>,
        product_id: String,
        product_definition_formation_id: String,
        product_definition_id: String,
        id_owning_organization_name: String,
    ) {
        self.source = Some(source);
        self.product_id = Some(product_id);
        self.product_definition_formation_id = Some(product_definition_formation_id);
        self.product_definition_id = Some(product_definition_id);
        self.id_owning_organization_name = Some(id_owning_organization_name);
        self.has_id_owning_organization_name = true;
    }

    /// Initializes with four fields (without organization name)
    pub fn init(
        &mut self,
        source: Rc<RefCell<StepBasicExternalSource>>,
        product_id: String,
        product_definition_formation_id: String,
        product_definition_id: String,
    ) {
        self.source = Some(source);
        self.product_id = Some(product_id);
        self.product_definition_formation_id = Some(product_definition_formation_id);
        self.product_definition_id = Some(product_definition_id);
        self.has_id_owning_organization_name = false;
        self.id_owning_organization_name = None;
    }

    /// Returns the external source
    pub fn source(&self) -> Option<Rc<RefCell<StepBasicExternalSource>>> {
        self.source.clone()
    }

    /// Sets the external source
    pub fn set_source(&mut self, source: Rc<RefCell<StepBasicExternalSource>>) {
        self.source = Some(source);
    }

    /// Returns the product ID
    pub fn product_id(&self) -> Option<&str> {
        self.product_id.as_deref()
    }

    /// Sets the product ID
    pub fn set_product_id(&mut self, product_id: String) {
        self.product_id = Some(product_id);
    }

    /// Returns the product definition formation ID
    pub fn product_definition_formation_id(&self) -> Option<&str> {
        self.product_definition_formation_id.as_deref()
    }

    /// Sets the product definition formation ID
    pub fn set_product_definition_formation_id(&mut self, id: String) {
        self.product_definition_formation_id = Some(id);
    }

    /// Returns the product definition ID
    pub fn product_definition_id(&self) -> Option<&str> {
        self.product_definition_id.as_deref()
    }

    /// Sets the product definition ID
    pub fn set_product_definition_id(&mut self, id: String) {
        self.product_definition_id = Some(id);
    }

    /// Returns the ID owning organization name
    pub fn id_owning_organization_name(&self) -> Option<&str> {
        self.id_owning_organization_name.as_deref()
    }

    /// Sets the ID owning organization name
    pub fn set_id_owning_organization_name(&mut self, name: String) {
        self.id_owning_organization_name = Some(name);
        self.has_id_owning_organization_name = true;
    }

    /// Returns whether the ID owning organization name is set
    pub fn has_id_owning_organization_name(&self) -> bool {
        self.has_id_owning_organization_name
    }
}

impl Default for StepBasicProductDefinitionReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let pdr = StepBasicProductDefinitionReference::new();
        assert_eq!(pdr.product_id(), None);
        assert!(!pdr.has_id_owning_organization_name());
    }

    #[test]
    fn test_set_and_get_product_id() {
        let mut pdr = StepBasicProductDefinitionReference::new();
        pdr.set_product_id("PROD001".to_string());
        assert_eq!(pdr.product_id(), Some("PROD001"));
    }

    #[test]
    fn test_set_id_owning_organization_name() {
        let mut pdr = StepBasicProductDefinitionReference::new();
        pdr.set_id_owning_organization_name("ACME".to_string());
        assert_eq!(pdr.id_owning_organization_name(), Some("ACME"));
        assert!(pdr.has_id_owning_organization_name());
    }

    #[test]
    fn test_default() {
        let pdr = StepBasicProductDefinitionReference::default();
        assert_eq!(pdr.product_id(), None);
    }
}
