// FILE: step_ap214_auto_design_presented_item_select.rs
// occt: StepAP214_AutoDesignPresentedItemSelect

/// Representation of STEP AP214 AutoDesignPresentedItemSelect SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignPresentedItemSelect {
    ProductDefinition,
    ProductDefinitionRelationship,
    ProductDefinitionShape,
    RepresentationRelationship,
    ShapeAspect,
    DocumentRelationship,
}

impl AutoDesignPresentedItemSelect {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignPresentedItemSelect::ProductDefinition => 1,
            AutoDesignPresentedItemSelect::ProductDefinitionRelationship => 2,
            AutoDesignPresentedItemSelect::ProductDefinitionShape => 3,
            AutoDesignPresentedItemSelect::RepresentationRelationship => 4,
            AutoDesignPresentedItemSelect::ShapeAspect => 5,
            AutoDesignPresentedItemSelect::DocumentRelationship => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignPresentedItemSelect::ProductDefinition.case_num(), 1);
        assert_eq!(AutoDesignPresentedItemSelect::DocumentRelationship.case_num(), 6);
    }
}
