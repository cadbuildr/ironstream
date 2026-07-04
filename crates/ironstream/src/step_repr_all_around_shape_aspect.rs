// FILE: step_repr_all_around_shape_aspect.rs
// occt: StepRepr_AllAroundShapeAspect

/// Representation of STEP entity AllAroundShapeAspect.
#[derive(Clone, Debug, Default)]
pub struct StepReprAllAroundShapeAspect;

impl StepReprAllAroundShapeAspect {
    pub fn new() -> Self {
        StepReprAllAroundShapeAspect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _aspect = StepReprAllAroundShapeAspect::new();
    }
}
