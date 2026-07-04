// FILE: step_ap214_auto_design_dated_item.rs
// occt: StepAP214_AutoDesignDatedItem

/// Representation of STEP AP214 AutoDesignDatedItem SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignDatedItem {
    ApprovalPersonOrganization,
    AutoDesignDateAndPersonAssignment,
    ProductDefinitionEffectivity,
}

impl AutoDesignDatedItem {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignDatedItem::ApprovalPersonOrganization => 1,
            AutoDesignDatedItem::AutoDesignDateAndPersonAssignment => 2,
            AutoDesignDatedItem::ProductDefinitionEffectivity => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignDatedItem::ApprovalPersonOrganization.case_num(), 1);
        assert_eq!(AutoDesignDatedItem::AutoDesignDateAndPersonAssignment.case_num(), 2);
    }
}
