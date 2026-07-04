// FILE: step_shape_non_manifold_surface_shape_representation.rs
// occt: StepShape_NonManifoldSurfaceShapeRepresentation

//! Representation of STEP entity NonManifoldSurfaceShapeRepresentation

#[derive(Clone, Debug)]
pub struct NonManifoldSurfaceShapeRepresentation {}

impl NonManifoldSurfaceShapeRepresentation {
    /// Empty constructor
    pub fn new() -> Self {
        NonManifoldSurfaceShapeRepresentation {}
    }
}

impl Default for NonManifoldSurfaceShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = NonManifoldSurfaceShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = NonManifoldSurfaceShapeRepresentation::default();
        let _ = repr;
    }
}
