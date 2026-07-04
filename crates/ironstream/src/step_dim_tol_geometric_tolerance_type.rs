// FILE: step_dim_tol_geometric_tolerance_type.rs
// occt: StepDimTol_GeometricToleranceType

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometricToleranceType {
    AngularityTolerance,
    CircularRunoutTolerance,
    CoaxialityTolerance,
    ConcentricityTolerance,
    CylindricityTolerance,
    FlatnessTolerance,
    LineProfileTolerance,
    ParallelismTolerance,
    PerpendicularityTolerance,
    PositionTolerance,
    RoundnessTolerance,
    StraightnessTolerance,
    SurfaceProfileTolerance,
    SymmetryTolerance,
    TotalRunoutTolerance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_all_variants() {
        let types = [
            GeometricToleranceType::AngularityTolerance,
            GeometricToleranceType::CircularRunoutTolerance,
            GeometricToleranceType::CoaxialityTolerance,
            GeometricToleranceType::ConcentricityTolerance,
            GeometricToleranceType::CylindricityTolerance,
            GeometricToleranceType::FlatnessTolerance,
            GeometricToleranceType::LineProfileTolerance,
            GeometricToleranceType::ParallelismTolerance,
            GeometricToleranceType::PerpendicularityTolerance,
            GeometricToleranceType::PositionTolerance,
            GeometricToleranceType::RoundnessTolerance,
            GeometricToleranceType::StraightnessTolerance,
            GeometricToleranceType::SurfaceProfileTolerance,
            GeometricToleranceType::SymmetryTolerance,
            GeometricToleranceType::TotalRunoutTolerance,
        ];
        assert_eq!(types.len(), 15);
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(
            GeometricToleranceType::AngularityTolerance,
            GeometricToleranceType::AngularityTolerance
        );
        assert_ne!(
            GeometricToleranceType::AngularityTolerance,
            GeometricToleranceType::PositionTolerance
        );
    }

    #[test]
    fn test_enum_copy() {
        let t1 = GeometricToleranceType::PositionTolerance;
        let t2 = t1;
        assert_eq!(t1, t2);
    }
}
