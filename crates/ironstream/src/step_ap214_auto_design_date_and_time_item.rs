// FILE: step_ap214_auto_design_date_and_time_item.rs
// occt: StepAP214_AutoDesignDateAndTimeItem

/// Representation of STEP AP214 AutoDesignDateAndTimeItem SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignDateAndTimeItem {
    ApprovalPersonOrganization,
    AutoDesignDateAndPersonAssignment,
    ProductDefinitionEffectivity,
}

impl AutoDesignDateAndTimeItem {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignDateAndTimeItem::ApprovalPersonOrganization => 1,
            AutoDesignDateAndTimeItem::AutoDesignDateAndPersonAssignment => 2,
            AutoDesignDateAndTimeItem::ProductDefinitionEffectivity => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignDateAndTimeItem::ApprovalPersonOrganization.case_num(), 1);
        assert_eq!(AutoDesignDateAndTimeItem::AutoDesignDateAndPersonAssignment.case_num(), 2);
    }
}
