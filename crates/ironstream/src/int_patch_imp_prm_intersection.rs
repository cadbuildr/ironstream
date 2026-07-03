// FILE: int_patch_imp_prm_intersection.rs
// occt: IntPatch_ImpPrmIntersection

/// Implementation of IntPatch_ImpPrmIntersection
pub struct IntPatch_ImpPrmIntersection;

impl IntPatch_ImpPrmIntersection {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_ImpPrmIntersection
    }
}

impl Default for IntPatch_ImpPrmIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_ImpPrmIntersection::new();
    }
}
