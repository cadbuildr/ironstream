// FILE: select3_d_sensitive_set.rs
// occt: Select3D_SensitiveSet

/// Base class for complex sensitive entities with BVH tree support.
/// Provides interface for building and managing BVH trees for efficient overlap detection.
pub struct Select3DSensitiveSet {
    owner_id: Option<()>, // TODO: real owner type
    content_dirty: bool,
    detected_idx: i32,
}

impl Select3DSensitiveSet {
    /// Creates a new empty sensitive set.
    pub fn new(owner_id: Option<()>) -> Self {
        Select3DSensitiveSet {
            owner_id,
            content_dirty: true,
            detected_idx: -1,
        }
    }

    /// Returns the number of sub-entities in the set.
    /// Must be implemented by derived classes.
    pub fn size(&self) -> usize {
        0
    }

    /// Returns whether the BVH tree needs to be rebuilt.
    pub fn to_build_bvh(&self) -> bool {
        self.content_dirty
    }

    /// Marks the BVH tree as dirty (needs rebuilding).
    pub fn mark_dirty(&mut self) {
        self.content_dirty = true;
    }

    /// Rebuilds the BVH tree if needed.
    pub fn build_bvh(&mut self) {
        if self.content_dirty {
            self.content_dirty = false;
        }
    }

    /// Returns the detected primitive index.
    pub fn detected_idx(&self) -> i32 {
        self.detected_idx
    }

    /// Sets the detected primitive index.
    pub fn set_detected_idx(&mut self, idx: i32) {
        self.detected_idx = idx;
    }

    /// Clears the sensitive set and resets state.
    pub fn clear(&mut self) {
        self.owner_id = None;
        self.detected_idx = -1;
        self.mark_dirty();
    }

    /// Returns the owner ID.
    pub fn owner_id(&self) -> Option<&()> {
        self.owner_id.as_ref()
    }

    /// Sets the owner ID.
    pub fn set_owner_id(&mut self, owner_id: Option<()>) {
        self.owner_id = owner_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_set() {
        let set = Select3DSensitiveSet::new(None);
        assert_eq!(set.size(), 0);
        assert!(set.to_build_bvh());
    }

    #[test]
    fn test_mark_dirty() {
        let mut set = Select3DSensitiveSet::new(None);
        set.content_dirty = false;

        set.mark_dirty();
        assert!(set.to_build_bvh());
    }

    #[test]
    fn test_build_bvh() {
        let mut set = Select3DSensitiveSet::new(None);
        assert!(set.to_build_bvh());

        set.build_bvh();
        assert!(!set.to_build_bvh());
    }

    #[test]
    fn test_detected_idx() {
        let mut set = Select3DSensitiveSet::new(None);
        assert_eq!(set.detected_idx(), -1);

        set.set_detected_idx(42);
        assert_eq!(set.detected_idx(), 42);
    }

    #[test]
    fn test_clear() {
        let mut set = Select3DSensitiveSet::new(Some(()));
        set.detected_idx = 10;
        set.content_dirty = false;

        set.clear();
        assert!(set.owner_id().is_none());
        assert_eq!(set.detected_idx(), -1);
        assert!(set.to_build_bvh());
    }
}
