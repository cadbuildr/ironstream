// FILE: step_shape_faceted_brep.rs
// occt: StepShape_FacetedBrep

//! Representation of STEP entity FacetedBrep

#[derive(Clone, Debug)]
pub struct FacetedBrep {
    // Inherits from ManifoldSolidBrep
}

impl FacetedBrep {
    /// Returns a FacetedBrep
    pub fn new() -> Self {
        FacetedBrep {}
    }
}

impl Default for FacetedBrep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let brep = FacetedBrep::new();
        let _ = brep;
    }

    #[test]
    fn test_default() {
        let brep = FacetedBrep::default();
        let _ = brep;
    }
}
