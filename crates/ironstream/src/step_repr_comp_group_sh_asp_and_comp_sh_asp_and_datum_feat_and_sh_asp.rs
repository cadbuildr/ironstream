// FILE: step_repr_comp_group_sh_asp_and_comp_sh_asp_and_datum_feat_and_sh_asp.rs
// occt: StepRepr_CompGroupShAspAndCompShAspAndDatumFeatAndShAsp

/// StepRepr_CompGroupShAspAndCompShAspAndDatumFeatAndShAsp:
/// Added for Dimensional Tolerances.
/// Inherits from StepRepr_CompShAspAndDatumFeatAndShAsp.
/// Marker class with no additional fields beyond parent.
#[derive(Clone, Debug)]
pub struct StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp {}

impl StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp {}
    }
}

impl Default for StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let obj = StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp::new();
        assert_eq!(std::mem::size_of_val(&obj), 0);
    }
}
