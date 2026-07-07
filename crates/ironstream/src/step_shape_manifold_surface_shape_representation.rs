// FILE: step_shape_manifold_surface_shape_representation.rs
// occt: StepShape_ManifoldSurfaceShapeRepresentation

//! Representation of STEP entity ManifoldSurfaceShapeRepresentation

#[derive(Clone, Debug)]
pub struct ManifoldSurfaceShapeRepresentation {}

impl ManifoldSurfaceShapeRepresentation {
    /// Returns a ManifoldSurfaceShapeRepresentation
    pub fn new() -> Self {
        ManifoldSurfaceShapeRepresentation {}
    }
}

impl Default for ManifoldSurfaceShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = ManifoldSurfaceShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = ManifoldSurfaceShapeRepresentation::default();
        let _ = repr;
    }
}
