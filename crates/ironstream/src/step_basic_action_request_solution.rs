// FILE: step_basic_action_request_solution.rs
// occt: StepBasic_ActionRequestSolution

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_ActionRequestSolution {
    method: Option<Rc<RefCell<dyn std::any::Any>>>,
    request: Option<Rc<RefCell<dyn std::any::Any>>>,
}

impl StepBasic_ActionRequestSolution {
    pub fn new() -> Self {
        StepBasic_ActionRequestSolution {
            method: None,
            request: None,
        }
    }

    pub fn init(
        &mut self,
        method: Option<Rc<RefCell<dyn std::any::Any>>>,
        request: Option<Rc<RefCell<dyn std::any::Any>>>,
    ) {
        self.method = method;
        self.request = request;
    }

    pub fn method(&self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.method.clone()
    }

    pub fn set_method(&mut self, method: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.method = method;
    }

    pub fn request(&self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.request.clone()
    }

    pub fn set_request(&mut self, request: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.request = request;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let solution = StepBasic_ActionRequestSolution::new();
        assert!(solution.method().is_none());
        assert!(solution.request().is_none());
    }

    #[test]
    fn test_init() {
        let mut solution = StepBasic_ActionRequestSolution::new();
        let method = Rc::new(RefCell::new(1));
        let request = Rc::new(RefCell::new(2));
        solution.init(Some(method), Some(request));
        assert!(solution.method().is_some());
        assert!(solution.request().is_some());
    }
}
