// FILE: step_ap214_auto_design_grouped_item.rs
// occt: StepAP214_AutoDesignGroupedItem

/// Representation of STEP AP214 AutoDesignGroupedItem SelectType.
#[derive(Clone, Debug)]
pub enum AutoDesignGroupedItem {
    AdvancedBrepShapeRepresentation,
    CsgShapeRepresentation,
    FacetedBrepShapeRepresentation,
    GeometricallyBoundedSurfaceShapeRepresentation,
    GeometricallyBoundedWireframeShapeRepresentation,
    ManifoldSurfaceShapeRepresentation,
    Representation,
    RepresentationItem,
    ShapeAspect,
    ShapeRepresentation,
    TemplateInstance,
}

impl AutoDesignGroupedItem {
    pub fn case_num(&self) -> i32 {
        match self {
            AutoDesignGroupedItem::AdvancedBrepShapeRepresentation => 1,
            AutoDesignGroupedItem::CsgShapeRepresentation => 2,
            AutoDesignGroupedItem::FacetedBrepShapeRepresentation => 3,
            AutoDesignGroupedItem::GeometricallyBoundedSurfaceShapeRepresentation => 4,
            AutoDesignGroupedItem::GeometricallyBoundedWireframeShapeRepresentation => 5,
            AutoDesignGroupedItem::ManifoldSurfaceShapeRepresentation => 6,
            AutoDesignGroupedItem::Representation => 7,
            AutoDesignGroupedItem::RepresentationItem => 8,
            AutoDesignGroupedItem::ShapeAspect => 9,
            AutoDesignGroupedItem::ShapeRepresentation => 10,
            AutoDesignGroupedItem::TemplateInstance => 11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_nums() {
        assert_eq!(AutoDesignGroupedItem::AdvancedBrepShapeRepresentation.case_num(), 1);
        assert_eq!(AutoDesignGroupedItem::TemplateInstance.case_num(), 11);
    }
}
