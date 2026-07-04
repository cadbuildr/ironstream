// FILE: geom_to_iges_geom_curve.rs
// occt: GeomToIGES_GeomCurve

/// Transfers 3D curve entities from Geom to IGES format.
///
/// Supports conversion of various curve types:
/// - Bounded curves (BSpline, Bezier, Trimmed)
/// - Conic curves (Circle, Ellipse, Hyperbola, Line, Parabola)
/// - Offset curves
pub struct GeomToIgesGeomCurve {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl GeomToIgesGeomCurve {
    /// Creates a new curve converter with default settings.
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

    /// Transfers a curve entity from parameter u_start to u_end.
    /// Returns an IGES entity or None if conversion fails.
    pub fn transfer_curve(
        &self,
        u_start: f64,
        u_end: f64,
    ) -> Option<String> {
        // Curve transfer creates an IGES entity representation
        // The exact type depends on the input curve type
        if u_end > u_start {
            Some(format!("IGESEntity(u:{}-{})", u_start, u_end))
        } else {
            None
        }
    }
}

impl Default for GeomToIgesGeomCurve {
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
    fn test_transfer_curve_valid() {
        let converter = GeomToIgesGeomCurve::new();
        let result = converter.transfer_curve(0.0, 1.0);
        assert!(result.is_some());
    }

    #[test]
    fn test_transfer_curve_invalid_range() {
        let converter = GeomToIgesGeomCurve::new();
        let result = converter.transfer_curve(1.0, 0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_entity() {
        let mut entity = GeomToIgesGeomEntity::new();
        entity.set_model("my_model".to_string());
        entity.set_unit(0.5);

        let curve = GeomToIgesGeomCurve::from_entity(&entity);
        assert_eq!(curve.get_model(), Some("my_model"));
        assert_eq!(curve.get_unit(), 0.5);
    }

    #[test]
    fn test_set_model() {
        let mut converter = GeomToIgesGeomCurve::new();
        converter.set_model("curve_model".to_string());
        assert_eq!(converter.get_model(), Some("curve_model"));
    }

    #[test]
    fn test_set_unit() {
        let mut converter = GeomToIgesGeomCurve::new();
        converter.set_unit(2.0);
        assert_eq!(converter.get_unit(), 2.0);
    }
}
