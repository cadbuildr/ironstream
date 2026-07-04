// FILE: step_basic_action_request_assignment.rs
// occt: StepBasic_ActionRequestAssignment

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_ActionRequestAssignment {
    assigned_action_request: Option<Rc<RefCell<dyn std::any::Any>>>,
}

impl StepBasic_ActionRequestAssignment {
    pub fn new() -> Self {
        StepBasic_ActionRequestAssignment {
            assigned_action_request: None,
        }
    }

    pub fn init(&mut self, assigned_action_request: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.assigned_action_request = assigned_action_request;
    }

    pub fn assigned_action_request(&self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.assigned_action_request.clone()
    }

    pub fn set_assigned_action_request(
        &mut self,
        assigned_action_request: Option<Rc<RefCell<dyn std::any::Any>>>,
    ) {
        self.assigned_action_request = assigned_action_request;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let assignment = StepBasic_ActionRequestAssignment::new();
        assert!(assignment.assigned_action_request().is_none());
    }

    #[test]
    fn test_init() {
        let mut assignment = StepBasic_ActionRequestAssignment::new();
        let request = Rc::new(RefCell::new(42));
        assignment.init(Some(request.clone()));
        assert!(assignment.assigned_action_request().is_some());
    }
}
