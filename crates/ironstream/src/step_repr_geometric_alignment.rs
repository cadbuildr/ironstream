// FILE: step_repr_geometric_alignment.rs
// occt: StepRepr_GeometricAlignment

/// StepRepr_GeometricAlignment:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_DerivedShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprGeometricAlignment {}

impl StepReprGeometricAlignment {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprGeometricAlignment {}
    }
}

impl Default for StepReprGeometricAlignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprGeometricAlignment::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
