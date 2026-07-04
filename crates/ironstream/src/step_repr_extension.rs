// FILE: step_repr_extension.rs
// occt: StepRepr_Extension

/// StepRepr_Extension:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_DerivedShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprExtension {}

impl StepReprExtension {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprExtension {}
    }
}

impl Default for StepReprExtension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprExtension::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
