// FILE: step_ap214_auto_design_general_org_item.rs
// occt: StepAP214_AutoDesignGeneralOrgItem

/// Representation of STEP AP214 AutoDesignGeneralOrgItem SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignGeneralOrgItem {
    Product,
    ProductDefinition,
    ProductDefinitionFormation,
    ProductDefinitionRelationship,
    ProductDefinitionWithAssociatedDocuments,
    Representation,
    ExternallyDefinedRepresentation,
    AutoDesignDocumentReference,
}

impl AutoDesignGeneralOrgItem {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignGeneralOrgItem::Product => 1,
            AutoDesignGeneralOrgItem::ProductDefinition => 2,
            AutoDesignGeneralOrgItem::ProductDefinitionFormation => 3,
            AutoDesignGeneralOrgItem::ProductDefinitionRelationship => 4,
            AutoDesignGeneralOrgItem::ProductDefinitionWithAssociatedDocuments => 5,
            AutoDesignGeneralOrgItem::Representation => 6,
            AutoDesignGeneralOrgItem::ExternallyDefinedRepresentation => 7,
            AutoDesignGeneralOrgItem::AutoDesignDocumentReference => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignGeneralOrgItem::Product.case_num(), 1);
        assert_eq!(AutoDesignGeneralOrgItem::AutoDesignDocumentReference.case_num(), 8);
    }
}
