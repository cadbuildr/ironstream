// FILE: geom_to_iges_geom_point.rs
// occt: GeomToIGES_GeomPoint

/// Transfers 3D point entities from Geom to IGES format.
///
/// Converts cartesian points to IGES Point entities with
/// proper coordinate transformation and unit scaling.
pub struct GeomToIgesGeomPoint {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl GeomToIgesGeomPoint {
    /// Creates a new point converter with default settings.
    pub fn new() -> Self {
        Self {
            model_handle: None,
            unit_factor: 1.0,
        }
    }

    /// Copies configuration from another geometry entity.
    pub fn from_entity(entity: &GeomToIgesGeomEntity) -> Self {
        Self {
            model_handle: entity.get_model().map(|s| s.to_string()),
            unit_factor: entity.get_unit(),
        }
    }

    /// Sets the IGES model handle.
    pub fn set_model(&mut self, model: String) {
        self.model_handle = Some(model);
    }

    /// Returns the current model handle.
    pub fn get_model(&self) -> Option<&str> {
        self.model_handle.as_deref()
    }

    /// Sets the unit factor.
    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }

    /// Returns the unit factor.
    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }

    /// Transfers a 3D point with unit scaling applied.
    pub fn transfer_point(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        (x * self.unit_factor, y * self.unit_factor, z * self.unit_factor)
    }
}

impl Default for GeomToIgesGeomPoint {
    fn default() -> Self {
        Self::new()
    }
}

/// Parent class for geometric entity conversion.
pub struct GeomToIgesGeomEntity {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl GeomToIgesGeomEntity {
    /// Creates a new entity converter.
    pub fn new() -> Self {
        Self {
            model_handle: None,
            unit_factor: 1.0,
        }
    }

    /// Copies configuration from another entity.
    pub fn from_entity(entity: &GeomToIgesGeomEntity) -> Self {
        Self {
            model_handle: entity.model_handle.clone(),
            unit_factor: entity.unit_factor,
        }
    }

    /// Sets the IGES model handle.
    pub fn set_model(&mut self, model: String) {
        self.model_handle = Some(model);
    }

    /// Returns the current model handle.
    pub fn get_model(&self) -> Option<&str> {
        self.model_handle.as_deref()
    }

    /// Sets the unit factor.
    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }

    /// Returns the unit factor.
    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }
}

impl Default for GeomToIgesGeomEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_point_default_unit() {
        let converter = GeomToIgesGeomPoint::new();
        let result = converter.transfer_point(1.0, 2.0, 3.0);
        assert_eq!(result, (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_transfer_point_scaled() {
        let mut converter = GeomToIgesGeomPoint::new();
        converter.set_unit(2.0);
        let result = converter.transfer_point(1.0, 2.0, 3.0);
        assert_eq!(result, (2.0, 4.0, 6.0));
    }

    #[test]
    fn test_from_entity() {
        let mut entity = GeomToIgesGeomEntity::new();
        entity.set_model("point_model".to_string());
        entity.set_unit(0.5);

        let converter = GeomToIgesGeomPoint::from_entity(&entity);
        assert_eq!(converter.get_model(), Some("point_model"));
        assert_eq!(converter.get_unit(), 0.5);
    }

    #[test]
    fn test_set_model() {
        let mut converter = GeomToIgesGeomPoint::new();
        converter.set_model("model1".to_string());
        assert_eq!(converter.get_model(), Some("model1"));
    }

    #[test]
    fn test_set_unit() {
        let mut converter = GeomToIgesGeomPoint::new();
        converter.set_unit(3.0);
        assert_eq!(converter.get_unit(), 3.0);
    }
}
