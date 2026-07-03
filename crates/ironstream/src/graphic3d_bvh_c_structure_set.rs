// FILE: graphic3d_bvh_c_structure_set.rs
// occt: Graphic3d_BvhCStructureSet

//! Set of structures for building a BVH (Bounding Volume Hierarchy) tree.
//!
//! This class maintains an indexed map of 3D structures and provides
//! operations for BVH construction and queries.

use std::collections::HashMap;

/// Placeholder for a 3D structure reference.
/// In the real OCCT, this would be Graphic3d_CStructure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructureId(usize);

/// 3D bounding box using doubles.
#[derive(Debug, Clone, Copy)]
pub struct BndBox3d {
    min: [f64; 3],
    max: [f64; 3],
}

impl BndBox3d {
    pub fn new() -> Self {
        BndBox3d {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        }
    }

    pub fn with_bounds(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> Self {
        BndBox3d {
            min: [min_x, min_y, min_z],
            max: [max_x, max_y, max_z],
        }
    }

    pub fn corner_min(&self) -> &[f64; 3] {
        &self.min
    }

    pub fn corner_max(&self) -> &[f64; 3] {
        &self.max
    }

    pub fn is_empty(&self) -> bool {
        self.min[0] > self.max[0] || self.min[1] > self.max[1] || self.min[2] > self.max[2]
    }

    pub fn center(&self, axis: usize) -> f64 {
        if axis < 3 && !self.is_empty() {
            (self.min[axis] + self.max[axis]) * 0.5
        } else {
            0.0
        }
    }
}

impl Default for BndBox3d {
    fn default() -> Self {
        Self::new()
    }
}

/// Set of structures for BVH clipping.
#[derive(Debug)]
pub struct BvhCStructureSet {
    /// Map of structure id to bounding box.
    structures: Vec<BndBox3d>,
    /// Map from structure id to index in structures vec.
    id_to_index: HashMap<StructureId, usize>,
    dirty: bool,
}

impl BvhCStructureSet {
    /// Creates an empty primitive set for BVH clipping.
    pub fn new() -> Self {
        BvhCStructureSet {
            structures: Vec::new(),
            id_to_index: HashMap::new(),
            dirty: false,
        }
    }

    /// Returns total number of structures.
    pub fn size(&self) -> usize {
        self.structures.len()
    }

    /// Returns the AABB of the structure at the given index.
    pub fn box_at(&self, idx: usize) -> Option<BndBox3d> {
        self.structures.get(idx).copied()
    }

    /// Calculates center of the AABB along given axis for structure at index.
    pub fn center(&self, idx: usize, axis: usize) -> f64 {
        if let Some(bbox) = self.structures.get(idx) {
            bbox.center(axis)
        } else {
            0.0
        }
    }

    /// Swaps structures with the given indices.
    ///
    /// Note: as in OCCT, Swap() does not mark the set dirty (it is used
    /// internally during BVH construction).
    pub fn swap(&mut self, idx1: usize, idx2: usize) {
        if idx1 < self.structures.len() && idx2 < self.structures.len() {
            self.structures.swap(idx1, idx2);
        }
    }

    /// Adds structure to the set.
    ///
    /// Returns true if structure was added, false if it already exists.
    pub fn add(&mut self, struct_id: StructureId, bbox: BndBox3d) -> bool {
        if self.id_to_index.contains_key(&struct_id) {
            return false;
        }

        let index = self.structures.len();
        self.structures.push(bbox);
        self.id_to_index.insert(struct_id, index);
        self.mark_dirty();
        true
    }

