// FILE: step_ap214_presented_item_select.rs
// occt: StepAP214_PresentedItemSelect

/// Representation of STEP AP214 PresentedItemSelect SelectType.
#[derive(Clone, Debug)]
pub enum PresentedItemSelect {
    ProductDefinition,
    ProductDefinitionRelationship,
}

impl PresentedItemSelect {
    pub fn case_num(&self) -> i32 {
        match self {
            PresentedItemSelect::ProductDefinition => 1,
            PresentedItemSelect::ProductDefinitionRelationship => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(PresentedItemSelect::ProductDefinition.case_num(), 1);
        assert_eq!(PresentedItemSelect::ProductDefinitionRelationship.case_num(), 2);
    }
}
