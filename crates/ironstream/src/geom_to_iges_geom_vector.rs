// FILE: geom_to_iges_geom_vector.rs
// occt: GeomToIGES_GeomVector

/// Transfers 3D vector entities from Geom to IGES format.
///
/// Converts various vector types (Vector, Direction, VectorWithMagnitude)
/// to IGES Direction entities with proper normalization.
pub struct GeomToIgesGeomVector {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl GeomToIgesGeomVector {
    /// Creates a new vector converter with default settings.
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

    /// Transfers a 3D vector to IGES Direction format.
    /// Returns (X, Y, Z) direction cosines as-is.
    pub fn transfer_vector(&self, x: f64, y: f64, z: f64) -> Option<(f64, f64, f64)> {
        Some((x, y, z))
    }

    /// Transfers a 3D VectorWithMagnitude to IGES Direction format.
    /// Normalizes by magnitude to get direction cosines.
    /// Returns None if magnitude is zero.
    pub fn transfer_vector_with_magnitude(&self, x: f64, y: f64, z: f64) -> Option<(f64, f64, f64)> {
        let magnitude = (x * x + y * y + z * z).sqrt();
        if magnitude > 1e-15 {
            Some((x / magnitude, y / magnitude, z / magnitude))
        } else {
            None
        }
    }

    /// Transfers a 3D Direction to IGES Direction format.
    /// Directions are already normalized.
    pub fn transfer_direction(&self, x: f64, y: f64, z: f64) -> Option<(f64, f64, f64)> {
        Some((x, y, z))
    }
}

impl Default for GeomToIgesGeomVector {
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
    fn test_transfer_vector() {
        let converter = GeomToIgesGeomVector::new();
        let result = converter.transfer_vector(1.0, 2.0, 3.0);
        assert_eq!(result, Some((1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_transfer_vector_with_magnitude() {
        let converter = GeomToIgesGeomVector::new();
        let result = converter.transfer_vector_with_magnitude(3.0, 4.0, 0.0);
        assert_eq!(result, Some((0.6, 0.8, 0.0)));
    }

    #[test]
    fn test_transfer_vector_with_magnitude_zero() {
        let converter = GeomToIgesGeomVector::new();
        let result = converter.transfer_vector_with_magnitude(0.0, 0.0, 0.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_transfer_direction() {
        let converter = GeomToIgesGeomVector::new();
        let result = converter.transfer_direction(1.0, 0.0, 0.0);
        assert_eq!(result, Some((1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_from_entity() {
        let mut entity = GeomToIgesGeomEntity::new();
        entity.set_model("vector_model".to_string());
        entity.set_unit(3.0);

        let converter = GeomToIgesGeomVector::from_entity(&entity);
        assert_eq!(converter.get_model(), Some("vector_model"));
        assert_eq!(converter.get_unit(), 3.0);
    }
}
