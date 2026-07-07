// FILE: step_shape_geometrically_bounded_wireframe_shape_representation.rs
// occt: StepShape_GeometricallyBoundedWireframeShapeRepresentation

//! Representation of STEP entity GeometricallyBoundedWireframeShapeRepresentation

#[derive(Clone, Debug)]
pub struct GeometricallyBoundedWireframeShapeRepresentation {}

impl GeometricallyBoundedWireframeShapeRepresentation {
    /// Returns a GeometricallyBoundedWireframeShapeRepresentation
    pub fn new() -> Self {
        GeometricallyBoundedWireframeShapeRepresentation {}
    }
}

impl Default for GeometricallyBoundedWireframeShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = GeometricallyBoundedWireframeShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = GeometricallyBoundedWireframeShapeRepresentation::default();
        let _ = repr;
    }
}
