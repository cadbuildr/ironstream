// FILE: step_ap214_date_item.rs
// occt: StepAP214_DateItem

/// Representation of STEP AP214 DateItem SelectType.
#[derive(Clone, Debug)]
pub enum DateItem {
    ApprovalPersonOrganization,
    AppliedPersonAndOrganizationAssignment,
    AppliedOrganizationAssignment,
    AssemblyComponentUsageSubstitute,
    DocumentFile,
    Effectivity,
    MaterialDesignation,
    MechanicalDesignGeometricPresentationRepresentation,
    PresentationArea,
    Product,
    ProductDefinition,
    ProductDefinitionFormation,
    ProductDefinitionRelationship,
    PropertyDefinition,
    ShapeRepresentation,
    AppliedSecurityClassificationAssignment,
    Document,
}

impl DateItem {
    pub fn case_num(&self) -> i32 {
        match self {
            DateItem::ApprovalPersonOrganization => 1,
            DateItem::AppliedPersonAndOrganizationAssignment => 2,
            DateItem::AppliedOrganizationAssignment => 3,
            DateItem::AssemblyComponentUsageSubstitute => 4,
            DateItem::DocumentFile => 5,
            DateItem::Effectivity => 6,
            DateItem::MaterialDesignation => 7,
            DateItem::MechanicalDesignGeometricPresentationRepresentation => 8,
            DateItem::PresentationArea => 9,
            DateItem::Product => 10,
            DateItem::ProductDefinition => 11,
            DateItem::ProductDefinitionFormation => 12,
            DateItem::ProductDefinitionRelationship => 13,
            DateItem::PropertyDefinition => 14,
            DateItem::ShapeRepresentation => 15,
            DateItem::AppliedSecurityClassificationAssignment => 16,
            DateItem::Document => 17,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(DateItem::ApprovalPersonOrganization.case_num(), 1);
        assert_eq!(DateItem::Document.case_num(), 17);
    }
}