    /// Removes the given structure from the set.
    ///
    /// Returns true if structure was removed, false if it was not in the set.
    pub fn remove(&mut self, struct_id: StructureId) -> bool {
        if let Some(idx) = self.id_to_index.remove(&struct_id) {
            // Swap with last element
            let last_idx = self.structures.len() - 1;
            if idx != last_idx {
                self.structures.swap(idx, last_idx);
                // Update the mapping for the swapped structure
                if let Some(last_id) = self.id_to_index.iter()
                    .find(|(_, &v)| v == last_idx)
                    .map(|(&k, _)| k)
                {
                    self.id_to_index.insert(last_id, idx);
                }
            }
            self.structures.pop();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    /// Cleans the whole primitive set.
    pub fn clear(&mut self) {
        self.structures.clear();
        self.id_to_index.clear();
        self.mark_dirty();
    }

    /// Returns the bounding box of the structure with the given ID.
    pub fn get_structure_by_id(&self, struct_id: StructureId) -> Option<BndBox3d> {
        self.id_to_index
            .get(&struct_id)
            .and_then(|&idx| self.structures.get(idx))
            .copied()
    }

    /// Returns true if the set has been marked dirty (needs BVH rebuild).
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks the set as dirty, indicating BVH needs to be rebuilt.
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Marks the set as clean.
    pub fn validate(&mut self) {
        self.dirty = false;
    }
}

impl Default for BvhCStructureSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_c_structure_set_new() {
        let set = BvhCStructureSet::new();
        assert_eq!(set.size(), 0);
        // OCCT: BVH_ObjectTransient initializes myIsDirty to false;
        // a freshly created set is clean until structures are added.
        assert!(!set.is_dirty());
    }

    #[test]
    fn test_bvh_c_structure_set_add() {
        let mut set = BvhCStructureSet::new();
        let bbox = BndBox3d::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
        let struct_id = StructureId(1);

        assert!(set.add(struct_id, bbox));
        assert_eq!(set.size(), 1);
        assert!(set.is_dirty());

        // Adding the same structure again should fail
        assert!(!set.add(struct_id, bbox));
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_bvh_c_structure_set_remove() {
        let mut set = BvhCStructureSet::new();
        let bbox = BndBox3d::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
        let struct_id = StructureId(1);

        set.add(struct_id, bbox);
        assert_eq!(set.size(), 1);

        assert!(set.remove(struct_id));
        assert_eq!(set.size(), 0);

        // Removing again should fail
        assert!(!set.remove(struct_id));
    }

    #[test]
    fn test_bvh_c_structure_set_get_by_id() {
        let mut set = BvhCStructureSet::new();
        let bbox = BndBox3d::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
        let struct_id = StructureId(1);

        set.add(struct_id, bbox);
        let retrieved = set.get_structure_by_id(struct_id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_bvh_c_structure_set_clear() {
        let mut set = BvhCStructureSet::new();
        let bbox = BndBox3d::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);

        set.add(StructureId(1), bbox);
        set.add(StructureId(2), bbox);
        assert_eq!(set.size(), 2);

        set.clear();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_bvh_c_structure_set_box_at() {
        let mut set = BvhCStructureSet::new();
        let bbox = BndBox3d::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
        set.add(StructureId(1), bbox);

        let retrieved = set.box_at(0);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_bvh_c_structure_set_center() {
        let mut set = BvhCStructureSet::new();
        let bbox = BndBox3d::with_bounds(0.0, 2.0, 4.0, 6.0, 8.0, 10.0);
        set.add(StructureId(1), bbox);

        // Center should be at (3, 5, 7)
        assert_eq!(set.center(0, 0), 3.0);
        assert_eq!(set.center(0, 1), 5.0);
        assert_eq!(set.center(0, 2), 7.0);
    }

    #[test]
    fn test_bvh_c_structure_set_swap() {
        let mut set = BvhCStructureSet::new();
        let bbox1 = BndBox3d::with_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let bbox2 = BndBox3d::with_bounds(2.0, 2.0, 2.0, 3.0, 3.0, 3.0);

        set.add(StructureId(1), bbox1);
        set.add(StructureId(2), bbox2);

        set.swap(0, 1);

        // After swap, the centers should be different
        assert_eq!(set.center(0, 0), 2.5);
        assert_eq!(set.center(1, 0), 0.5);
    }

    #[test]
    fn test_bvh_c_structure_set_validate() {
        let mut set = BvhCStructureSet::new();
        // OCCT: a fresh set is clean (myIsDirty is initialized to false).
        assert!(!set.is_dirty());

        let bbox = BndBox3d::with_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        set.add(StructureId(1), bbox);
        assert!(set.is_dirty());

        set.validate();
        assert!(!set.is_dirty());
    }

    #[test]
    fn test_bnd_box3d_new() {
        let bbox = BndBox3d::new();
        assert!(bbox.is_empty());
    }

    #[test]
    fn test_bnd_box3d_with_bounds() {
        let bbox = BndBox3d::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
        assert!(!bbox.is_empty());
        assert_eq!(bbox.corner_min()[0], 0.0);
        assert_eq!(bbox.corner_max()[0], 3.0);
    }
}
