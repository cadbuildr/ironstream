// FILE: step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_geo_tol_wth_max_tol.rs
// occt: StepDimTol_GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol

pub struct GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub datum_system: Vec<String>,
    pub tolerance_type: GeometricToleranceType,
    pub maximum_tolerance: Option<f64>,
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

impl GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol {
    pub fn new() -> Self {
        GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            datum_system: Vec::new(),
            tolerance_type: GeometricToleranceType::AngularityTolerance,
            maximum_tolerance: None,
        }
    }

    pub fn set_maximum_tolerance(&mut self, max_tol: f64) {
        self.maximum_tolerance = Some(max_tol);
    }

    pub fn get_maximum_tolerance(&self) -> Option<f64> {
        self.maximum_tolerance
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
        let tol = GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol::new();
        assert!(tol.name.is_none());
        assert!(tol.maximum_tolerance.is_none());
    }

    #[test]
    fn test_set_maximum_tolerance() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol::new();
        tol.set_maximum_tolerance(5.5);
        assert_eq!(tol.get_maximum_tolerance(), Some(5.5));
    }

    #[test]
    fn test_tolerance_type() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol::new();
        tol.set_tolerance_type(GeometricToleranceType::PositionTolerance);
        assert_eq!(tol.get_tolerance_type(), GeometricToleranceType::PositionTolerance);
    }
}
