// FILE: step_shape_edge_based_wireframe_shape_representation.rs
// occt: StepShape_EdgeBasedWireframeShapeRepresentation

//! Representation of STEP entity EdgeBasedWireframeShapeRepresentation

#[derive(Clone, Debug)]
pub struct EdgeBasedWireframeShapeRepresentation {}

impl EdgeBasedWireframeShapeRepresentation {
    /// Empty constructor
    pub fn new() -> Self {
        EdgeBasedWireframeShapeRepresentation {}
    }
}

impl Default for EdgeBasedWireframeShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = EdgeBasedWireframeShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = EdgeBasedWireframeShapeRepresentation::default();
        let _ = repr;
    }
}
