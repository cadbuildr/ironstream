// FILE: step_ap214_auto_design_organization_item.rs
// occt: StepAP214_AutoDesignOrganizationItem

/// Representation of STEP AP214 AutoDesignOrganizationItem SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignOrganizationItem {
    Document,
    PhysicallyModeledProductDefinition,
}

impl AutoDesignOrganizationItem {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignOrganizationItem::Document => 1,
            AutoDesignOrganizationItem::PhysicallyModeledProductDefinition => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignOrganizationItem::Document.case_num(), 1);
        assert_eq!(AutoDesignOrganizationItem::PhysicallyModeledProductDefinition.case_num(), 2);
    }
}
