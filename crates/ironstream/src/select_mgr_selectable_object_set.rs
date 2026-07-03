// FILE: select_mgr_selectable_object_set.rs
// occt: SelectMgr_SelectableObjectSet

use std::collections::HashMap;

/// Enumeration of BVH subsets for selectable objects.
/// Each subset has an independent BVH tree and is updated based on
/// the object's persistence type.
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BvhSubset {
    /// Normal world-space 3D objects
    Subset3d = 0,
    /// 3D persistent objects (rotate, pan, zoom persistence)
    Subset3dPersistent = 1,
    /// 2D persistent objects (camera projection dependent)
    Subset2dPersistent = 2,
    /// 3D persistent objects with orthogonal persistence mode
    SubsetOrtho3dPersistent = 3,
    /// 2D persistent objects with orthogonal persistence mode
    SubsetOrtho2dPersistent = 4,
}

const SUBSET_COUNT: usize = 5;

/// Set of selectable objects organized by persistence type.
/// Maintains multiple BVH trees, one for each subset of objects.
#[derive(Debug)]
pub struct SelectMgrSelectableObjectSet {
    /// Objects organized by subset type
    objects: [HashMap<usize, String>; SUBSET_COUNT],
    /// BVH trees for each subset (represented as empty for simplicity)
    bvh: [Vec<()>; SUBSET_COUNT],
    /// Dirty flags for each subset
    is_dirty: [bool; SUBSET_COUNT],
}

impl SelectMgrSelectableObjectSet {
    /// Creates a new empty objects set.
    pub fn new() -> Self {
        SelectMgrSelectableObjectSet {
            objects: [
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
            ],
            bvh: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            is_dirty: [true; SUBSET_COUNT],
        }
    }

    /// Adds a selectable object to the set.
    /// Returns true if the object was added, false if it already exists.
    pub fn append(&mut self, object_id: usize, subset: BvhSubset) -> bool {
        let subset_idx = subset as usize;
        if self.objects[subset_idx].contains_key(&object_id) {
            return false;
        }
        self.objects[subset_idx].insert(object_id, format!("object_{}", object_id));
        self.is_dirty[subset_idx] = true;
        true
    }

    /// Removes a selectable object from the set.
    /// Returns true if the object was removed, false if it wasn't found.
    pub fn remove(&mut self, object_id: usize) -> bool {
        for subset_idx in 0..SUBSET_COUNT {
            if self.objects[subset_idx].remove(&object_id).is_some() {
                self.is_dirty[subset_idx] = true;
                return true;
            }
        }
        false
    }

    /// Changes the subset of an object (when its persistence type changes).
    pub fn change_subset(&mut self, object_id: usize, new_subset: BvhSubset) {
        if self.remove(object_id) {
            self.append(object_id, new_subset);
        }
    }

    /// Updates BVH trees for outdated subsets.
    pub fn update_bvh(&mut self) {
        for i in 0..SUBSET_COUNT {
            if self.is_dirty[i] {
                self.bvh[i].clear();
                self.is_dirty[i] = false;
            }
        }
    }

    /// Marks every BVH subset for update.
    pub fn mark_dirty(&mut self) {
        for i in 0..SUBSET_COUNT {
            self.is_dirty[i] = true;
        }
    }

    /// Returns true if the set contains the given object.
    pub fn contains(&self, object_id: usize) -> bool {
        self.objects.iter().any(|map| map.contains_key(&object_id))
    }

    /// Returns true if the object set is empty.
    pub fn is_empty(&self) -> bool {
        self.objects.iter().all(|map| map.is_empty())
    }

    /// Returns true if the specified subset is empty.
    pub fn is_empty_subset(&self, subset: BvhSubset) -> bool {
        self.objects[subset as usize].is_empty()
    }

    /// Returns an object from the subset by index.
    pub fn get_object_by_id(&self, subset: BvhSubset, index: usize) -> Option<String> {
        let subset_idx = subset as usize;
        let mut iter = self.objects[subset_idx].values();
        iter.nth(index).cloned()
    }
}

impl Default for SelectMgrSelectableObjectSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_set() {
        let set = SelectMgrSelectableObjectSet::new();
        assert!(set.is_empty());
    }

    #[test]
    fn test_append_object() {
        let mut set = SelectMgrSelectableObjectSet::new();
        assert!(set.append(1, BvhSubset::Subset3d));
        assert!(!set.append(1, BvhSubset::Subset3d)); // Already exists
        assert!(set.contains(1));
    }

    #[test]
    fn test_remove_object() {
        let mut set = SelectMgrSelectableObjectSet::new();
        set.append(1, BvhSubset::Subset3d);
        assert!(set.contains(1));
        assert!(set.remove(1));
        assert!(!set.contains(1));
        assert!(!set.remove(1)); // Not found
    }

    #[test]
    fn test_subset_empty() {
        let mut set = SelectMgrSelectableObjectSet::new();
        assert!(set.is_empty_subset(BvhSubset::Subset3d));
        set.append(1, BvhSubset::Subset3d);
        assert!(!set.is_empty_subset(BvhSubset::Subset3d));
    }

    #[test]
    fn test_change_subset() {
        let mut set = SelectMgrSelectableObjectSet::new();
        set.append(1, BvhSubset::Subset3d);
        assert!(set.contains(1));
        set.change_subset(1, BvhSubset::Subset3dPersistent);
        assert!(set.contains(1));
        assert!(set.is_empty_subset(BvhSubset::Subset3d));
        assert!(!set.is_empty_subset(BvhSubset::Subset3dPersistent));
    }

    #[test]
    fn test_mark_dirty() {
        let mut set = SelectMgrSelectableObjectSet::new();
        set.mark_dirty();
        assert!(set.is_dirty[BvhSubset::Subset3d as usize]);
    }

    #[test]
    fn test_update_bvh() {
        let mut set = SelectMgrSelectableObjectSet::new();
        set.mark_dirty();
        set.update_bvh();
        assert!(!set.is_dirty[BvhSubset::Subset3d as usize]);
    }

    #[test]
    fn test_get_object_by_id() {
        let mut set = SelectMgrSelectableObjectSet::new();
        set.append(1, BvhSubset::Subset3d);
        let obj = set.get_object_by_id(BvhSubset::Subset3d, 0);
        assert!(obj.is_some());
    }
}
