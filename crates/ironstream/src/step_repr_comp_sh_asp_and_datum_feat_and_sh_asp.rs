// FILE: step_repr_comp_sh_asp_and_datum_feat_and_sh_asp.rs
// occt: StepRepr_CompShAspAndDatumFeatAndShAsp

/// StepRepr_CompShAspAndDatumFeatAndShAsp:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_ShapeAspect.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprCompShAspAndDatumFeatAndShAsp {}

impl StepReprCompShAspAndDatumFeatAndShAsp {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprCompShAspAndDatumFeatAndShAsp {}
    }
}

impl Default for StepReprCompShAspAndDatumFeatAndShAsp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprCompShAspAndDatumFeatAndShAsp::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
