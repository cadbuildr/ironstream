// FILE: step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_geo_tol_wth_mod.rs
// occt: StepDimTol_GeoTolAndGeoTolWthDatRefAndGeoTolWthMod

pub struct GeoTolAndGeoTolWthDatRefAndGeoTolWthMod {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub datum_system: Vec<String>,
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

impl GeoTolAndGeoTolWthDatRefAndGeoTolWthMod {
    pub fn new() -> Self {
        GeoTolAndGeoTolWthDatRefAndGeoTolWthMod {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            datum_system: Vec::new(),
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
        let tol = GeoTolAndGeoTolWthDatRefAndGeoTolWthMod::new();
        assert!(tol.name.is_none());
        assert_eq!(tol.modifiers.len(), 0);
    }

    #[test]
    fn test_add_modifier() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndGeoTolWthMod::new();
        tol.add_modifier(GeometricToleranceModifier::FreeState);
        tol.add_modifier(GeometricToleranceModifier::MaximumMaterialRequirement);
        assert_eq!(tol.get_modifiers().len(), 2);
    }

    #[test]
    fn test_tolerance_type() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndGeoTolWthMod::new();
        tol.set_tolerance_type(GeometricToleranceType::PerpendicularityTolerance);
        assert_eq!(
            tol.get_tolerance_type(),
            GeometricToleranceType::PerpendicularityTolerance
        );
    }
}
