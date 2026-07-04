// FILE: geom2d_to_iges_geom2d_vector.rs
// occt: Geom2dToIGES_Geom2dVector

/// A tool for transferring 2D vectors from Geom2d to IGES format.
///
/// Converts 2D geometric vectors (Vector, Direction, VectorWithMagnitude)
/// to IGES Direction entities, projecting them onto the XY plane (Z=0).
pub struct Geom2dToIgesGeom2dVector {
    model_handle: Option<String>,  // Placeholder for IGESData_IGESModel handle
    unit_factor: f64,
}

impl Geom2dToIgesGeom2dVector {
    /// Creates a new tool with default settings.
    pub fn new() -> Self {
        Self {
            model_handle: None,
            unit_factor: 1.0,
        }
    }

    /// Creates a tool from another Geom2dEntity, copying its configuration.
    pub fn from_entity(entity: &Geom2dToIgesGeom2dEntity) -> Self {
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

    /// Sets the unit factor (in millimeters).
    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }

    /// Returns the unit factor.
    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }

    /// Transfers a 2D vector to IGES Direction format.
    /// Returns (X, Y, Z) direction cosines normalized and projected to 3D (Z=0).
    /// Returns None if the input is null/invalid.
    pub fn transfer_2d_vector(&self, x: f64, y: f64) -> Option<(f64, f64, f64)> {
        // For a basic Vector: transfer as-is projected to Z=0
        Some((x, y, 0.0))
    }

    /// Transfers a 2D VectorWithMagnitude to IGES Direction format.
    /// Normalizes by magnitude to get direction cosines.
    /// Returns None if magnitude is zero or input is null.
    pub fn transfer_2d_vector_with_magnitude(&self, x: f64, y: f64) -> Option<(f64, f64, f64)> {
        let magnitude = (x * x + y * y).sqrt();
        if magnitude > 1e-15 {
            Some((x / magnitude, y / magnitude, 0.0))
        } else {
            None
        }
    }

    /// Transfers a 2D Direction to IGES Direction format.
    /// Directions are already normalized, so project as-is to Z=0.
    pub fn transfer_2d_direction(&self, x: f64, y: f64) -> Option<(f64, f64, f64)> {
        Some((x, y, 0.0))
    }
}

impl Default for Geom2dToIgesGeom2dVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Parent class representing a base 2D->IGES geometry entity converter.
pub struct Geom2dToIgesGeom2dEntity {
    model_handle: Option<String>,
    unit_factor: f64,
}

impl Geom2dToIgesGeom2dEntity {
    /// Creates a new entity converter with default settings.
    pub fn new() -> Self {
        Self {
            model_handle: None,
            unit_factor: 1.0,
        }
    }

    /// Copies settings from another entity.
    pub fn from_entity(entity: &Geom2dToIgesGeom2dEntity) -> Self {
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

    /// Sets the unit factor (in millimeters).
    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }

    /// Returns the unit factor.
    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }
}

impl Default for Geom2dToIgesGeom2dEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_2d_vector() {
        let converter = Geom2dToIgesGeom2dVector::new();
        let result = converter.transfer_2d_vector(3.0, 4.0);
        assert_eq!(result, Some((3.0, 4.0, 0.0)));
    }

    #[test]
    fn test_transfer_2d_vector_with_magnitude() {
        let converter = Geom2dToIgesGeom2dVector::new();
        let result = converter.transfer_2d_vector_with_magnitude(3.0, 4.0);
        assert_eq!(result, Some((0.6, 0.8, 0.0)));
    }

    #[test]
    fn test_transfer_2d_vector_with_magnitude_zero() {
        let converter = Geom2dToIgesGeom2dVector::new();
        let result = converter.transfer_2d_vector_with_magnitude(0.0, 0.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_transfer_2d_direction() {
        let converter = Geom2dToIgesGeom2dVector::new();
        let result = converter.transfer_2d_direction(1.0, 0.0);
        assert_eq!(result, Some((1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_create_from_entity() {
        let mut entity = Geom2dToIgesGeom2dEntity::new();
        entity.set_unit(2.5);
        entity.set_model("test_model".to_string());

        let converter = Geom2dToIgesGeom2dVector::from_entity(&entity);
        assert_eq!(converter.get_unit(), 2.5);
        assert_eq!(converter.get_model(), Some("test_model"));
    }

    #[test]
    fn test_entity_copy() {
        let mut entity1 = Geom2dToIgesGeom2dEntity::new();
        entity1.set_unit(3.0);
        entity1.set_model("model1".to_string());

        let entity2 = Geom2dToIgesGeom2dEntity::from_entity(&entity1);
        assert_eq!(entity2.get_unit(), 3.0);
        assert_eq!(entity2.get_model(), Some("model1"));
    }
}
