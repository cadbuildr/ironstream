// FILE: toptools_map.rs
// occt: TopTools_MapOfShape, TopTools_DataMapOfShapeShape,
//       TopTools_IndexedMapOfShape, TopTools_ListOfShape

use std::collections::HashMap;

/// Set of shape IDs (no duplicates).
/// occt: TopTools_MapOfShape
#[derive(Clone, Debug, Default)]
pub struct TopToolsMapOfShape {
    ids: std::collections::HashSet<u32>,
}

impl TopToolsMapOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, shape_id: u32) -> bool { self.ids.insert(shape_id) }
    pub fn remove(&mut self, shape_id: u32) -> bool { self.ids.remove(&shape_id) }
    pub fn contains(&self, shape_id: u32) -> bool { self.ids.contains(&shape_id) }
    pub fn extent(&self) -> usize { self.ids.len() }
    pub fn is_empty(&self) -> bool { self.ids.is_empty() }
    pub fn clear(&mut self) { self.ids.clear(); }

    pub fn unite(&mut self, other: &TopToolsMapOfShape) {
        for &id in &other.ids { self.ids.insert(id); }
    }
    pub fn intersect(&mut self, other: &TopToolsMapOfShape) {
        self.ids.retain(|id| other.ids.contains(id));
    }
    pub fn subtract(&mut self, other: &TopToolsMapOfShape) {
        self.ids.retain(|id| !other.ids.contains(id));
    }
}

/// Map from shape ID to another shape ID.
/// occt: TopTools_DataMapOfShapeShape
#[derive(Clone, Debug, Default)]
pub struct TopToolsDataMapOfShapeShape {
    map: HashMap<u32, u32>,
}

impl TopToolsDataMapOfShapeShape {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key: u32, value: u32) { self.map.insert(key, value); }
    pub fn unbind(&mut self, key: u32) { self.map.remove(&key); }
    pub fn find(&self, key: u32) -> Option<u32> { self.map.get(&key).copied() }
    pub fn is_bound(&self, key: u32) -> bool { self.map.contains_key(&key) }
    pub fn extent(&self) -> usize { self.map.len() }
    pub fn clear(&mut self) { self.map.clear(); }
}

/// Indexed map of shape IDs — each shape gets a stable 1-based index.
/// occt: TopTools_IndexedMapOfShape
#[derive(Clone, Debug, Default)]
pub struct TopToolsIndexedMapOfShape {
    items: Vec<u32>,
    index: HashMap<u32, usize>,
}

impl TopToolsIndexedMapOfShape {
    pub fn new() -> Self { Self::default() }

    /// Add a shape and return its 1-based index. Duplicates return existing index.
    pub fn add(&mut self, shape_id: u32) -> usize {
        if let Some(&i) = self.index.get(&shape_id) { return i; }
        self.items.push(shape_id);
        let i = self.items.len();
        self.index.insert(shape_id, i);
        i
    }

    pub fn find_index(&self, shape_id: u32) -> usize {
        self.index.get(&shape_id).copied().unwrap_or(0)
    }

    pub fn find_key(&self, idx: usize) -> Option<u32> {
        if idx == 0 { return None; }
        self.items.get(idx - 1).copied()
    }

    pub fn contains(&self, shape_id: u32) -> bool { self.index.contains_key(&shape_id) }
    pub fn extent(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear(); self.index.clear(); }

    pub fn substitute(&mut self, idx: usize, new_shape_id: u32) {
        if idx == 0 || idx > self.items.len() { return; }
        let old = self.items[idx - 1];
        self.index.remove(&old);
        self.items[idx - 1] = new_shape_id;
        self.index.insert(new_shape_id, idx);
    }
}

/// Ordered list of shape IDs (allows duplicates).
/// occt: TopTools_ListOfShape
#[derive(Clone, Debug, Default)]
pub struct TopToolsListOfShape {
    list: Vec<u32>,
}

impl TopToolsListOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, shape_id: u32) { self.list.push(shape_id); }
    pub fn prepend(&mut self, shape_id: u32) { self.list.insert(0, shape_id); }

    pub fn first(&self) -> Option<u32> { self.list.first().copied() }
    pub fn last(&self) -> Option<u32> { self.list.last().copied() }

    pub fn remove_first(&mut self) -> Option<u32> {
        if self.list.is_empty() { None } else { Some(self.list.remove(0)) }
    }

    pub fn extent(&self) -> usize { self.list.len() }
    pub fn is_empty(&self) -> bool { self.list.is_empty() }
    pub fn clear(&mut self) { self.list.clear(); }

    pub fn contains(&self, shape_id: u32) -> bool { self.list.contains(&shape_id) }

    /// Append all shapes from another list.
    pub fn append_list(&mut self, other: &TopToolsListOfShape) {
        self.list.extend_from_slice(&other.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_of_shape_add_contains() {
        let mut m = TopToolsMapOfShape::new();
        assert!(m.add(1));
        assert!(m.add(2));
        assert!(!m.add(1)); // duplicate
        assert_eq!(m.extent(), 2);
        assert!(m.contains(1));
        m.remove(1);
        assert!(!m.contains(1));
    }

    #[test]
    fn map_of_shape_operations() {
        let mut a = TopToolsMapOfShape::new();
        let mut b = TopToolsMapOfShape::new();
        a.add(1); a.add(2); a.add(3);
        b.add(2); b.add(4);
        a.unite(&b);
        assert_eq!(a.extent(), 4);
        a.intersect(&b);
        // occt: NCollection_Map::Intersect keeps exactly the common elements;
        // after unite, a = {1,2,3,4} and a ∩ {2,4} = {2,4}.
        assert_eq!(a.extent(), 2);
        assert!(a.contains(2));
        assert!(a.contains(4));
    }

    #[test]
    fn data_map_bind_find() {
        let mut m = TopToolsDataMapOfShapeShape::new();
        m.bind(1, 100);
        m.bind(2, 200);
        assert_eq!(m.find(1), Some(100));
        assert!(m.is_bound(2));
        m.unbind(1);
        assert!(!m.is_bound(1));
    }

    #[test]
    fn indexed_map_add_find() {
        let mut m = TopToolsIndexedMapOfShape::new();
        let i1 = m.add(10);
        let i2 = m.add(20);
        let i3 = m.add(10); // duplicate
        assert_eq!(i1, 1); assert_eq!(i2, 2); assert_eq!(i3, 1);
        assert_eq!(m.find_key(1), Some(10));
        assert_eq!(m.find_index(20), 2);
        assert_eq!(m.find_index(999), 0);
    }

    #[test]
    fn list_of_shape_basic() {
        let mut l = TopToolsListOfShape::new();
        l.append(1); l.append(2); l.prepend(0);
        assert_eq!(l.extent(), 3);
        assert_eq!(l.first(), Some(0));
        assert_eq!(l.last(), Some(2));
        l.remove_first();
        assert_eq!(l.extent(), 2);
    }
}
