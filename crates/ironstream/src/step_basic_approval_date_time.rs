// FILE: step_basic_approval_date_time.rs
// occt: StepBasic_ApprovalDateTime

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

pub struct StepBasic_ApprovalStatus;

pub struct StepBasic_Approval {
    status: Option<Rc<RefCell<StepBasic_ApprovalStatus>>>,
    level: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_Approval {
    pub fn new() -> Self {
        StepBasic_Approval {
            status: None,
            level: None,
        }
    }
}

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
}

pub struct StepBasic_ApprovalDateTime {
    date_time: StepBasic_DateTimeSelect,
    dated_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
}

impl StepBasic_ApprovalDateTime {
    pub fn new() -> Self {
        StepBasic_ApprovalDateTime {
            date_time: StepBasic_DateTimeSelect::new(DateTimeSelectKind::Date),
            dated_approval: None,
        }
    }

    pub fn init(
        &mut self,
        date_time: StepBasic_DateTimeSelect,
        dated_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
    ) {
        self.date_time = date_time;
        self.dated_approval = dated_approval;
    }

    pub fn set_date_time(&mut self, date_time: StepBasic_DateTimeSelect) {
        self.date_time = date_time;
    }

    pub fn date_time(&self) -> StepBasic_DateTimeSelect {
        self.date_time.clone()
    }

    pub fn set_dated_approval(&mut self, dated_approval: Option<Rc<RefCell<StepBasic_Approval>>>) {
        self.dated_approval = dated_approval;
    }

    pub fn dated_approval(&self) -> Option<Rc<RefCell<StepBasic_Approval>>> {
        self.dated_approval.clone()
    }
}

impl Default for StepBasic_ApprovalDateTime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let adt = StepBasic_ApprovalDateTime::new();
        assert_eq!(adt.date_time().kind(), DateTimeSelectKind::Date);
        assert!(adt.dated_approval().is_none());
    }

    #[test]
    fn test_set_date_time() {
        let mut adt = StepBasic_ApprovalDateTime::new();
        let dt = StepBasic_DateTimeSelect::new(DateTimeSelectKind::DateAndTime);
        adt.set_date_time(dt);
        assert_eq!(adt.date_time().kind(), DateTimeSelectKind::DateAndTime);
    }

    #[test]
    fn test_set_dated_approval() {
        let mut adt = StepBasic_ApprovalDateTime::new();
        let approval = Rc::new(RefCell::new(StepBasic_Approval::new()));
        adt.set_dated_approval(Some(approval));
        assert!(adt.dated_approval().is_some());
    }
}
