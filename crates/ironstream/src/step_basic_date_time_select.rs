// FILE: step_basic_date_time_select.rs
// occt: StepBasic_DateTimeSelect

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateTimeSelectKind {
    Date,
    LocalTime,
    DateAndTime,
}

#[derive(Clone, Debug)]
pub struct StepBasic_DateTimeSelect {
    kind: DateTimeSelectKind,
}

impl StepBasic_DateTimeSelect {
    pub fn new(kind: DateTimeSelectKind) -> Self {
        StepBasic_DateTimeSelect { kind }
    }

    pub fn kind(&self) -> DateTimeSelectKind {
        self.kind
    }

    pub fn case_num(&self) -> i32 {
        match self.kind {
            DateTimeSelectKind::Date => 1,
            DateTimeSelectKind::LocalTime => 2,
            DateTimeSelectKind::DateAndTime => 3,
        }
    }
}

impl Default for StepBasic_DateTimeSelect {
    fn default() -> Self {
        StepBasic_DateTimeSelect::new(DateTimeSelectKind::Date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dts = StepBasic_DateTimeSelect::new(DateTimeSelectKind::DateAndTime);
        assert_eq!(dts.kind(), DateTimeSelectKind::DateAndTime);
        assert_eq!(dts.case_num(), 3);
    }
}
