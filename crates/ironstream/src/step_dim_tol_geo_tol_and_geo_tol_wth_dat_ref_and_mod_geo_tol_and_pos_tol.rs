// FILE: step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_mod_geo_tol_and_pos_tol.rs
// occt: StepDimTol_GeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol

pub struct GeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol {
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

impl GeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol {
    pub fn new() -> Self {
        GeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            datum_system: Vec::new(),
            tolerance_type: GeometricToleranceType::PositionTolerance,
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
        let tol = GeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol::new();
        assert_eq!(
            tol.get_tolerance_type(),
            GeometricToleranceType::PositionTolerance
        );
    }

    #[test]
    fn test_add_modifiers() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol::new();
        tol.add_modifier(GeometricToleranceModifier::MaximumMaterialRequirement);
        assert_eq!(tol.get_modifiers().len(), 1);
    }
}
