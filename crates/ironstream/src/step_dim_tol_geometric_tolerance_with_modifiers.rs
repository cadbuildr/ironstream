// FILE: step_dim_tol_geometric_tolerance_with_modifiers.rs
// occt: StepDimTol_GeometricToleranceWithModifiers

pub struct GeometricToleranceWithModifiers {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub modifiers: Vec<GeometricToleranceModifier>,
}

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

impl GeometricToleranceWithModifiers {
    pub fn new() -> Self {
        GeometricToleranceWithModifiers {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            modifiers: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn add_modifier(&mut self, modifier: GeometricToleranceModifier) {
        self.modifiers.push(modifier);
    }

    pub fn get_modifiers(&self) -> &[GeometricToleranceModifier] {
        &self.modifiers
    }

    pub fn set_magnitude(&mut self, magnitude: String) {
        self.magnitude = Some(magnitude);
    }

    pub fn get_magnitude(&self) -> Option<&str> {
        self.magnitude.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeometricToleranceWithModifiers::new();
        assert!(tol.modifiers.is_empty());
    }

    #[test]
    fn test_add_modifier() {
        let mut tol = GeometricToleranceWithModifiers::new();
        tol.add_modifier(GeometricToleranceModifier::FreeState);
        assert_eq!(tol.get_modifiers().len(), 1);
    }

    #[test]
    fn test_multiple_modifiers() {
        let mut tol = GeometricToleranceWithModifiers::new();
        tol.add_modifier(GeometricToleranceModifier::FreeState);
        tol.add_modifier(GeometricToleranceModifier::MaximumMaterialRequirement);
        tol.add_modifier(GeometricToleranceModifier::TangentPlane);
        assert_eq!(tol.get_modifiers().len(), 3);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tol = GeometricToleranceWithModifiers::new();
        tol.set_name("tol_with_mod".to_string());
        assert_eq!(tol.get_name(), Some("tol_with_mod"));
    }
}
