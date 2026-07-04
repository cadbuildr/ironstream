// FILE: step_ap214_security_classification_item.rs
// occt: StepAP214_SecurityClassificationItem

/// Representation of STEP AP214 SecurityClassificationItem SelectType.
#[derive(Clone, Debug)]
pub enum SecurityClassificationItem {
    Action,
    AssemblyComponentUsage,
    AssemblyComponentUsageSubstitute,
    ConfigurationDesign,
    ConfigurationEffectivity,
    Document,
    DocumentFile,
    DraughtingModel,
    GeneralProperty,
    MakeFromUsageOption,
    MaterialDesignation,
    MechanicalDesignGeometricPresentationRepresentation,
    PresentationArea,
    Product,
    ProductConcept,
    ProductDefinition,
    ProductDefinitionFormation,
    ProductDefinitionRelationship,
    ProductDefinitionUsage,
    PropertyDefinition,
    ShapeRepresentation,
    VersionedActionRequest,
}

impl SecurityClassificationItem {
    pub fn case_num(&self) -> i32 {
        match self {
            SecurityClassificationItem::Action => 1,
            SecurityClassificationItem::AssemblyComponentUsage => 2,
            SecurityClassificationItem::AssemblyComponentUsageSubstitute => 3,
            SecurityClassificationItem::ConfigurationDesign => 4,
            SecurityClassificationItem::ConfigurationEffectivity => 5,
            SecurityClassificationItem::Document => 6,
            SecurityClassificationItem::DocumentFile => 7,
            SecurityClassificationItem::DraughtingModel => 8,
            SecurityClassificationItem::GeneralProperty => 9,
            SecurityClassificationItem::MakeFromUsageOption => 10,
            SecurityClassificationItem::MaterialDesignation => 11,
            SecurityClassificationItem::MechanicalDesignGeometricPresentationRepresentation => 12,
            SecurityClassificationItem::PresentationArea => 13,
            SecurityClassificationItem::Product => 14,
            SecurityClassificationItem::ProductConcept => 15,
            SecurityClassificationItem::ProductDefinition => 16,
            SecurityClassificationItem::ProductDefinitionFormation => 17,
            SecurityClassificationItem::ProductDefinitionRelationship => 18,
            SecurityClassificationItem::ProductDefinitionUsage => 19,
            SecurityClassificationItem::PropertyDefinition => 20,
            SecurityClassificationItem::ShapeRepresentation => 21,
            SecurityClassificationItem::VersionedActionRequest => 22,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(SecurityClassificationItem::Action.case_num(), 1);
        assert_eq!(SecurityClassificationItem::VersionedActionRequest.case_num(), 22);
    }
}
