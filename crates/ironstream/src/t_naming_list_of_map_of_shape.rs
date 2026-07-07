// FILE: t_naming_list_of_map_of_shape.rs
// occt: TNaming_ListOfMapOfShape

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_List<TopTools_MapOfShape> TNaming_ListOfMapOfShape;`
//! plus `TNaming_ListIteratorOfListOfMapOfShape`.
//!
//! Items are shape sets (TopTools_MapOfShape with IsSame identity);
//! the list has NCollection_List semantics.

use std::collections::HashSet;

/// Local stand-in for `TopoDS_Shape` (IsSame identity).
#[derive(Clone, Debug)]
pub struct NamingShapeStubLms {
    pub tshape_id: u64,
    pub location_id: u32,
}

impl NamingShapeStubLms {
    pub fn new(tshape_id: u64, location_id: u32) -> Self {
        NamingShapeStubLms { tshape_id, location_id }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }
}

#[derive(Clone, Debug)]
struct NamingShapeKeyLms(NamingShapeStubLms);

impl PartialEq for NamingShapeKeyLms {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same(&other.0)
    }
}
impl Eq for NamingShapeKeyLms {}
impl std::hash::Hash for NamingShapeKeyLms {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.tshape_id.hash(state);
        self.0.location_id.hash(state);
    }
}

/// `TopTools_MapOfShape` (list item type).
#[derive(Default, Clone)]
pub struct TopToolsMapOfShapeLms {
    inner: HashSet<NamingShapeKeyLms>,
}

impl TopToolsMapOfShapeLms {
    pub fn new() -> Self {
        TopToolsMapOfShapeLms { inner: HashSet::new() }
    }

    /// Add — true when the shape was not in the map.
    pub fn add(&mut self, shape: NamingShapeStubLms) -> bool {
        self.inner.insert(NamingShapeKeyLms(shape))
    }

    pub fn contains(&self, shape: &NamingShapeStubLms) -> bool {
        self.inner.contains(&NamingShapeKeyLms(shape.clone()))
    }

    /// Remove — true when the shape was in the map.
    pub fn remove(&mut self, shape: &NamingShapeStubLms) -> bool {
        self.inner.remove(&NamingShapeKeyLms(shape.clone()))
    }

    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// `TNaming_ListOfMapOfShape` (NCollection_List semantics).
#[derive(Default)]
pub struct TNamingListOfMapOfShape {
    items: Vec<TopToolsMapOfShapeLms>,
}

impl TNamingListOfMapOfShape {
    pub fn new() -> Self {
        TNamingListOfMapOfShape { items: Vec::new() }
    }

    pub fn extent(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, map: TopToolsMapOfShapeLms) {
        self.items.push(map);
    }

    pub fn prepend(&mut self, map: TopToolsMapOfShapeLms) {
        self.items.insert(0, map);
    }

    pub fn first(&self) -> &TopToolsMapOfShapeLms {
        assert!(!self.items.is_empty(), "List: First on empty list");
        &self.items[0]
    }

    pub fn last(&self) -> &TopToolsMapOfShapeLms {
        assert!(!self.items.is_empty(), "List: Last on empty list");
        self.items.last().unwrap()
    }

    pub fn remove_first(&mut self) {
        assert!(!self.items.is_empty(), "List: RemoveFirst on empty list");
        self.items.remove(0);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// `TNaming_ListIteratorOfListOfMapOfShape`.
    pub fn iter(&self) -> impl Iterator<Item = &TopToolsMapOfShapeLms> {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh(id: u64) -> NamingShapeStubLms {
        NamingShapeStubLms::new(id, 0)
    }

    #[test]
    fn map_of_shape_dedup_by_is_same() {
        let mut m = TopToolsMapOfShapeLms::new();
        assert!(m.add(sh(7)));
        assert!(!m.add(sh(7)));
        assert!(m.add(NamingShapeStubLms::new(7, 1)), "moved shape is distinct");
        assert_eq!(m.extent(), 2);
        assert!(m.remove(&sh(7)));
        assert!(!m.contains(&sh(7)));
    }

    #[test]
    fn list_append_prepend_iterate() {
        let mut list = TNamingListOfMapOfShape::new();
        let mut m1 = TopToolsMapOfShapeLms::new();
        m1.add(sh(1));
        let mut m2 = TopToolsMapOfShapeLms::new();
        m2.add(sh(2));
        m2.add(sh(3));
        list.append(m1);
        list.prepend(m2);
        assert_eq!(list.extent(), 2);
        assert_eq!(list.first().extent(), 2, "prepended map is first");
        assert_eq!(list.last().extent(), 1);
        let total: usize = list.iter().map(|m| m.extent()).sum();
        assert_eq!(total, 3);
        list.remove_first();
        assert!(list.first().contains(&sh(1)));
    }
}
