// FILE: t_naming_list_of_indexed_data_map_of_shape_list_of_shape.rs
// occt: TNaming_ListOfIndexedDataMapOfShapeListOfShape

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_List<TopTools_IndexedDataMapOfShapeListOfShape>
//!    TNaming_ListOfIndexedDataMapOfShapeListOfShape;`
//!
//! A list whose items are indexed data maps Shape -> ListOfShape
//! (the ancestor maps used by TNaming selection). Both layers are
//! implemented with genuine OCCT semantics: 1-based indexed map,
//! IsSame shape identity, list Append/Prepend/RemoveFirst.

use std::collections::HashMap;

/// Local stand-in for `TopoDS_Shape` (IsSame identity).
#[derive(Clone, Debug)]
pub struct NamingShapeStubLid {
    pub tshape_id: u64,
    pub location_id: u32,
}

impl NamingShapeStubLid {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        NamingShapeStubLid { tshape_id, location_id }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }
}

#[derive(Clone, Debug)]
struct NamingShapeKeyLid(NamingShapeStubLid);

impl PartialEq for NamingShapeKeyLid {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for NamingShapeKeyLid {}
impl std::hash::Hash for NamingShapeKeyLid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
    }
}

/// `TopTools_ListOfShape` (item value).
pub type TopToolsListOfShapeLid = Vec<NamingShapeStubLid>;

/// `TopTools_IndexedDataMapOfShapeListOfShape` with 1-based indices.
#[derive(Default, Clone)]
pub struct TopToolsIndexedDataMapShapeListLid {
    keys: Vec<NamingShapeStubLid>,
    values: Vec<TopToolsListOfShapeLid>,
    index_of: HashMap<NamingShapeKeyLid, usize>, // 1-based
}

impl TopToolsIndexedDataMapShapeListLid {
    pub fn new() -> Self {
        TopToolsIndexedDataMapShapeListLid {
            keys: Vec::new(),
            values: Vec::new(),
            index_of: HashMap::new(),
        }
    }

    /// Add — returns existing index when the key is already present
    /// (value untouched), otherwise appends and returns the new index.
    pub fn add(&mut self, key: NamingShapeStubLid, value: TopToolsListOfShapeLid) -> usize {
        if let Some(&i) = self.index_of.get(&NamingShapeKeyLid(key.clone())) {
            return i;
        }
        self.keys.push(key.clone());
        self.values.push(value);
        let idx = self.keys.len();
        self.index_of.insert(NamingShapeKeyLid(key), idx);
        idx
    }

    pub fn contains(&self, key: &NamingShapeStubLid) -> bool {
        self.index_of.contains_key(&NamingShapeKeyLid(key.clone()))
    }

    pub fn find_index(&self, key: &NamingShapeStubLid) -> usize {
        self.index_of
            .get(&NamingShapeKeyLid(key.clone()))
            .copied()
            .unwrap_or(0)
    }

    /// FindKey(index) — 1-based.
    pub fn find_key(&self, index: usize) -> &NamingShapeStubLid {
        assert!(index >= 1 && index <= self.keys.len(), "IndexedDataMap: index out of range");
        &self.keys[index - 1]
    }

    /// FindFromIndex(index) — 1-based.
    pub fn find_from_index(&self, index: usize) -> &TopToolsListOfShapeLid {
        assert!(index >= 1 && index <= self.values.len(), "IndexedDataMap: index out of range");
        &self.values[index - 1]
    }

    /// ChangeFromKey — mutable ancestor list for a shape.
    pub fn change_from_key(
        &mut self,
        key: &NamingShapeStubLid,
    ) -> Option<&mut TopToolsListOfShapeLid> {
        let idx = self.find_index(key);
        if idx == 0 {
            None
        } else {
            Some(&mut self.values[idx - 1])
        }
    }

    pub fn extent(&self) -> usize {
        self.keys.len()
    }
}

/// `TNaming_ListOfIndexedDataMapOfShapeListOfShape` (NCollection_List).
#[derive(Default)]
pub struct TNamingListOfIndexedDataMapOfShapeListOfShape {
    items: Vec<TopToolsIndexedDataMapShapeListLid>,
}

impl TNamingListOfIndexedDataMapOfShapeListOfShape {
    pub fn new() -> Self {
        TNamingListOfIndexedDataMapOfShapeListOfShape { items: Vec::new() }
    }

    pub fn extent(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, map: TopToolsIndexedDataMapShapeListLid) {
        self.items.push(map);
    }

    pub fn prepend(&mut self, map: TopToolsIndexedDataMapShapeListLid) {
        self.items.insert(0, map);
    }

    pub fn first(&self) -> &TopToolsIndexedDataMapShapeListLid {
        assert!(!self.items.is_empty(), "List: First on empty list");
        &self.items[0]
    }

    pub fn last(&self) -> &TopToolsIndexedDataMapShapeListLid {
        assert!(!self.items.is_empty(), "List: Last on empty list");
        self.items.last().unwrap()
    }

    /// RemoveFirst.
    pub fn remove_first(&mut self) {
        assert!(!self.items.is_empty(), "List: RemoveFirst on empty list");
        self.items.remove(0);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// `TNaming_ListIteratorOfListOfIndexedDataMapOfShapeListOfShape`.
    pub fn iter(&self) -> impl Iterator<Item = &TopToolsIndexedDataMapShapeListLid> {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh(id: u64) -> NamingShapeStubLid {
        NamingShapeStubLid::new(id, 0)
    }

    #[test]
    fn indexed_map_semantics() {
        let mut map = TopToolsIndexedDataMapShapeListLid::new();
        let edge = sh(10);
        let idx = map.add(edge.clone(), vec![sh(100), sh(101)]);
        assert_eq!(idx, 1);
        // Re-adding same key keeps existing value and index.
        assert_eq!(map.add(sh(10), vec![]), 1);
        assert_eq!(map.find_from_index(1).len(), 2);
        assert_eq!(map.find_index(&edge), 1);
        assert_eq!(map.find_index(&sh(99)), 0);
        assert!(map.find_key(1).is_same(&edge));
    }

    #[test]
    fn change_from_key_extends_ancestors() {
        let mut map = TopToolsIndexedDataMapShapeListLid::new();
        map.add(sh(1), vec![sh(2)]);
        map.change_from_key(&sh(1)).unwrap().push(sh(3));
        assert_eq!(map.find_from_index(1).len(), 2);
        assert!(map.change_from_key(&sh(42)).is_none());
    }

    #[test]
    fn list_of_maps_generations() {
        let mut list = TNamingListOfIndexedDataMapOfShapeListOfShape::new();
        let mut gen1 = TopToolsIndexedDataMapShapeListLid::new();
        gen1.add(sh(1), vec![sh(11)]);
        let mut gen2 = TopToolsIndexedDataMapShapeListLid::new();
        gen2.add(sh(2), vec![sh(21), sh(22)]);
        list.append(gen1);
        list.append(gen2);
        assert_eq!(list.extent(), 2);
        assert_eq!(list.first().extent(), 1);
        assert_eq!(list.last().find_from_index(1).len(), 2);
        list.remove_first();
        assert_eq!(list.extent(), 1);
        assert!(list.first().contains(&sh(2)));
    }
}
