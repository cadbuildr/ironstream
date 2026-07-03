// FILE: int_patch_curv_int_surf.rs
// occt: IntPatch_CurvIntSurf

/// Implementation of IntPatch_CurvIntSurf
pub struct IntPatch_CurvIntSurf;

impl IntPatch_CurvIntSurf {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_CurvIntSurf
    }
}

impl Default for IntPatch_CurvIntSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_CurvIntSurf::new();
    }
}
