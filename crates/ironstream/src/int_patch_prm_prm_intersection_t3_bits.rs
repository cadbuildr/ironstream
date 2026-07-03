// FILE: int_patch_prm_prm_intersection_t3_bits.rs
// occt: IntPatch_PrmPrmIntersection_T3Bits

/// Implementation of IntPatch_PrmPrmIntersection_T3Bits
pub struct IntPatch_PrmPrmIntersection_T3Bits;

impl IntPatch_PrmPrmIntersection_T3Bits {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_PrmPrmIntersection_T3Bits
    }
}

impl Default for IntPatch_PrmPrmIntersection_T3Bits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_PrmPrmIntersection_T3Bits::new();
    }
}
