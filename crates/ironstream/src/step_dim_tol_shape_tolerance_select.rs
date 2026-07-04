// FILE: step_dim_tol_shape_tolerance_select.rs
// occt: StepDimTol_ShapeToleranceSelect

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShapeToleranceSelect {
    GeometricTolerance,
    DimensionalTolerance,
}

impl ShapeToleranceSelect {
    pub fn case_num(&self) -> i32 {
        match self {
            ShapeToleranceSelect::GeometricTolerance => 1,
            ShapeToleranceSelect::DimensionalTolerance => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        let gt = ShapeToleranceSelect::GeometricTolerance;
        let dt = ShapeToleranceSelect::DimensionalTolerance;
        assert_eq!(gt.case_num(), 1);
        assert_eq!(dt.case_num(), 2);
    }

    #[test]
    fn test_enum_equality() {
        let gt1 = ShapeToleranceSelect::GeometricTolerance;
        let gt2 = ShapeToleranceSelect::GeometricTolerance;
        assert_eq!(gt1, gt2);
    }
}
