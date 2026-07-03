// FILE: int_patch_bvh_traversal.rs
// occt: IntPatch_BVHTraversal

/// Implementation of IntPatch_BVHTraversal
pub struct IntPatch_BVHTraversal;

impl IntPatch_BVHTraversal {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_BVHTraversal
    }
}

impl Default for IntPatch_BVHTraversal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_BVHTraversal::new();
    }
}
