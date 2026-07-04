// FILE: step_basic_approval_relationship.rs
// occt: StepBasic_ApprovalRelationship

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }

    pub fn as_str(&self) -> &str {
        &self.value
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

pub struct StepBasic_ApprovalRelationship {
    name: Option<Rc<RefCell<HString>>>,
    description: Option<Rc<RefCell<HString>>>,
    relating_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
    related_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
}

impl StepBasic_ApprovalRelationship {
    pub fn new() -> Self {
        StepBasic_ApprovalRelationship {
            name: None,
            description: None,
            relating_approval: None,
            related_approval: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        description: Option<Rc<RefCell<HString>>>,
        relating_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
        related_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
    ) {
        self.name = name;
        self.description = description;
        self.relating_approval = relating_approval;
        self.related_approval = related_approval;
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_description(&mut self, description: Option<Rc<RefCell<HString>>>) {
        self.description = description;
    }

    pub fn description(&self) -> Option<Rc<RefCell<HString>>> {
        self.description.clone()
    }

    pub fn set_relating_approval(&mut self, relating_approval: Option<Rc<RefCell<StepBasic_Approval>>>) {
        self.relating_approval = relating_approval;
    }

    pub fn relating_approval(&self) -> Option<Rc<RefCell<StepBasic_Approval>>> {
        self.relating_approval.clone()
    }

    pub fn set_related_approval(&mut self, related_approval: Option<Rc<RefCell<StepBasic_Approval>>>) {
        self.related_approval = related_approval;
    }

    pub fn related_approval(&self) -> Option<Rc<RefCell<StepBasic_Approval>>> {
        self.related_approval.clone()
    }
}

impl Default for StepBasic_ApprovalRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ar = StepBasic_ApprovalRelationship::new();
        assert!(ar.name().is_none());
        assert!(ar.description().is_none());
        assert!(ar.relating_approval().is_none());
        assert!(ar.related_approval().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut ar = StepBasic_ApprovalRelationship::new();
        let name = HString::new("relation".to_string());
        ar.set_name(Some(name));
        assert!(ar.name().is_some());
    }

    #[test]
    fn test_init() {
        let mut ar = StepBasic_ApprovalRelationship::new();
        let name = HString::new("test_relation".to_string());
        let desc = HString::new("test description".to_string());
        let app1 = Rc::new(RefCell::new(StepBasic_Approval::new()));
        let app2 = Rc::new(RefCell::new(StepBasic_Approval::new()));
        ar.init(Some(name), Some(desc), Some(app1), Some(app2));
        assert!(ar.name().is_some());
        assert!(ar.description().is_some());
        assert!(ar.relating_approval().is_some());
        assert!(ar.related_approval().is_some());
    }
}
