// FILE: step_basic_approval_person_organization.rs
// occt: StepBasic_ApprovalPersonOrganization

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

pub struct StepBasic_ApprovalRole {
    role: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_ApprovalRole {
    pub fn new() -> Self {
        StepBasic_ApprovalRole { role: None }
    }
}

#[derive(Clone, Debug)]
pub struct StepBasic_PersonOrganizationSelect {
    variant: String,
}

impl StepBasic_PersonOrganizationSelect {
    pub fn new(variant: String) -> Self {
        StepBasic_PersonOrganizationSelect { variant }
    }
}

pub struct StepBasic_ApprovalPersonOrganization {
    person_organization: StepBasic_PersonOrganizationSelect,
    authorized_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
    role: Option<Rc<RefCell<StepBasic_ApprovalRole>>>,
}

impl StepBasic_ApprovalPersonOrganization {
    pub fn new() -> Self {
        StepBasic_ApprovalPersonOrganization {
            person_organization: StepBasic_PersonOrganizationSelect::new("".to_string()),
            authorized_approval: None,
            role: None,
        }
    }

    pub fn init(
        &mut self,
        person_organization: StepBasic_PersonOrganizationSelect,
        authorized_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
        role: Option<Rc<RefCell<StepBasic_ApprovalRole>>>,
    ) {
        self.person_organization = person_organization;
        self.authorized_approval = authorized_approval;
        self.role = role;
    }

    pub fn set_person_organization(&mut self, person_organization: StepBasic_PersonOrganizationSelect) {
        self.person_organization = person_organization;
    }

    pub fn person_organization(&self) -> StepBasic_PersonOrganizationSelect {
        self.person_organization.clone()
    }

    pub fn set_authorized_approval(&mut self, authorized_approval: Option<Rc<RefCell<StepBasic_Approval>>>) {
        self.authorized_approval = authorized_approval;
    }

    pub fn authorized_approval(&self) -> Option<Rc<RefCell<StepBasic_Approval>>> {
        self.authorized_approval.clone()
    }

    pub fn set_role(&mut self, role: Option<Rc<RefCell<StepBasic_ApprovalRole>>>) {
        self.role = role;
    }

    pub fn role(&self) -> Option<Rc<RefCell<StepBasic_ApprovalRole>>> {
        self.role.clone()
    }
}

impl Default for StepBasic_ApprovalPersonOrganization {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let apo = StepBasic_ApprovalPersonOrganization::new();
        assert!(apo.authorized_approval().is_none());
        assert!(apo.role().is_none());
    }

    #[test]
    fn test_set_person_organization() {
        let mut apo = StepBasic_ApprovalPersonOrganization::new();
        let po = StepBasic_PersonOrganizationSelect::new("person".to_string());
        apo.set_person_organization(po);
        assert_eq!(apo.person_organization().variant, "person");
    }

    #[test]
    fn test_init() {
        let mut apo = StepBasic_ApprovalPersonOrganization::new();
        let po = StepBasic_PersonOrganizationSelect::new("organization".to_string());
        let approval = Rc::new(RefCell::new(StepBasic_Approval::new()));
        apo.init(po, Some(approval), None);
        assert!(apo.authorized_approval().is_some());
    }
}
