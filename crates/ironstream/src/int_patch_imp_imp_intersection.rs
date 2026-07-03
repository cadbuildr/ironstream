// FILE: int_patch_imp_imp_intersection.rs
// occt: IntPatch_ImpImpIntersection

/// Implementation of IntPatch_ImpImpIntersection
pub struct IntPatch_ImpImpIntersection;

impl IntPatch_ImpImpIntersection {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_ImpImpIntersection
    }
}

impl Default for IntPatch_ImpImpIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_ImpImpIntersection::new();
    }
}
