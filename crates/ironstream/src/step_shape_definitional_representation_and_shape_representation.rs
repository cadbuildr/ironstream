// FILE: step_shape_definitional_representation_and_shape_representation.rs
// occt: StepShape_DefinitionalRepresentationAndShapeRepresentation

//! Implements complex type
//! (DEFINITIONAL_REPRESENTATION, REPRESENTATION, SHAPE_REPRESENTATION)

#[derive(Clone, Debug)]
pub struct DefinitionalRepresentationAndShapeRepresentation {}

impl DefinitionalRepresentationAndShapeRepresentation {
    /// Constructor
    pub fn new() -> Self {
        DefinitionalRepresentationAndShapeRepresentation {}
    }
}

impl Default for DefinitionalRepresentationAndShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = DefinitionalRepresentationAndShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = DefinitionalRepresentationAndShapeRepresentation::default();
        let _ = repr;
    }
}
