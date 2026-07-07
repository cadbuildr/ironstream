// FILE: step_repr_between_shape_aspect.rs
// occt: StepRepr_BetweenShapeAspect

/// StepRepr_BetweenShapeAspect: A shape aspect added for Dimensional Tolerances.
/// Inherits from StepRepr_ContinuosShapeAspect.
/// This is a marker class with no additional fields beyond the parent.
#[derive(Clone, Debug)]
pub struct StepReprBetweenShapeAspect {
    // Inherits from ContinuosShapeAspect
    // which inherits from CompositeShapeAspect
    // which inherits from ShapeAspect
}

impl StepReprBetweenShapeAspect {
    /// Create a new StepReprBetweenShapeAspect
    pub fn new() -> Self {
        StepReprBetweenShapeAspect {}
    }
}

impl Default for StepReprBetweenShapeAspect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_between_shape_aspect() {
        let aspect = StepReprBetweenShapeAspect::new();
        assert_eq!(std::mem::size_of_val(&aspect), 0);
    }
}
