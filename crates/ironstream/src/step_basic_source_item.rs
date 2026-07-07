// FILE: step_basic_source_item.rs
// occt: StepBasic_SourceItem

#[derive(Clone, Debug)]
pub enum StepBasicSourceItem {
    ProductDefinitionFormation(String),
    ProductDefinitionRelationship(String),
}

impl StepBasicSourceItem {
    pub fn case_num(&self) -> i32 {
        match self {
            Self::ProductDefinitionFormation(_) => 1,
            Self::ProductDefinitionRelationship(_) => 2,
        }
    }

    pub fn product_definition_formation(&self) -> Option<&str> {
        match self { Self::ProductDefinitionFormation(v) => Some(v), _ => None }
    }

    pub fn product_definition_relationship(&self) -> Option<&str> {
        match self { Self::ProductDefinitionRelationship(v) => Some(v), _ => None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        let p = StepBasicSourceItem::ProductDefinitionFormation("PDF-1".into());
        assert_eq!(p.case_num(), 1);

        let r = StepBasicSourceItem::ProductDefinitionRelationship("PDR-1".into());
        assert_eq!(r.case_num(), 2);
    }
}
