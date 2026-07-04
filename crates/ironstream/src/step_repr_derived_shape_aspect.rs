// FILE: step_repr_derived_shape_aspect.rs
// occt: StepRepr_DerivedShapeAspect

/// StepRepr_DerivedShapeAspect:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_ShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprDerivedShapeAspect {}

impl StepReprDerivedShapeAspect {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprDerivedShapeAspect {}
    }
}

impl Default for StepReprDerivedShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprDerivedShapeAspect::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
