// FILE: step_basic_product_definition_effectivity.rs
// occt: StepBasic_ProductDefinitionEffectivity

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
#[derive(Debug, PartialEq)]
pub struct StepBasicProductDefinitionRelationship;

/// Base class: Effectivity
pub struct StepBasicEffectivity {
    id: Option<String>,
}

impl StepBasicEffectivity {
    pub fn new() -> Self {
        StepBasicEffectivity { id: None }
    }

    pub fn init(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

impl Default for StepBasicEffectivity {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a ProductDefinitionEffectivity in the STEP AP standard.
///
/// A ProductDefinitionEffectivity specifies the effectivity of a product
/// definition relationship with respect to a particular usage.
pub struct StepBasicProductDefinitionEffectivity {
    base: StepBasicEffectivity,
    usage: Option<Rc<RefCell<StepBasicProductDefinitionRelationship>>>,
}

impl StepBasicProductDefinitionEffectivity {
    /// Creates a new, uninitialized ProductDefinitionEffectivity
    pub fn new() -> Self {
        StepBasicProductDefinitionEffectivity {
            base: StepBasicEffectivity::new(),
            usage: None,
        }
    }

    /// Initializes the ProductDefinitionEffectivity with all required attributes
    pub fn init(
        &mut self,
        id: String,
        usage: Rc<RefCell<StepBasicProductDefinitionRelationship>>,
    ) {
        self.base.init(id);
        self.usage = Some(usage);
    }

    /// Returns the usage relationship
    pub fn usage(&self) -> Option<Rc<RefCell<StepBasicProductDefinitionRelationship>>> {
        self.usage.clone()
    }

    /// Sets the usage relationship
    pub fn set_usage(&mut self, usage: Rc<RefCell<StepBasicProductDefinitionRelationship>>) {
        self.usage = Some(usage);
    }

    // Delegate to base class
    pub fn id(&self) -> Option<&str> {
        self.base.id()
    }

    pub fn set_id(&mut self, id: String) {
        self.base.set_id(id);
    }
}

impl Default for StepBasicProductDefinitionEffectivity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_effectivity() {
        let eff = StepBasicProductDefinitionEffectivity::new();
        assert_eq!(eff.id(), None);
        assert_eq!(eff.usage(), None);
    }

    #[test]
    fn test_set_and_get_id() {
        let mut eff = StepBasicProductDefinitionEffectivity::new();
        eff.set_id("EFF001".to_string());
        assert_eq!(eff.id(), Some("EFF001"));
    }

    #[test]
    fn test_default() {
        let eff = StepBasicProductDefinitionEffectivity::default();
        assert_eq!(eff.id(), None);
    }
}
