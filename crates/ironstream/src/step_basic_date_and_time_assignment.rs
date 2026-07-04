// FILE: step_basic_date_and_time_assignment.rs
// occt: StepBasic_DateAndTimeAssignment

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_Date {
    year_component: i32,
}

pub struct StepBasic_LocalTime {
    hour: i32,
}

pub struct StepBasic_DateAndTime {
    date_component: Option<Rc<RefCell<StepBasic_Date>>>,
    time_component: Option<Rc<RefCell<StepBasic_LocalTime>>>,
}

impl StepBasic_DateAndTime {
    pub fn new() -> Self {
        StepBasic_DateAndTime {
            date_component: None,
            time_component: None,
        }
    }
}

pub struct StepBasic_DateTimeRole {
    name: Option<Rc<RefCell<String>>>,
}

impl StepBasic_DateTimeRole {
    pub fn new() -> Self {
        StepBasic_DateTimeRole { name: None }
    }
}

pub struct StepBasic_DateAndTimeAssignment {
    assigned_date_and_time: Option<Rc<RefCell<StepBasic_DateAndTime>>>,
    role: Option<Rc<RefCell<StepBasic_DateTimeRole>>>,
}

impl StepBasic_DateAndTimeAssignment {
    pub fn new() -> Self {
        StepBasic_DateAndTimeAssignment {
            assigned_date_and_time: None,
            role: None,
        }
    }

    pub fn init(
        &mut self,
        assigned_date_and_time: Option<Rc<RefCell<StepBasic_DateAndTime>>>,
        role: Option<Rc<RefCell<StepBasic_DateTimeRole>>>,
    ) {
        self.assigned_date_and_time = assigned_date_and_time;
        self.role = role;
    }

    pub fn set_assigned_date_and_time(&mut self, assigned_date_and_time: Option<Rc<RefCell<StepBasic_DateAndTime>>>) {
        self.assigned_date_and_time = assigned_date_and_time;
    }

    pub fn assigned_date_and_time(&self) -> Option<Rc<RefCell<StepBasic_DateAndTime>>> {
        self.assigned_date_and_time.clone()
    }

    pub fn set_role(&mut self, role: Option<Rc<RefCell<StepBasic_DateTimeRole>>>) {
        self.role = role;
    }

    pub fn role(&self) -> Option<Rc<RefCell<StepBasic_DateTimeRole>>> {
        self.role.clone()
    }
}

impl Default for StepBasic_DateAndTimeAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let data = StepBasic_DateAndTimeAssignment::new();
        assert!(data.assigned_date_and_time().is_none());
        assert!(data.role().is_none());
    }
}
