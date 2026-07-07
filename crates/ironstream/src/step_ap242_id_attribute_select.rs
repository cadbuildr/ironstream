// FILE: step_ap242_id_attribute_select.rs
// occt: StepAP242_IdAttributeSelect

/// Representation of STEP AP242 IdAttributeSelect SelectType.
#[derive(Clone, Debug)]
pub enum IdAttributeSelect {
    Action,
    Address,
    ApplicationContext,
    DimensionalSize,
    GeometricTolerance,
    Group,
    ProductCategory,
    PropertyDefinition,
    Representation,
    ShapeAspect,
    ShapeAspectRelationship,
}

impl IdAttributeSelect {
    pub fn case_num(&self) -> i32 {
        match self {
            IdAttributeSelect::Action => 1,
            IdAttributeSelect::Address => 2,
            IdAttributeSelect::ApplicationContext => 3,
            IdAttributeSelect::DimensionalSize => 4,
            IdAttributeSelect::GeometricTolerance => 5,
            IdAttributeSelect::Group => 6,
            IdAttributeSelect::ProductCategory => 8,
            IdAttributeSelect::PropertyDefinition => 9,
            IdAttributeSelect::Representation => 10,
            IdAttributeSelect::ShapeAspect => 11,
            IdAttributeSelect::ShapeAspectRelationship => 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(IdAttributeSelect::Action.case_num(), 1);
        assert_eq!(IdAttributeSelect::ShapeAspectRelationship.case_num(), 12);
    }
}
