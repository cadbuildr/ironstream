// FILE: iges_geom_direction_o.rs
// occt: IGESGeom_Direction

/// Represents an IGES Direction entity (Type 123, Form 0).
/// A direction entity is a non-zero vector in Euclidean 3-space
/// defined by three direction ratios (components) with respect to coordinate axes.
/// Constraint: x^2 + y^2 + z^2 > 0
#[derive(Clone, Copy, Debug)]
pub struct IgesGeomDirection {
    direction: [f64; 3],
}

impl IgesGeomDirection {
    /// Creates a new empty Direction entity.
    pub fn new() -> Self {
        IgesGeomDirection {
            direction: [0.0, 0.0, 0.0],
        }
    }

    /// Sets the direction ratios.
    ///
    /// # Arguments
    /// - `direction`: Array [x, y, z] representing the direction ratios
    pub fn init(&mut self, direction: [f64; 3]) {
        self.direction = direction;
    }

    /// Returns the direction as a vector [x, y, z].
    pub fn value(&self) -> [f64; 3] {
        self.direction
    }

    /// Returns the direction value after applying transformation matrix.
    /// For now, returns the same as value() (transformation would be applied from parent entity).
    pub fn transformed_value(&self) -> [f64; 3] {
        self.direction
    }

    /// Returns the magnitude (norm) of the direction vector.
    pub fn magnitude(&self) -> f64 {
        (self.direction[0] * self.direction[0]
            + self.direction[1] * self.direction[1]
            + self.direction[2] * self.direction[2])
        .sqrt()
    }

    /// Validates that the direction is non-zero.
    pub fn is_valid(&self) -> bool {
        self.magnitude() > 0.0
    }
}

impl Default for IgesGeomDirection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_creation() {
        let dir = IgesGeomDirection::new();
        assert_eq!(dir.value(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_direction_init() {
        let mut dir = IgesGeomDirection::new();
        dir.init([1.0, 0.0, 0.0]);
        assert_eq!(dir.value(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_direction_magnitude() {
        let mut dir = IgesGeomDirection::new();
        dir.init([3.0, 4.0, 0.0]);
        assert_eq!(dir.magnitude(), 5.0);
    }

    #[test]
    fn test_direction_validity() {
        let mut dir = IgesGeomDirection::new();
        assert!(!dir.is_valid());
        dir.init([1.0, 2.0, 3.0]);
        assert!(dir.is_valid());
    }

    #[test]
    fn test_direction_transformed_value() {
        let mut dir = IgesGeomDirection::new();
        dir.init([2.0, 3.0, 4.0]);
        assert_eq!(dir.transformed_value(), [2.0, 3.0, 4.0]);
    }
}
