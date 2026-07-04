// FILE: step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref.rs
// occt: StepDimTol_GeoTolAndGeoTolWthDatRef

pub struct GeoTolAndGeoTolWthDatRef {
    pub geometric_tolerance_with_datum_reference: Option<Box<GeometricToleranceWithDatumReference>>,
    pub geometric_tolerance_type: GeometricToleranceType,
}

pub struct GeometricToleranceWithDatumReference {
    pub name: Option<String>,
    pub description: Option<String>,
    pub magnitude: Option<String>,
    pub toleranced_shape_aspect: Option<String>,
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

impl GeoTolAndGeoTolWthDatRef {
    pub fn new() -> Self {
        GeoTolAndGeoTolWthDatRef {
            geometric_tolerance_with_datum_reference: None,
            geometric_tolerance_type: GeometricToleranceType::AngularityTolerance,
        }
    }

    pub fn set_geometric_tolerance_with_datum_reference(
        &mut self,
        gtwd: GeometricToleranceWithDatumReference,
    ) {
        self.geometric_tolerance_with_datum_reference = Some(Box::new(gtwd));
    }

    pub fn get_geometric_tolerance_with_datum_reference(
        &self,
    ) -> Option<&GeometricToleranceWithDatumReference> {
        self.geometric_tolerance_with_datum_reference.as_ref().map(|b| b.as_ref())
    }

    pub fn set_geometric_tolerance_type(&mut self, tol_type: GeometricToleranceType) {
        self.geometric_tolerance_type = tol_type;
    }

    pub fn get_geometric_tolerance_type(&self) -> GeometricToleranceType {
        self.geometric_tolerance_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let geo_tol = GeoTolAndGeoTolWthDatRef::new();
        assert_eq!(
            geo_tol.get_geometric_tolerance_type(),
            GeometricToleranceType::AngularityTolerance
        );
        assert!(geo_tol.get_geometric_tolerance_with_datum_reference().is_none());
    }

    #[test]
    fn test_set_tolerance_type() {
        let mut geo_tol = GeoTolAndGeoTolWthDatRef::new();
        geo_tol.set_geometric_tolerance_type(GeometricToleranceType::ParallelismTolerance);
        assert_eq!(
            geo_tol.get_geometric_tolerance_type(),
            GeometricToleranceType::ParallelismTolerance
        );
    }

    #[test]
    fn test_set_and_get_datum_reference() {
        let mut geo_tol = GeoTolAndGeoTolWthDatRef::new();
        let gtwd = GeometricToleranceWithDatumReference {
            name: Some("test".to_string()),
            description: Some("test description".to_string()),
            magnitude: None,
            toleranced_shape_aspect: None,
        };
        geo_tol.set_geometric_tolerance_with_datum_reference(gtwd);
        assert!(geo_tol.get_geometric_tolerance_with_datum_reference().is_some());
    }
}
