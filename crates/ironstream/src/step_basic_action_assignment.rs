// FILE: step_basic_action_assignment.rs
// occt: StepBasic_ActionAssignment

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_ActionAssignment {
    assigned_action: Option<Rc<RefCell<dyn std::any::Any>>>,
}

impl StepBasic_ActionAssignment {
    pub fn new() -> Self {
        StepBasic_ActionAssignment {
            assigned_action: None,
        }
    }

    pub fn init(&mut self, assigned_action: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.assigned_action = assigned_action;
    }

    pub fn assigned_action(&self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.assigned_action.clone()
    }

    pub fn set_assigned_action(&mut self, assigned_action: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.assigned_action = assigned_action;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_assignment_creation() {
        let assignment = StepBasic_ActionAssignment::new();
        assert!(assignment.assigned_action().is_none());
    }

    #[test]
    fn test_action_assignment_init() {
        let mut assignment = StepBasic_ActionAssignment::new();
        let action = Rc::new(RefCell::new(42));
        assignment.init(Some(action.clone()));
        assert!(assignment.assigned_action().is_some());
    }

    #[test]
    fn test_action_assignment_set() {
        let mut assignment = StepBasic_ActionAssignment::new();
        let action = Rc::new(RefCell::new("test"));
        assignment.set_assigned_action(Some(action.clone()));
        assert!(assignment.assigned_action().is_some());
    }
}
