// FILE: step_shape_faceted_brep_shape_representation.rs
// occt: StepShape_FacetedBrepShapeRepresentation

//! Representation of STEP entity FacetedBrepShapeRepresentation

#[derive(Clone, Debug)]
pub struct FacetedBrepShapeRepresentation {}

impl FacetedBrepShapeRepresentation {
    /// Returns a FacetedBrepShapeRepresentation
    pub fn new() -> Self {
        FacetedBrepShapeRepresentation {}
    }
}

impl Default for FacetedBrepShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let repr = FacetedBrepShapeRepresentation::new();
        let _ = repr;
    }

    #[test]
    fn test_default() {
        let repr = FacetedBrepShapeRepresentation::default();
        let _ = repr;
    }
}
