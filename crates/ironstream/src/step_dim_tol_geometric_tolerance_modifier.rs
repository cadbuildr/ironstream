// FILE: step_dim_tol_geometric_tolerance_modifier.rs
// occt: StepDimTol_GeometricToleranceModifier

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometricToleranceModifier {
    AnyCrossSection,
    CommonZone,
    EachRadialElement,
    FreeState,
    LeastMaterialRequirement,
    LineElement,
    MajorDiameter,
    MaximumMaterialRequirement,
    MinorDiameter,
    NotConvex,
    PitchDiameter,
    ReciprocityRequirement,
    SeparateRequirement,
    StatisticalTolerance,
    TangentPlane,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        let m1 = GeometricToleranceModifier::FreeState;
        let m2 = GeometricToleranceModifier::MaximumMaterialRequirement;
        assert_ne!(m1, m2);
        assert_eq!(m1, GeometricToleranceModifier::FreeState);
    }

    #[test]
    fn test_enum_copy() {
        let m1 = GeometricToleranceModifier::TangentPlane;
        let m2 = m1;
        assert_eq!(m1, m2);
    }
}
