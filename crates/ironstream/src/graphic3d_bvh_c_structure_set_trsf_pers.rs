// FILE: graphic3d_bvh_c_structure_set_trsf_pers.rs
// occt: Graphic3d_BvhCStructureSetTrsfPers

use std::collections::BTreeMap;

/// Set of transformation persistent OpenGl_Structure for building BVH tree.
/// Provides built-in mechanism to invalidate tree when world view projection state changes.
/// Due to frequent invalidation of BVH tree the choice of BVH tree builder is made
/// in favor of BVH linear builder (quick rebuild).
pub struct Graphic3dBvhCStructureSetTrsfPers {
    /// Indexed map of structures (stored by pointer address for identity)
    structures: BTreeMap<usize, ()>,
    /// Marks object state as outdated (needs BVH rebuilding)
    is_dirty: bool,
}

impl Graphic3dBvhCStructureSetTrsfPers {
    /// Creates an empty primitive set for BVH clipping.
    pub fn new() -> Self {
        Self {
            structures: BTreeMap::new(),
            is_dirty: false,
        }
    }

    /// Returns total number of structures.
    pub fn size(&self) -> usize {
        self.structures.len()
    }

    /// Adds structure to the set.
    /// Returns true if structure added, otherwise returns false (structure already in the set).
    pub fn add(&mut self, struct_id: usize) -> bool {
        self.structures.insert(struct_id, ()).is_none()
    }

    /// Removes the given structure from the set.
    /// Returns true if structure removed, otherwise returns false (structure is not in the set).
    pub fn remove(&mut self, struct_id: usize) -> bool {
        self.structures.remove(&struct_id).is_some()
    }

    /// Cleans the whole primitive set.
    pub fn clear(&mut self) {
        self.structures.clear();
    }

    /// Marks object state as outdated (needs BVH rebuilding).
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    /// Returns whether the object state is marked as dirty.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
}

impl Default for Graphic3dBvhCStructureSetTrsfPers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let set = Graphic3dBvhCStructureSetTrsfPers::new();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_add_structure() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        assert!(set.add(1));
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_add_duplicate_returns_false() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        assert!(set.add(1));
        assert!(!set.add(1));
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_remove_structure() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        set.add(1);
        assert!(set.remove(1));
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_remove_nonexistent_returns_false() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        assert!(!set.remove(1));
    }

    #[test]
    fn test_clear_removes_all() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        set.add(1);
        set.add(2);
        set.add(3);
        assert_eq!(set.size(), 3);
        set.clear();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_mark_dirty() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        assert!(!set.is_dirty());
        set.mark_dirty();
        assert!(set.is_dirty());
    }

    #[test]
    fn test_multiple_adds() {
        let mut set = Graphic3dBvhCStructureSetTrsfPers::new();
        assert!(set.add(1));
        assert!(set.add(2));
        assert!(set.add(3));
        assert_eq!(set.size(), 3);
    }
}
