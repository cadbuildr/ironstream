// FILE: int_patch_prm_prm_intersection.rs
// occt: IntPatch_PrmPrmIntersection

/// Implementation of IntPatch_PrmPrmIntersection
pub struct IntPatch_PrmPrmIntersection;

impl IntPatch_PrmPrmIntersection {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_PrmPrmIntersection
    }
}

impl Default for IntPatch_PrmPrmIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_PrmPrmIntersection::new();
    }
}
