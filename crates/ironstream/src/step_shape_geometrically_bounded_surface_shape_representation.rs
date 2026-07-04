// FILE: step_shape_geometrically_bounded_surface_shape_representation.rs
// occt: StepShape_GeometricallyBoundedSurfaceShapeRepresentation

//! Representation of STEP entity GeometricallyBoundedSurfaceShapeRepresentation

#[derive(Clone, Debug)]
pub struct GeometricallyBoundedSurfaceShapeRepresentation {}

impl GeometricallyBoundedSurfaceShapeRepresentation {
    /// Returns a GeometricallyBoundedSurfaceShapeRepresentation
    pub fn new() -> Self {
        GeometricallyBoundedSurfaceShapeRepresentation {}
    }
}

impl Default for GeometricallyBoundedSurfaceShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = GeometricallyBoundedSurfaceShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = GeometricallyBoundedSurfaceShapeRepresentation::default();
        let _ = repr;
    }
}
