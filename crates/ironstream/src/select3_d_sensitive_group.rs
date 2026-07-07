// FILE: select3_d_sensitive_group.rs
// occt: Select3D_SensitiveGroup

use std::collections::BTreeMap;

/// A framework to define selection of a sensitive group
/// containing multiple 3D sensitive entities.
pub struct Select3DSensitiveGroup {
    owner_id: Option<()>, // TODO: real owner type
    entities: BTreeMap<usize, ()>, // TODO: replace with real Select3D_SensitiveEntity
    must_match_all: bool,
    check_overlap_all: bool,
    center: (f64, f64, f64), // TODO: replace with real gp_Pnt
    bnd_box: Option<()>,    // TODO: replace with real Select3D_BndBox3d
    bvh_prim_indexes: Vec<i32>,
    detected_idx: i32,
}

impl Select3DSensitiveGroup {
    /// Creates an empty sensitive group.
    pub fn new(owner_id: Option<()>, must_match_all: bool) -> Self {
        Select3DSensitiveGroup {
            owner_id,
            entities: BTreeMap::new(),
            must_match_all,
            check_overlap_all: false,
            center: (0.0, 0.0, 0.0),
            bnd_box: None,
            bvh_prim_indexes: Vec::new(),
            detected_idx: -1,
        }
    }

    /// Returns the number of entities in the group.
    pub fn nb_sub_elements(&self) -> usize {
        self.entities.len()
    }

    /// Returns whether all entities must be matched.
    pub fn must_match_all(&self) -> bool {
        self.must_match_all
    }

    /// Sets whether all entities must be matched.
    pub fn set_match_type(&mut self, must_match_all: bool) {
        self.must_match_all = must_match_all;
    }

    /// Returns whether to check overlap with all entities.
    pub fn to_check_overlap_all(&self) -> bool {
        self.check_overlap_all
    }

    /// Sets whether to check overlap with all entities.
    pub fn set_check_overlap_all(&mut self, check_all: bool) {
        self.check_overlap_all = check_all;
    }

    /// Returns the last detected entity index, or -1 if none.
    pub fn last_detected_entity_index(&self) -> i32 {
        if self.detected_idx != -1 && (self.detected_idx as usize) < self.bvh_prim_indexes.len() {
            self.bvh_prim_indexes[self.detected_idx as usize]
        } else {
            -1
        }
    }

    /// Clears all entities from the group.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.bvh_prim_indexes.clear();
        self.detected_idx = -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_group() {
        let group = Select3DSensitiveGroup::new(None, true);
        assert_eq!(group.nb_sub_elements(), 0);
        assert!(group.must_match_all());
    }

    #[test]
    fn test_match_type() {
        let mut group = Select3DSensitiveGroup::new(None, true);
        assert!(group.must_match_all());

        group.set_match_type(false);
        assert!(!group.must_match_all());
    }

    #[test]
    fn test_check_overlap_all() {
        let mut group = Select3DSensitiveGroup::new(None, true);
        assert!(!group.to_check_overlap_all());

        group.set_check_overlap_all(true);
        assert!(group.to_check_overlap_all());
    }

    #[test]
    fn test_last_detected_entity_index() {
        let mut group = Select3DSensitiveGroup::new(None, true);
        assert_eq!(group.last_detected_entity_index(), -1);

        group.detected_idx = 0;
        group.bvh_prim_indexes.push(42);
        assert_eq!(group.last_detected_entity_index(), 42);
    }

    #[test]
    fn test_clear() {
        let mut group = Select3DSensitiveGroup::new(None, true);
        group.bvh_prim_indexes.push(1);
        group.detected_idx = 0;

        group.clear();
        assert_eq!(group.nb_sub_elements(), 0);
        assert_eq!(group.detected_idx, -1);
    }
}
