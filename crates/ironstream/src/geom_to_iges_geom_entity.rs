// FILE: geom_to_iges_geom_entity.rs
// occt: GeomToIGES_GeomEntity

/// Base class for transferring 3D geometric entities from Geom to IGES format.
///
/// Provides common interface for setting up model context and unit scaling
/// for geometric entity conversion.
pub struct GeomToIgesGeomEntity {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl GeomToIgesGeomEntity {
    /// Creates a new geometry entity converter with default settings.
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

    /// Sets the unit factor (in meters).
    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }

    /// Returns the unit factor in meters.
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
    fn test_create_new() {
        let entity = GeomToIgesGeomEntity::new();
        assert_eq!(entity.get_unit(), 1.0);
        assert_eq!(entity.get_model(), None);
    }

    #[test]
    fn test_set_model() {
        let mut entity = GeomToIgesGeomEntity::new();
        entity.set_model("test_model".to_string());
        assert_eq!(entity.get_model(), Some("test_model"));
    }

    #[test]
    fn test_set_unit() {
        let mut entity = GeomToIgesGeomEntity::new();
        entity.set_unit(0.001);
        assert_eq!(entity.get_unit(), 0.001);
    }

    #[test]
    fn test_from_entity() {
        let mut entity1 = GeomToIgesGeomEntity::new();
        entity1.set_model("model1".to_string());
        entity1.set_unit(2.5);

        let entity2 = GeomToIgesGeomEntity::from_entity(&entity1);
        assert_eq!(entity2.get_model(), Some("model1"));
        assert_eq!(entity2.get_unit(), 2.5);
    }

    #[test]
    fn test_default() {
        let entity = GeomToIgesGeomEntity::default();
        assert_eq!(entity.get_unit(), 1.0);
        assert_eq!(entity.get_model(), None);
    }
}
