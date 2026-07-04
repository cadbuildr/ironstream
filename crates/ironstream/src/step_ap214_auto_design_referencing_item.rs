// FILE: step_ap214_auto_design_referencing_item.rs
// occt: StepAP214_AutoDesignReferencingItem

/// Representation of STEP AP214 AutoDesignReferencingItem SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignReferencingItem {
    Approval,
    DocumentRelationship,
    ExternallyDefinedRepresentation,
    MappedItem,
    MaterialDesignation,
    PresentationArea,
    PresentationView,
    ProductCategory,
    ProductDefinition,
    ProductDefinitionRelationship,
    PropertyDefinition,
    Representation,
    RepresentationRelationship,
    ShapeAspect,
}

impl AutoDesignReferencingItem {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignReferencingItem::Approval => 1,
            AutoDesignReferencingItem::DocumentRelationship => 2,
            AutoDesignReferencingItem::ExternallyDefinedRepresentation => 3,
            AutoDesignReferencingItem::MappedItem => 4,
            AutoDesignReferencingItem::MaterialDesignation => 5,
            AutoDesignReferencingItem::PresentationArea => 6,
            AutoDesignReferencingItem::PresentationView => 7,
            AutoDesignReferencingItem::ProductCategory => 8,
            AutoDesignReferencingItem::ProductDefinition => 9,
            AutoDesignReferencingItem::ProductDefinitionRelationship => 10,
            AutoDesignReferencingItem::PropertyDefinition => 11,
            AutoDesignReferencingItem::Representation => 12,
            AutoDesignReferencingItem::RepresentationRelationship => 13,
            AutoDesignReferencingItem::ShapeAspect => 14,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignReferencingItem::Approval.case_num(), 1);
        assert_eq!(AutoDesignReferencingItem::ShapeAspect.case_num(), 14);
    }
}
