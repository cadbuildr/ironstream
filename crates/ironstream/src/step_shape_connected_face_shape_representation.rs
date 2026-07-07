// FILE: step_shape_connected_face_shape_representation.rs
// occt: StepShape_ConnectedFaceShapeRepresentation

//! Representation of STEP entity ConnectedFaceShapeRepresentation

#[derive(Clone, Debug)]
pub struct ConnectedFaceShapeRepresentation {}

impl ConnectedFaceShapeRepresentation {
    /// Empty constructor
    pub fn new() -> Self {
        ConnectedFaceShapeRepresentation {}
    }
}

impl Default for ConnectedFaceShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = ConnectedFaceShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = ConnectedFaceShapeRepresentation::default();
        let _ = repr;
    }
}
