// FILE: step_ap214_date_and_time_item.rs
// occt: StepAP214_DateAndTimeItem

/// Representation of STEP AP214 DateAndTimeItem SelectType.
#[derive(Clone, Debug)]
pub enum DateAndTimeItem {
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
    SecurityClassification,
}

impl DateAndTimeItem {
    pub fn case_num(&self) -> i32 {
        match self {
            DateAndTimeItem::ApprovalPersonOrganization => 1,
            DateAndTimeItem::AppliedPersonAndOrganizationAssignment => 2,
            DateAndTimeItem::AppliedOrganizationAssignment => 3,
            DateAndTimeItem::AssemblyComponentUsageSubstitute => 4,
            DateAndTimeItem::DocumentFile => 5,
            DateAndTimeItem::Effectivity => 6,
            DateAndTimeItem::MaterialDesignation => 7,
            DateAndTimeItem::MechanicalDesignGeometricPresentationRepresentation => 8,
            DateAndTimeItem::PresentationArea => 9,
            DateAndTimeItem::Product => 10,
            DateAndTimeItem::ProductDefinition => 11,
            DateAndTimeItem::ProductDefinitionFormation => 12,
            DateAndTimeItem::ProductDefinitionRelationship => 13,
            DateAndTimeItem::PropertyDefinition => 14,
            DateAndTimeItem::ShapeRepresentation => 15,
            DateAndTimeItem::SecurityClassification => 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(DateAndTimeItem::ApprovalPersonOrganization.case_num(), 1);
        assert_eq!(DateAndTimeItem::SecurityClassification.case_num(), 16);
    }
}
