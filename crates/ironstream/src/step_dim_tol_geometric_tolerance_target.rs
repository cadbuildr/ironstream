// FILE: step_dim_tol_geometric_tolerance_target.rs
// occt: StepDimTol_GeometricToleranceTarget

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeometricToleranceTarget {
    DimensionalLocation,
    DimensionalSize,
    ProductDefinitionShape,
    ShapeAspect,
}

impl GeometricToleranceTarget {
    pub fn case_num(&self) -> i32 {
        match self {
            GeometricToleranceTarget::DimensionalLocation => 1,
            GeometricToleranceTarget::DimensionalSize => 2,
            GeometricToleranceTarget::ProductDefinitionShape => 3,
            GeometricToleranceTarget::ShapeAspect => 4,
        }
    }

    pub fn is_dimensional_location(&self) -> bool {
        matches!(self, GeometricToleranceTarget::DimensionalLocation)
    }

    pub fn is_dimensional_size(&self) -> bool {
        matches!(self, GeometricToleranceTarget::DimensionalSize)
    }

    pub fn is_product_definition_shape(&self) -> bool {
        matches!(self, GeometricToleranceTarget::ProductDefinitionShape)
    }

    pub fn is_shape_aspect(&self) -> bool {
        matches!(self, GeometricToleranceTarget::ShapeAspect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_dimensional_location() {
        let target = GeometricToleranceTarget::DimensionalLocation;
        assert_eq!(target.case_num(), 1);
    }

    #[test]
    fn test_case_num_dimensional_size() {
        let target = GeometricToleranceTarget::DimensionalSize;
        assert_eq!(target.case_num(), 2);
    }

    #[test]
    fn test_case_num_product_definition_shape() {
        let target = GeometricToleranceTarget::ProductDefinitionShape;
        assert_eq!(target.case_num(), 3);
    }

    #[test]
    fn test_case_num_shape_aspect() {
        let target = GeometricToleranceTarget::ShapeAspect;
        assert_eq!(target.case_num(), 4);
    }

    #[test]
    fn test_is_shape_aspect() {
        let target = GeometricToleranceTarget::ShapeAspect;
        assert!(target.is_shape_aspect());
        assert!(!target.is_dimensional_location());
    }

    #[test]
    fn test_is_dimensional_location() {
        let target = GeometricToleranceTarget::DimensionalLocation;
        assert!(target.is_dimensional_location());
        assert!(!target.is_shape_aspect());
    }
}
