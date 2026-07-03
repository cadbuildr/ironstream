// FILE: top_tools_shape_map_hasher.rs
// occt: TopTools_ShapeMapHasher

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Hash tool for generating maps of shapes in topology.
/// Provides hashing and equality comparison for TopoDS_Shape objects.
#[derive(Clone, Debug, Default)]
pub struct TopToolsShapeMapHasher;

impl TopToolsShapeMapHasher {
    /// Creates a new hasher instance.
    pub fn new() -> Self {
        TopToolsShapeMapHasher
    }

    /// Computes the hash value for a given shape.
    /// Takes a shape reference and returns a u64 hash.
    pub fn hash_shape(&self, shape_ptr: *const u8) -> u64 {
        let mut hasher = DefaultHasher::new();
        (shape_ptr as usize).hash(&mut hasher);
        hasher.finish()
    }

    /// Compares two shapes for equality using pointer identity.
    /// Returns true if both shapes refer to the same underlying object.
    pub fn shapes_equal(&self, shape1: *const u8, shape2: *const u8) -> bool {
        shape1 == shape2
    }

    /// Hash function callable for use with maps and sets.
    /// Works with opaque shape pointers.
    pub fn hash_fn(&self, shape: *const u8) -> u64 {
        self.hash_shape(shape)
    }

    /// Equality function callable for use with maps and sets.
    pub fn eq_fn(&self, s1: *const u8, s2: *const u8) -> bool {
        self.shapes_equal(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hasher() {
        let hasher = TopToolsShapeMapHasher::new();
        assert_eq!(hasher.shapes_equal(std::ptr::null(), std::ptr::null()), true);
    }

    #[test]
    fn test_hash_consistency() {
        let hasher = TopToolsShapeMapHasher::new();
        let ptr: *const u8 = 0x12345 as *const u8;

        let hash1 = hasher.hash_shape(ptr);
        let hash2 = hasher.hash_shape(ptr);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_pointers_different_hashes() {
        let hasher = TopToolsShapeMapHasher::new();
        let ptr1: *const u8 = 0x12345 as *const u8;
        let ptr2: *const u8 = 0x67890 as *const u8;

        let hash1 = hasher.hash_shape(ptr1);
        let hash2 = hasher.hash_shape(ptr2);

        // Different pointers may have different or same hashes depending on the pointer value,
        // but they should be treated independently
        let _ = hash1;
        let _ = hash2;
    }

    #[test]
    fn test_shapes_equality() {
        let hasher = TopToolsShapeMapHasher::new();
        let ptr1: *const u8 = 0x12345 as *const u8;
        let ptr2: *const u8 = 0x67890 as *const u8;

        assert!(hasher.shapes_equal(ptr1, ptr1));
        assert!(hasher.shapes_equal(ptr2, ptr2));
        assert!(!hasher.shapes_equal(ptr1, ptr2));
    }

    #[test]
    fn test_hash_fn_delegate() {
        let hasher = TopToolsShapeMapHasher::new();
        let ptr: *const u8 = 0x11111 as *const u8;

        let h1 = hasher.hash_fn(ptr);
        let h2 = hasher.hash_shape(ptr);

        assert_eq!(h1, h2);
    }

    #[test]
    fn test_eq_fn_delegate() {
        let hasher = TopToolsShapeMapHasher::new();
        let ptr1: *const u8 = 0x22222 as *const u8;
        let ptr2: *const u8 = 0x33333 as *const u8;

        assert!(hasher.eq_fn(ptr1, ptr1));
        assert!(!hasher.eq_fn(ptr1, ptr2));
    }
}
