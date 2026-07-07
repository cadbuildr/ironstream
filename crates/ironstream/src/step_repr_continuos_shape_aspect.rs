// FILE: step_repr_continuos_shape_aspect.rs
// occt: StepRepr_ContinuosShapeAspect

/// StepRepr_ContinuosShapeAspect:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_CompositeShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprContinuosShapeAspect {}

impl StepReprContinuosShapeAspect {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprContinuosShapeAspect {}
    }
}

impl Default for StepReprContinuosShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprContinuosShapeAspect::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
