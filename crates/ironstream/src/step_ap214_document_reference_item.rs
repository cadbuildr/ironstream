// FILE: step_ap214_document_reference_item.rs
// occt: StepAP214_DocumentReferenceItem

/// Representation of STEP AP214 DocumentReferenceItem SelectType.
#[derive(Clone, Debug)]
pub enum DocumentReferenceItem {
    Approval,
    DescriptiveRepresentationItem,
    MaterialDesignation,
    ProductDefinition,
    ProductDefinitionRelationship,
    PropertyDefinition,
    Representation,
    ShapeAspect,
    ShapeAspectRelationship,
    AppliedExternalIdentificationAssignment,
    AssemblyComponentUsage,
    CharacterizedObject,
    DimensionalSize,
    ExternallyDefinedItem,
    Group,
    GroupRelationship,
    MeasureRepresentationItem,
    ProductCategory,
    ProductDefinitionContext,
    RepresentationItem,
}

impl DocumentReferenceItem {
    pub fn case_num(&self) -> i32 {
        match self {
            DocumentReferenceItem::Approval => 1,
            DocumentReferenceItem::DescriptiveRepresentationItem => 2,
            DocumentReferenceItem::MaterialDesignation => 3,
            DocumentReferenceItem::ProductDefinition => 4,
            DocumentReferenceItem::ProductDefinitionRelationship => 5,
            DocumentReferenceItem::PropertyDefinition => 6,
            DocumentReferenceItem::Representation => 7,
            DocumentReferenceItem::ShapeAspect => 8,
            DocumentReferenceItem::ShapeAspectRelationship => 9,
            DocumentReferenceItem::AppliedExternalIdentificationAssignment => 10,
            DocumentReferenceItem::AssemblyComponentUsage => 11,
            DocumentReferenceItem::CharacterizedObject => 12,
            DocumentReferenceItem::DimensionalSize => 13,
            DocumentReferenceItem::ExternallyDefinedItem => 14,
            DocumentReferenceItem::Group => 15,
            DocumentReferenceItem::GroupRelationship => 16,
            DocumentReferenceItem::MeasureRepresentationItem => 17,
            DocumentReferenceItem::ProductCategory => 18,
            DocumentReferenceItem::ProductDefinitionContext => 19,
            DocumentReferenceItem::RepresentationItem => 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(DocumentReferenceItem::Approval.case_num(), 1);
        assert_eq!(DocumentReferenceItem::RepresentationItem.case_num(), 20);
    }
}
