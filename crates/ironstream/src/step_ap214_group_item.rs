// FILE: step_ap214_group_item.rs
// occt: StepAP214_GroupItem

/// Representation of STEP AP214 GroupItem SelectType.
#[derive(Clone, Debug)]
pub enum GroupItem {
    GeometricRepresentationItem,
    GroupRelationship,
    MappedItem,
    ProductDefinition,
    ProductDefinitionFormation,
    PropertyDefinitionRepresentation,
    Representation,
    RepresentationItem,
    RepresentationRelationshipWithTransformation,
    ShapeAspect,
    ShapeAspectRelationship,
    ShapeRepresentationRelationship,
    StyledItem,
    TopologicalRepresentationItem,
}

impl GroupItem {
    pub fn case_num(&self) -> i32 {
        match self {
            GroupItem::GeometricRepresentationItem => 1,
            GroupItem::GroupRelationship => 2,
            GroupItem::MappedItem => 3,
            GroupItem::ProductDefinition => 4,
            GroupItem::ProductDefinitionFormation => 5,
            GroupItem::PropertyDefinitionRepresentation => 6,
            GroupItem::Representation => 7,
            GroupItem::RepresentationItem => 8,
            GroupItem::RepresentationRelationshipWithTransformation => 9,
            GroupItem::ShapeAspect => 10,
            GroupItem::ShapeAspectRelationship => 11,
            GroupItem::ShapeRepresentationRelationship => 12,
            GroupItem::StyledItem => 13,
            GroupItem::TopologicalRepresentationItem => 14,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(GroupItem::GeometricRepresentationItem.case_num(), 1);
        assert_eq!(GroupItem::TopologicalRepresentationItem.case_num(), 14);
    }
}
