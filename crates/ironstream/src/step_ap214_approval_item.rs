// FILE: step_ap214_approval_item.rs
// occt: StepAP214_ApprovalItem

/// Representation of STEP AP214 ApprovalItem SelectType.
/// A select type that can represent various entities in a STEP file.
#[derive(Clone, Debug)]
pub enum ApprovalItem {
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
    ConfigurationItem,
    Date,
    Document,
    Effectivity,
    Group,
    GroupRelationship,
    ProductDefinitionFormationRelationship,
    Representation,
    ShapeAspectRelationship,
}

impl ApprovalItem {
    /// Returns a case number for the entity type (1-21 or 0 for unknown).
    pub fn case_num(&self) -> i32 {
        match self {
            ApprovalItem::AssemblyComponentUsageSubstitute => 1,
            ApprovalItem::DocumentFile => 2,
            ApprovalItem::MaterialDesignation => 3,
            ApprovalItem::MechanicalDesignGeometricPresentationRepresentation => 4,
            ApprovalItem::PresentationArea => 5,
            ApprovalItem::Product => 6,
            ApprovalItem::ProductDefinition => 7,
            ApprovalItem::ProductDefinitionFormation => 8,
            ApprovalItem::ProductDefinitionRelationship => 9,
            ApprovalItem::PropertyDefinition => 10,
            ApprovalItem::ShapeRepresentation => 11,
            ApprovalItem::SecurityClassification => 12,
            ApprovalItem::ConfigurationItem => 13,
            ApprovalItem::Date => 14,
            ApprovalItem::Document => 15,
            ApprovalItem::Effectivity => 16,
            ApprovalItem::Group => 17,
            ApprovalItem::GroupRelationship => 18,
            ApprovalItem::ProductDefinitionFormationRelationship => 19,
            ApprovalItem::Representation => 20,
            ApprovalItem::ShapeAspectRelationship => 21,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_numbers() {
        assert_eq!(ApprovalItem::AssemblyComponentUsageSubstitute.case_num(), 1);
        assert_eq!(ApprovalItem::DocumentFile.case_num(), 2);
        assert_eq!(ApprovalItem::Product.case_num(), 6);
        assert_eq!(ApprovalItem::ShapeAspectRelationship.case_num(), 21);
    }
}
