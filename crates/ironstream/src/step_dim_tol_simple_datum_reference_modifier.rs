// FILE: step_dim_tol_simple_datum_reference_modifier.rs
// occt: StepDimTol_SimpleDatumReferenceModifier

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimpleDatumReferenceModifier {
    AnyCrossSection,
    AnyLongitudinalSection,
    Basic,
    ContactingFeature,
    DegreeOfFreedomConstraintU,
    DegreeOfFreedomConstraintV,
    DegreeOfFreedomConstraintW,
    DegreeOfFreedomConstraintX,
    DegreeOfFreedomConstraintY,
    DegreeOfFreedomConstraintZ,
    DistanceVariable,
    FreeState,
    LeastMaterialRequirement,
    Line,
    MajorDiameter,
    MaximumMaterialRequirement,
    MinorDiameter,
    Orientation,
    PitchDiameter,
    Plane,
    Point,
    Translation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        let v1 = SimpleDatumReferenceModifier::FreeState;
        let v2 = SimpleDatumReferenceModifier::MaximumMaterialRequirement;
        assert_ne!(v1, v2);
        assert_eq!(v1, SimpleDatumReferenceModifier::FreeState);
    }

    #[test]
    fn test_enum_copy() {
        let v1 = SimpleDatumReferenceModifier::Point;
        let v2 = v1;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_all_variants_exist() {
        let _ = SimpleDatumReferenceModifier::AnyCrossSection;
        let _ = SimpleDatumReferenceModifier::Translation;
    }
}
