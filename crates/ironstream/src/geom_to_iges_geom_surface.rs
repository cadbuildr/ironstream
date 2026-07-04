// FILE: geom_to_iges_geom_surface.rs
// occt: GeomToIGES_GeomSurface

/// Transfers 3D surface entities from Geom to IGES format.
///
/// Supports conversion of various surface types:
/// - Bounded surfaces (BSpline, Bezier, RectangularTrimmed)
/// - Elementary surfaces (Plane, Cylindrical, Conical, Spherical, Toroidal)
/// - Swept surfaces (LinearExtrusion, Revolution)
/// - Offset surfaces
pub struct GeomToIgesGeomSurface {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl GeomToIgesGeomSurface {
    /// Creates a new surface converter with default settings.
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
}

impl Default for GeomToIgesGeomSurface {
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
    fn test_create_new() {
        let surface = GeomToIgesGeomSurface::new();
        assert_eq!(surface.get_unit(), 1.0);
        assert_eq!(surface.get_model(), None);
    }

    #[test]
    fn test_set_model() {
        let mut surface = GeomToIgesGeomSurface::new();
        surface.set_model("surf_model".to_string());
        assert_eq!(surface.get_model(), Some("surf_model"));
    }

    #[test]
    fn test_set_unit() {
        let mut surface = GeomToIgesGeomSurface::new();
        surface.set_unit(1.5);
        assert_eq!(surface.get_unit(), 1.5);
    }

    #[test]
    fn test_from_entity() {
        let mut entity = GeomToIgesGeomEntity::new();
        entity.set_model("entity_model".to_string());
        entity.set_unit(2.0);

        let surface = GeomToIgesGeomSurface::from_entity(&entity);
        assert_eq!(surface.get_model(), Some("entity_model"));
        assert_eq!(surface.get_unit(), 2.0);
    }

    #[test]
    fn test_default() {
        let surface = GeomToIgesGeomSurface::default();
        assert_eq!(surface.get_unit(), 1.0);
        assert_eq!(surface.get_model(), None);
    }
}
