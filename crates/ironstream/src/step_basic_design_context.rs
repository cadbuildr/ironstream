// FILE: step_basic_design_context.rs
// occt: StepBasic_DesignContext

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_ProductDefinitionContext {
    name: Option<Rc<RefCell<String>>>,
}

impl StepBasic_ProductDefinitionContext {
    pub fn new() -> Self {
        StepBasic_ProductDefinitionContext { name: None }
    }
}

pub struct StepBasic_DesignContext {
    base: StepBasic_ProductDefinitionContext,
}

impl StepBasic_DesignContext {
    pub fn new() -> Self {
        StepBasic_DesignContext {
            base: StepBasic_ProductDefinitionContext::new(),
        }
    }
}

impl Default for StepBasic_DesignContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dc = StepBasic_DesignContext::new();
        assert!(dc.base.name.is_none());
    }
}
