// FILE: step_repr_centre_of_symmetry.rs
// occt: StepRepr_CentreOfSymmetry

/// StepRepr_CentreOfSymmetry: Added for Dimensional Tolerances.
/// Inherits from StepRepr_DerivedShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprCentreOfSymmetry {}

impl StepReprCentreOfSymmetry {
    /// Create a new StepReprCentreOfSymmetry
    pub fn new() -> Self {
        StepReprCentreOfSymmetry {}
    }
}

impl Default for StepReprCentreOfSymmetry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_centre_of_symmetry() {
        let sym = StepReprCentreOfSymmetry::new();
        assert_eq!(std::mem::size_of_val(&sym), 0);
    }
}
