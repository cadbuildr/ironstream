// FILE: step_dim_tol_geo_tol_and_geo_tol_wth_mod.rs
// occt: StepDimTol_GeoTolAndGeoTolWthMod

pub struct GeoTolAndGeoTolWthMod {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub tolerance_type: GeometricToleranceType,
    pub modifiers: Vec<GeometricToleranceModifier>,
}

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

impl GeoTolAndGeoTolWthMod {
    pub fn new() -> Self {
        GeoTolAndGeoTolWthMod {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            tolerance_type: GeometricToleranceType::AngularityTolerance,
            modifiers: Vec::new(),
        }
    }

    pub fn add_modifier(&mut self, modifier: GeometricToleranceModifier) {
        self.modifiers.push(modifier);
    }

    pub fn get_modifiers(&self) -> &[GeometricToleranceModifier] {
        &self.modifiers
    }

    pub fn set_tolerance_type(&mut self, tol_type: GeometricToleranceType) {
        self.tolerance_type = tol_type;
    }

    pub fn get_tolerance_type(&self) -> GeometricToleranceType {
        self.tolerance_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tol = GeoTolAndGeoTolWthMod::new();
        assert_eq!(tol.modifiers.len(), 0);
    }

    #[test]
    fn test_add_modifiers() {
        let mut tol = GeoTolAndGeoTolWthMod::new();
        tol.add_modifier(GeometricToleranceModifier::FreeState);
        assert_eq!(tol.get_modifiers().len(), 1);
    }

    #[test]
    fn test_tolerance_type() {
        let mut tol = GeoTolAndGeoTolWthMod::new();
        tol.set_tolerance_type(GeometricToleranceType::CoaxialityTolerance);
        assert_eq!(
            tol.get_tolerance_type(),
            GeometricToleranceType::CoaxialityTolerance
        );
    }
}
