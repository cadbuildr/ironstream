// FILE: step_basic_application_context.rs
// occt: StepBasic_ApplicationContext

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }
}

pub struct StepBasic_ApplicationContext {
    application: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_ApplicationContext {
    pub fn new() -> Self {
        StepBasic_ApplicationContext { application: None }
    }

    pub fn init(&mut self, application: Option<Rc<RefCell<HString>>>) {
        self.application = application;
    }

    pub fn set_application(&mut self, application: Option<Rc<RefCell<HString>>>) {
        self.application = application;
    }

    pub fn application(&self) -> Option<Rc<RefCell<HString>>> {
        self.application.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ctx = StepBasic_ApplicationContext::new();
        assert!(ctx.application().is_none());
    }

    #[test]
    fn test_init() {
        let mut ctx = StepBasic_ApplicationContext::new();
        let app = HString::new("MyApp".to_string());
        ctx.init(Some(app));
        assert!(ctx.application().is_some());
    }

    #[test]
    fn test_set_application() {
        let mut ctx = StepBasic_ApplicationContext::new();
        let app = HString::new("AnotherApp".to_string());
        ctx.set_application(Some(app));
        assert!(ctx.application().is_some());
    }
}
