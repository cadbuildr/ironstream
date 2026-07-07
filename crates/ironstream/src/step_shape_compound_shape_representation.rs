// FILE: step_shape_compound_shape_representation.rs
// occt: StepShape_CompoundShapeRepresentation

//! Representation of STEP entity CompoundShapeRepresentation
//!
//! This is a simple marker class derived from ShapeRepresentation with no additional fields.
#[derive(Clone, Debug)]
pub struct CompoundShapeRepresentation {
    // Inherits from ShapeRepresentation (base class marker)
}

impl CompoundShapeRepresentation {
    /// Empty constructor
    pub fn new() -> Self {
        CompoundShapeRepresentation {}
    }
}

impl Default for CompoundShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = CompoundShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = CompoundShapeRepresentation::default();
        let _ = repr;
    }
}
