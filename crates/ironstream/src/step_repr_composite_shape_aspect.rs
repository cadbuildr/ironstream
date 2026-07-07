// FILE: step_repr_composite_shape_aspect.rs
// occt: StepRepr_CompositeShapeAspect

/// StepRepr_CompositeShapeAspect:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_ShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprCompositeShapeAspect {}

impl StepReprCompositeShapeAspect {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprCompositeShapeAspect {}
    }
}

impl Default for StepReprCompositeShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprCompositeShapeAspect::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
