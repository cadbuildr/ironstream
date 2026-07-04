// FILE: step_dim_tol_modified_geometric_tolerance.rs
// occt: StepDimTol_ModifiedGeometricTolerance

pub struct ModifiedGeometricTolerance {
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

impl ModifiedGeometricTolerance {
    pub fn new() -> Self {
        ModifiedGeometricTolerance {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            modifiers: Vec::new(),
        }
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
        let tol = ModifiedGeometricTolerance::new();
        assert!(tol.modifiers.is_empty());
    }

    #[test]
    fn test_add_modifier() {
        let mut tol = ModifiedGeometricTolerance::new();
        tol.add_modifier(GeometricToleranceModifier::MaximumMaterialRequirement);
        assert_eq!(tol.get_modifiers().len(), 1);
    }

    #[test]
    fn test_set_magnitude() {
        let mut tol = ModifiedGeometricTolerance::new();
        tol.set_magnitude("2.0".to_string());
        assert_eq!(tol.get_magnitude(), Some("2.0"));
    }
}
