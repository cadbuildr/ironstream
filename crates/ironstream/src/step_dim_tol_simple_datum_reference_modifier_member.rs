// FILE: step_dim_tol_simple_datum_reference_modifier_member.rs
// occt: StepDimTol_SimpleDatumReferenceModifierMember

pub struct SimpleDatumReferenceModifierMember {
    pub value: Option<SimpleDatumReferenceModifier>,
}

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

impl SimpleDatumReferenceModifierMember {
    pub fn new() -> Self {
        SimpleDatumReferenceModifierMember { value: None }
    }

    pub fn set_value(&mut self, val: SimpleDatumReferenceModifier) {
        self.value = Some(val);
    }

    pub fn get_value(&self) -> Option<SimpleDatumReferenceModifier> {
        self.value
    }

    pub fn name(&self) -> &'static str {
        "SIMPLE_DATUM_REFERENCE_MODIFIER"
    }

    pub fn kind(&self) -> i32 {
        4
    }

    pub fn enum_text(&self) -> Option<&'static str> {
        self.value.as_ref().map(|v| match v {
            SimpleDatumReferenceModifier::AnyCrossSection => "ANY_CROSS_SECTION",
            SimpleDatumReferenceModifier::AnyLongitudinalSection => "ANY_LONGITUDINAL_SECTION",
            SimpleDatumReferenceModifier::Basic => "BASIC",
            SimpleDatumReferenceModifier::ContactingFeature => "CONTACTING_FEATURE",
            SimpleDatumReferenceModifier::DegreeOfFreedomConstraintU => "DEGREE_OF_FREEDOM_CONSTRAINT_U",
            SimpleDatumReferenceModifier::DegreeOfFreedomConstraintV => "DEGREE_OF_FREEDOM_CONSTRAINT_V",
            SimpleDatumReferenceModifier::DegreeOfFreedomConstraintW => "DEGREE_OF_FREEDOM_CONSTRAINT_W",
            SimpleDatumReferenceModifier::DegreeOfFreedomConstraintX => "DEGREE_OF_FREEDOM_CONSTRAINT_X",
            SimpleDatumReferenceModifier::DegreeOfFreedomConstraintY => "DEGREE_OF_FREEDOM_CONSTRAINT_Y",
            SimpleDatumReferenceModifier::DegreeOfFreedomConstraintZ => "DEGREE_OF_FREEDOM_CONSTRAINT_Z",
            SimpleDatumReferenceModifier::DistanceVariable => "DISTANCE_VARIABLE",
            SimpleDatumReferenceModifier::FreeState => "FREE_STATE",
            SimpleDatumReferenceModifier::LeastMaterialRequirement => "LEAST_MATERIAL_REQUIREMENT",
            SimpleDatumReferenceModifier::Line => "LINE",
            SimpleDatumReferenceModifier::MajorDiameter => "MAJOR_DIAMETER",
            SimpleDatumReferenceModifier::MaximumMaterialRequirement => "MAXIMUM_MATERIAL_REQUIREMENT",
            SimpleDatumReferenceModifier::MinorDiameter => "MINOR_DIAMETER",
            SimpleDatumReferenceModifier::Orientation => "ORIENTATION",
            SimpleDatumReferenceModifier::PitchDiameter => "PITCH_DIAMETER",
            SimpleDatumReferenceModifier::Plane => "PLANE",
            SimpleDatumReferenceModifier::Point => "POINT",
            SimpleDatumReferenceModifier::Translation => "TRANSLATION",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let member = SimpleDatumReferenceModifierMember::new();
        assert!(member.value.is_none());
    }

    #[test]
    fn test_set_and_get_value() {
        let mut member = SimpleDatumReferenceModifierMember::new();
        member.set_value(SimpleDatumReferenceModifier::FreeState);
        assert_eq!(
            member.get_value(),
            Some(SimpleDatumReferenceModifier::FreeState)
        );
    }

    #[test]
    fn test_name() {
        let member = SimpleDatumReferenceModifierMember::new();
        assert_eq!(member.name(), "SIMPLE_DATUM_REFERENCE_MODIFIER");
    }

    #[test]
    fn test_kind() {
        let member = SimpleDatumReferenceModifierMember::new();
        assert_eq!(member.kind(), 4);
    }

    #[test]
    fn test_enum_text() {
        let mut member = SimpleDatumReferenceModifierMember::new();
        member.set_value(SimpleDatumReferenceModifier::Point);
        assert_eq!(member.enum_text(), Some("POINT"));
    }
}
