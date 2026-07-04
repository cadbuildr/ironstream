// FILE: step_ap214_person_and_organization_item.rs
// occt: StepAP214_PersonAndOrganizationItem

/// Representation of STEP AP214 PersonAndOrganizationItem SelectType.
#[derive(Clone, Debug)]
pub enum PersonAndOrganizationItem {
    AppliedOrganizationAssignment,
    AssemblyComponentUsageSubstitute,
    DocumentFile,
    MaterialDesignation,
    MechanicalDesignGeometricPresentationRepresentation,
    PresentationArea,
    Product,
    ProductDefinition,
    ProductDefinitionFormation,
    ProductDefinitionRelationship,
    PropertyDefinition,
    ShapeRepresentation,
    SecurityClassification,
    AppliedSecurityClassificationAssignment,
    Approval,
}

impl PersonAndOrganizationItem {
    pub fn case_num(&self) -> i32 {
        match self {
            PersonAndOrganizationItem::AppliedOrganizationAssignment => 1,
            PersonAndOrganizationItem::AssemblyComponentUsageSubstitute => 2,
            PersonAndOrganizationItem::DocumentFile => 3,
            PersonAndOrganizationItem::MaterialDesignation => 4,
            PersonAndOrganizationItem::MechanicalDesignGeometricPresentationRepresentation => 5,
            PersonAndOrganizationItem::PresentationArea => 6,
            PersonAndOrganizationItem::Product => 7,
            PersonAndOrganizationItem::ProductDefinition => 8,
            PersonAndOrganizationItem::ProductDefinitionFormation => 9,
            PersonAndOrganizationItem::ProductDefinitionRelationship => 10,
            PersonAndOrganizationItem::PropertyDefinition => 11,
            PersonAndOrganizationItem::ShapeRepresentation => 12,
            PersonAndOrganizationItem::SecurityClassification => 13,
            PersonAndOrganizationItem::AppliedSecurityClassificationAssignment => 14,
            PersonAndOrganizationItem::Approval => 15,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(PersonAndOrganizationItem::AppliedOrganizationAssignment.case_num(), 1);
        assert_eq!(PersonAndOrganizationItem::Approval.case_num(), 15);
    }
}
