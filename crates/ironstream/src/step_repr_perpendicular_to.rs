// FILE: step_repr_perpendicular_to.rs
// occt: StepRepr_PerpendicularTo

/// StepRepr_PerpendicularTo:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_DerivedShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprPerpendicularTo {}

impl StepReprPerpendicularTo {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprPerpendicularTo {}
    }
}

impl Default for StepReprPerpendicularTo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprPerpendicularTo::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
