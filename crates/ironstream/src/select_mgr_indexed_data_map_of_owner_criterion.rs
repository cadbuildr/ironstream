// FILE: select_mgr_indexed_data_map_of_owner_criterion.rs
// occt: SelectMgr_IndexedDataMapOfOwnerCriterion

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_IndexedDataMap<opencascade::handle<SelectMgr_EntityOwner>,
//!    SelectMgr_SortCriterion> SelectMgr_IndexedDataMapOfOwnerCriterion;`
//!
//! Used by SelectMgr_ViewerSelector to accumulate picked owners with their
//! sort criteria. Indexed-data-map semantics: 1-based stable indices,
//! Add returns the (possibly existing) index, keys hash by owner identity.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `SelectMgr_EntityOwner`.
#[derive(Debug)]
pub struct EntityOwnerStubOc {
    pub selectable_name: String,
    pub priority: i32,
}

pub type HandleOwnerOc = Rc<EntityOwnerStubOc>;

/// Local `SelectMgr_SortCriterion`: data used to sort pick results.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SortCriterionOc {
    /// Depth along the picking ray.
    pub depth: f64,
    /// Distance from the picking line.
    pub min_dist: f64,
    /// Selection priority (higher wins).
    pub priority: i32,
    /// Tolerance used for the closeness test.
    pub tolerance: f64,
    /// Z-layer position (higher layer wins first).
    pub z_layer_position: i32,
}

impl SortCriterionOc {
    /// SelectMgr_SortCriterion::IsCloserDepth-style comparison:
    /// higher z-layer wins, then higher priority, then smaller depth.
    pub fn is_higher_priority(&self, other: &SortCriterionOc) -> bool {
        if self.z_layer_position != other.z_layer_position {
            return self.z_layer_position > other.z_layer_position;
        }
        if self.priority != other.priority {
            return self.priority > other.priority;
        }
        self.depth < other.depth
    }
}

#[derive(Clone)]
struct OwnerIdentityKeyOc(HandleOwnerOc);

impl PartialEq for OwnerIdentityKeyOc {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for OwnerIdentityKeyOc {}
impl std::hash::Hash for OwnerIdentityKeyOc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

/// `SelectMgr_IndexedDataMapOfOwnerCriterion` with NCollection_IndexedDataMap
/// semantics (1-based indices).
#[derive(Default)]
pub struct SelectMgrIndexedDataMapOfOwnerCriterion {
    keys: Vec<HandleOwnerOc>,
    values: Vec<SortCriterionOc>,
    index_of: HashMap<OwnerIdentityKeyOc, usize>, // 1-based
}

impl SelectMgrIndexedDataMapOfOwnerCriterion {
    pub fn new() -> Self {
        SelectMgrIndexedDataMapOfOwnerCriterion {
            keys: Vec::new(),
            values: Vec::new(),
            index_of: HashMap::new(),
        }
    }

    /// Add — returns the index of the key (existing index if already present;
    /// in that case the value is NOT replaced, per NCollection_IndexedDataMap).
    pub fn add(&mut self, key: HandleOwnerOc, value: SortCriterionOc) -> usize {
        if let Some(&idx) = self.index_of.get(&OwnerIdentityKeyOc(key.clone())) {
            return idx;
        }
        self.keys.push(key.clone());
        self.values.push(value);
        let idx = self.keys.len();
        self.index_of.insert(OwnerIdentityKeyOc(key), idx);
        idx
    }

    pub fn contains(&self, key: &HandleOwnerOc) -> bool {
        self.index_of.contains_key(&OwnerIdentityKeyOc(key.clone()))
    }

    /// FindKey — key stored at a 1-based index.
    pub fn find_key(&self, index: usize) -> &HandleOwnerOc {
        assert!(index >= 1 && index <= self.keys.len(), "IndexedDataMap: index out of range");
        &self.keys[index - 1]
    }

    /// FindFromIndex — value stored at a 1-based index.
    pub fn find_from_index(&self, index: usize) -> &SortCriterionOc {
        assert!(index >= 1 && index <= self.values.len(), "IndexedDataMap: index out of range");
        &self.values[index - 1]
    }

    /// ChangeFromIndex — mutable value at a 1-based index.
    pub fn change_from_index(&mut self, index: usize) -> &mut SortCriterionOc {
        assert!(index >= 1 && index <= self.values.len(), "IndexedDataMap: index out of range");
        &mut self.values[index - 1]
    }

    /// FindIndex — 0 when not found (OCCT convention).
    pub fn find_index(&self, key: &HandleOwnerOc) -> usize {
        self.index_of
            .get(&OwnerIdentityKeyOc(key.clone()))
            .copied()
            .unwrap_or(0)
    }

    /// FindFromKey.
    pub fn find_from_key(&self, key: &HandleOwnerOc) -> Option<&SortCriterionOc> {
        let idx = self.find_index(key);
        if idx == 0 {
            None
        } else {
            Some(&self.values[idx - 1])
        }
    }

    pub fn extent(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
        self.index_of.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crit(depth: f64, priority: i32) -> SortCriterionOc {
        SortCriterionOc {
            depth,
            min_dist: 0.0,
            priority,
            tolerance: 0.001,
            z_layer_position: 0,
        }
    }

    #[test]
    fn one_based_indices_stable() {
        let mut map = SelectMgrIndexedDataMapOfOwnerCriterion::new();
        let o1 = Rc::new(EntityOwnerStubOc { selectable_name: "box".into(), priority: 5 });
        let o2 = Rc::new(EntityOwnerStubOc { selectable_name: "edge".into(), priority: 7 });
        assert_eq!(map.add(o1.clone(), crit(10.0, 5)), 1);
        assert_eq!(map.add(o2.clone(), crit(8.0, 7)), 2);
        assert_eq!(map.find_index(&o1), 1);
        assert!(Rc::ptr_eq(map.find_key(2), &o2));
        assert_eq!(map.find_from_index(2).priority, 7);
    }

    #[test]
    fn re_add_keeps_existing_value() {
        let mut map = SelectMgrIndexedDataMapOfOwnerCriterion::new();
        let o = Rc::new(EntityOwnerStubOc { selectable_name: "face".into(), priority: 1 });
        assert_eq!(map.add(o.clone(), crit(3.0, 1)), 1);
        // Adding the same key again returns index 1 and does not replace.
        assert_eq!(map.add(o.clone(), crit(99.0, 9)), 1);
        assert_eq!(map.extent(), 1);
        assert_eq!(map.find_from_index(1).depth, 3.0);
        // But ChangeFromIndex can mutate in place.
        map.change_from_index(1).depth = 5.0;
        assert_eq!(map.find_from_key(&o).unwrap().depth, 5.0);
    }

    #[test]
    fn missing_key_gives_zero_index() {
        let map = SelectMgrIndexedDataMapOfOwnerCriterion::new();
        let ghost = Rc::new(EntityOwnerStubOc { selectable_name: "ghost".into(), priority: 0 });
        assert_eq!(map.find_index(&ghost), 0);
        assert!(map.find_from_key(&ghost).is_none());
        assert!(map.is_empty());
    }

    #[test]
    fn sort_criterion_ordering() {
        // Higher priority wins regardless of depth.
        assert!(crit(100.0, 8).is_higher_priority(&crit(1.0, 2)));
        // Equal priority: smaller depth wins.
        assert!(crit(1.0, 2).is_higher_priority(&crit(5.0, 2)));
        // Higher z-layer dominates everything.
        let mut top = crit(50.0, 0);
        top.z_layer_position = 1;
        assert!(top.is_higher_priority(&crit(0.1, 9)));
    }
}
