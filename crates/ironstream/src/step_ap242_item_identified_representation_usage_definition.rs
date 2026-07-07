// FILE: step_ap242_item_identified_representation_usage_definition.rs
// occt: StepAP242_ItemIdentifiedRepresentationUsageDefinition

/// Representation of STEP AP242 ItemIdentifiedRepresentationUsageDefinition SelectType.
#[derive(Clone, Debug)]
pub enum ItemIdentifiedRepresentationUsageDefinition {
    AppliedApprovalAssignment,
    AppliedDateAndTimeAssignment,
    AppliedDateAssignment,
    AppliedDocumentReference,
    AppliedExternalIdentificationAssignment,
    AppliedGroupAssignment,
    AppliedOrganizationAssignment,
    AppliedPersonAndOrganizationAssignment,
    AppliedSecurityClassificationAssignment,
    DimensionalSize,
    GeneralProperty,
    GeometricTolerance,
    ProductDefinitionRelationship,
    PropertyDefinition,
    PropertyDefinitionRelationship,
    ShapeAspect,
    ShapeAspectRelationship,
}

impl ItemIdentifiedRepresentationUsageDefinition {
    pub fn case_num(&self) -> i32 {
        match self {
            ItemIdentifiedRepresentationUsageDefinition::AppliedApprovalAssignment => 1,
            ItemIdentifiedRepresentationUsageDefinition::AppliedDateAndTimeAssignment => 2,
            ItemIdentifiedRepresentationUsageDefinition::AppliedDateAssignment => 3,
            ItemIdentifiedRepresentationUsageDefinition::AppliedDocumentReference => 4,
            ItemIdentifiedRepresentationUsageDefinition::AppliedExternalIdentificationAssignment => 5,
            ItemIdentifiedRepresentationUsageDefinition::AppliedGroupAssignment => 6,
            ItemIdentifiedRepresentationUsageDefinition::AppliedOrganizationAssignment => 7,
            ItemIdentifiedRepresentationUsageDefinition::AppliedPersonAndOrganizationAssignment => 8,
            ItemIdentifiedRepresentationUsageDefinition::AppliedSecurityClassificationAssignment => 9,
            ItemIdentifiedRepresentationUsageDefinition::DimensionalSize => 10,
            ItemIdentifiedRepresentationUsageDefinition::GeneralProperty => 11,
            ItemIdentifiedRepresentationUsageDefinition::GeometricTolerance => 12,
            ItemIdentifiedRepresentationUsageDefinition::ProductDefinitionRelationship => 13,
            ItemIdentifiedRepresentationUsageDefinition::PropertyDefinition => 14,
            ItemIdentifiedRepresentationUsageDefinition::PropertyDefinitionRelationship => 15,
            ItemIdentifiedRepresentationUsageDefinition::ShapeAspect => 16,
            ItemIdentifiedRepresentationUsageDefinition::ShapeAspectRelationship => 17,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(ItemIdentifiedRepresentationUsageDefinition::AppliedApprovalAssignment.case_num(), 1);
        assert_eq!(ItemIdentifiedRepresentationUsageDefinition::ShapeAspectRelationship.case_num(), 17);
    }
}
