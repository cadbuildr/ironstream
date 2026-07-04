// FILE: step_basic_date_and_time.rs
// occt: StepBasic_DateAndTime

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

pub struct StepBasic_LocalTime {
    hour: i32,
}

impl StepBasic_LocalTime {
    pub fn new() -> Self {
        StepBasic_LocalTime { hour: 0 }
    }
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

    pub fn init(
        &mut self,
        date_component: Option<Rc<RefCell<StepBasic_Date>>>,
        time_component: Option<Rc<RefCell<StepBasic_LocalTime>>>,
    ) {
        self.date_component = date_component;
        self.time_component = time_component;
    }

    pub fn set_date_component(&mut self, date_component: Option<Rc<RefCell<StepBasic_Date>>>) {
        self.date_component = date_component;
    }

    pub fn date_component(&self) -> Option<Rc<RefCell<StepBasic_Date>>> {
        self.date_component.clone()
    }

    pub fn set_time_component(&mut self, time_component: Option<Rc<RefCell<StepBasic_LocalTime>>>) {
        self.time_component = time_component;
    }

    pub fn time_component(&self) -> Option<Rc<RefCell<StepBasic_LocalTime>>> {
        self.time_component.clone()
    }
}

impl Default for StepBasic_DateAndTime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dt = StepBasic_DateAndTime::new();
        assert!(dt.date_component().is_none());
        assert!(dt.time_component().is_none());
    }
}
