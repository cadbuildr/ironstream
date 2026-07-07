// FILE: bvh_primitive_set.rs
// occt: BVH_PrimitiveSet

/// Primitive set for BVH tree.
pub struct BvhPrimitiveSet;

impl BvhPrimitiveSet {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BvhPrimitiveSet {
    fn default() -> Self {
        Self::new()
    }
}
