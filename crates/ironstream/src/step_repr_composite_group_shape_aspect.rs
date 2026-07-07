// FILE: step_repr_composite_group_shape_aspect.rs
// occt: StepRepr_CompositeGroupShapeAspect

/// StepRepr_CompositeGroupShapeAspect:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_CompositeShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprCompositeGroupShapeAspect {}

impl StepReprCompositeGroupShapeAspect {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprCompositeGroupShapeAspect {}
    }
}

impl Default for StepReprCompositeGroupShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprCompositeGroupShapeAspect::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
