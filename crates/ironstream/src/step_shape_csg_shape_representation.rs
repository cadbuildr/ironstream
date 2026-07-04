// FILE: step_shape_csg_shape_representation.rs
// occt: StepShape_CsgShapeRepresentation

//! Representation of STEP entity CsgShapeRepresentation

#[derive(Clone, Debug)]
pub struct CsgShapeRepresentation {}

impl CsgShapeRepresentation {
    /// Returns a CsgShapeRepresentation
    pub fn new() -> Self {
        CsgShapeRepresentation {}
    }
}

impl Default for CsgShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = CsgShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = CsgShapeRepresentation::default();
        let _ = repr;
    }
}
