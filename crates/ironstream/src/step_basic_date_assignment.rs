// FILE: step_basic_date_assignment.rs
// occt: StepBasic_DateAssignment

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_Date {
    year_component: i32,
}

impl StepBasic_Date {
    pub fn new() -> Self {
        StepBasic_Date {
            year_component: 0,
        }
    }
}

pub struct StepBasic_DateRole {
    name: Option<Rc<RefCell<String>>>,
}

impl StepBasic_DateRole {
    pub fn new() -> Self {
        StepBasic_DateRole { name: None }
    }
}

pub struct StepBasic_DateAssignment {
    assigned_date: Option<Rc<RefCell<StepBasic_Date>>>,
    role: Option<Rc<RefCell<StepBasic_DateRole>>>,
}

impl StepBasic_DateAssignment {
    pub fn new() -> Self {
        StepBasic_DateAssignment {
            assigned_date: None,
            role: None,
        }
    }

    pub fn init(
        &mut self,
        assigned_date: Option<Rc<RefCell<StepBasic_Date>>>,
        role: Option<Rc<RefCell<StepBasic_DateRole>>>,
    ) {
        self.assigned_date = assigned_date;
        self.role = role;
    }

    pub fn set_assigned_date(&mut self, assigned_date: Option<Rc<RefCell<StepBasic_Date>>>) {
        self.assigned_date = assigned_date;
    }

    pub fn assigned_date(&self) -> Option<Rc<RefCell<StepBasic_Date>>> {
        self.assigned_date.clone()
    }

    pub fn set_role(&mut self, role: Option<Rc<RefCell<StepBasic_DateRole>>>) {
        self.role = role;
    }

    pub fn role(&self) -> Option<Rc<RefCell<StepBasic_DateRole>>> {
        self.role.clone()
    }
}

impl Default for StepBasic_DateAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let da = StepBasic_DateAssignment::new();
        assert!(da.assigned_date().is_none());
        assert!(da.role().is_none());
    }
}
