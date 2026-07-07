// FILE: step_ap214_organization_item.rs
// occt: StepAP214_OrganizationItem

/// Representation of STEP AP214 OrganizationItem SelectType.
#[derive(Clone, Debug)]
pub enum OrganizationItem {
    AppliedOrganizationAssignment,
    Approval,
    AppliedSecurityClassificationAssignment,
}

impl OrganizationItem {
    pub fn case_num(&self) -> i32 {
        match self {
            OrganizationItem::AppliedOrganizationAssignment => 1,
            OrganizationItem::Approval => 2,
            OrganizationItem::AppliedSecurityClassificationAssignment => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(OrganizationItem::AppliedOrganizationAssignment.case_num(), 1);
        assert_eq!(OrganizationItem::AppliedSecurityClassificationAssignment.case_num(), 3);
    }
}
