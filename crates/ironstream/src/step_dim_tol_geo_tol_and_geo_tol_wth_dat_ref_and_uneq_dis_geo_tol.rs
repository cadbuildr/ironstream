// FILE: step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_uneq_dis_geo_tol.rs
// occt: StepDimTol_GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol

pub struct GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
    pub datum_system: Vec<String>,
    pub tolerance_type: GeometricToleranceType,
    pub lower_displacement: Option<f64>,
    pub upper_displacement: Option<f64>,
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

impl GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol {
    pub fn new() -> Self {
        GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol {
            name: None,
            description: None,
            magnitude: None,
            toleranced_shape_aspect: None,
            datum_system: Vec::new(),
            tolerance_type: GeometricToleranceType::AngularityTolerance,
            lower_displacement: None,
            upper_displacement: None,
        }
    }

    pub fn set_lower_displacement(&mut self, val: f64) {
        self.lower_displacement = Some(val);
    }

    pub fn set_upper_displacement(&mut self, val: f64) {
        self.upper_displacement = Some(val);
    }

    pub fn get_lower_displacement(&self) -> Option<f64> {
        self.lower_displacement
    }

    pub fn get_upper_displacement(&self) -> Option<f64> {
        self.upper_displacement
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
        let tol = GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol::new();
        assert!(tol.lower_displacement.is_none());
        assert!(tol.upper_displacement.is_none());
    }

    #[test]
    fn test_set_displacements() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol::new();
        tol.set_lower_displacement(1.5);
        tol.set_upper_displacement(2.5);
        assert_eq!(tol.get_lower_displacement(), Some(1.5));
        assert_eq!(tol.get_upper_displacement(), Some(2.5));
    }

    #[test]
    fn test_tolerance_type() {
        let mut tol = GeoTolAndGeoTolWthDatRefAndUneqDisGeoTol::new();
        tol.set_tolerance_type(GeometricToleranceType::PositionTolerance);
        assert_eq!(
            tol.get_tolerance_type(),
            GeometricToleranceType::PositionTolerance
        );
    }
}
