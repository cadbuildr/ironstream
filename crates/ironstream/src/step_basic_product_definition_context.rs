// FILE: step_basic_product_definition_context.rs
// occt: StepBasic_ProductDefinitionContext

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicApplicationContext;

/// Represents the parent class ApplicationContextElement
pub struct StepBasicApplicationContextElement {
    name: Option<String>,
    frame_of_reference: Option<Rc<RefCell<StepBasicApplicationContext>>>,
}

impl StepBasicApplicationContextElement {
    pub fn new() -> Self {
        StepBasicApplicationContextElement {
            name: None,
            frame_of_reference: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        frame_of_reference: Rc<RefCell<StepBasicApplicationContext>>,
    ) {
        self.name = Some(name);
        self.frame_of_reference = Some(frame_of_reference);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_frame_of_reference(
        &mut self,
        frame_of_reference: Rc<RefCell<StepBasicApplicationContext>>,
    ) {
        self.frame_of_reference = Some(frame_of_reference);
    }

    pub fn frame_of_reference(&self) -> Option<Rc<RefCell<StepBasicApplicationContext>>> {
        self.frame_of_reference.clone()
    }
}

impl Default for StepBasicApplicationContextElement {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a ProductDefinitionContext in the STEP AP standard.
///
/// A ProductDefinitionContext specifies the context for a product definition,
/// including the lifecycle stage of the product.
pub struct StepBasicProductDefinitionContext {
    base: StepBasicApplicationContextElement,
    life_cycle_stage: Option<String>,
}

impl StepBasicProductDefinitionContext {
    /// Creates a new, uninitialized ProductDefinitionContext
    pub fn new() -> Self {
        StepBasicProductDefinitionContext {
            base: StepBasicApplicationContextElement::new(),
            life_cycle_stage: None,
        }
    }

    /// Initializes the ProductDefinitionContext with all required attributes
    pub fn init(
        &mut self,
        name: String,
        frame_of_reference: Rc<RefCell<StepBasicApplicationContext>>,
        life_cycle_stage: String,
    ) {
        self.base.init(name, frame_of_reference);
        self.life_cycle_stage = Some(life_cycle_stage);
    }

    /// Sets the lifecycle stage
    pub fn set_life_cycle_stage(&mut self, life_cycle_stage: String) {
        self.life_cycle_stage = Some(life_cycle_stage);
    }

    /// Returns the lifecycle stage
    pub fn life_cycle_stage(&self) -> Option<&str> {
        self.life_cycle_stage.as_deref()
    }

    // Delegate to base class
    pub fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }

    pub fn set_frame_of_reference(
        &mut self,
        frame_of_reference: Rc<RefCell<StepBasicApplicationContext>>,
    ) {
        self.base.set_frame_of_reference(frame_of_reference);
    }

    pub fn frame_of_reference(&self) -> Option<Rc<RefCell<StepBasicApplicationContext>>> {
        self.base.frame_of_reference()
    }
}

impl Default for StepBasicProductDefinitionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_context() {
        let ctx = StepBasicProductDefinitionContext::new();
        assert_eq!(ctx.name(), None);
        assert_eq!(ctx.life_cycle_stage(), None);
    }

    #[test]
    fn test_set_and_get_life_cycle_stage() {
        let mut ctx = StepBasicProductDefinitionContext::new();
        ctx.set_life_cycle_stage("design".to_string());
        assert_eq!(ctx.life_cycle_stage(), Some("design"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut ctx = StepBasicProductDefinitionContext::new();
        ctx.set_name("context_name".to_string());
        assert_eq!(ctx.name(), Some("context_name"));
    }

    #[test]
    fn test_default() {
        let ctx = StepBasicProductDefinitionContext::default();
        assert_eq!(ctx.life_cycle_stage(), None);
    }
}
