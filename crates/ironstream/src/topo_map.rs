// FILE: topo_map.rs
// occt: TopTools_IndexedMapOfShape, TopTools_MapOfShape, TopTools_DataMapOfShapeShape

use std::collections::HashMap;

// occt: TopTools_MapOfShape
/// Unordered set of shapes identified by their topology pointer.
#[derive(Clone, Debug, Default)]
pub struct MapOfShape {
    entries: Vec<ShapeKey>,
}

#[derive(Clone, Debug)]
pub struct ShapeKey {
    pub id: u64,
    pub shape_type: u8,
}

impl MapOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, key: ShapeKey) -> bool {
        if self.contains(key.id) { return false; }
        self.entries.push(key);
        true
    }

    pub fn contains(&self, id: u64) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }

    pub fn extent(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn clear(&mut self) { self.entries.clear(); }
}

// occt: TopTools_IndexedMapOfShape
/// Ordered map from shape to integer index (1-based).
#[derive(Clone, Debug, Default)]
pub struct IndexedMapOfShape {
    shapes: Vec<ShapeKey>,
}

impl IndexedMapOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, key: ShapeKey) -> usize {
        if let Some(i) = self.find_index(key.id) { return i; }
        self.shapes.push(key);
        self.shapes.len()
    }

    pub fn find_index(&self, id: u64) -> Option<usize> {
        self.shapes.iter().position(|s| s.id == id).map(|i| i + 1)
    }

    pub fn find_key(&self, index: usize) -> Option<&ShapeKey> {
        self.shapes.get(index.saturating_sub(1))
    }

    pub fn extent(&self) -> usize { self.shapes.len() }
    pub fn is_empty(&self) -> bool { self.shapes.is_empty() }
    pub fn clear(&mut self) { self.shapes.clear(); }
}

// occt: TopTools_DataMapOfShapeShape
/// Map from shape to shape.
#[derive(Clone, Debug, Default)]
pub struct DataMapOfShapeShape {
    map: HashMap<u64, u64>,
}

impl DataMapOfShapeShape {
    pub fn new() -> Self { Self::default() }
    pub fn bind(&mut self, from: u64, to: u64) { self.map.insert(from, to); }
    pub fn find(&self, from: u64) -> Option<u64> { self.map.get(&from).copied() }
    pub fn is_bound(&self, from: u64) -> bool { self.map.contains_key(&from) }
    pub fn unbind(&mut self, from: u64) { self.map.remove(&from); }
    pub fn extent(&self) -> usize { self.map.len() }
    pub fn clear(&mut self) { self.map.clear(); }
}

// occt: TopTools_DataMapOfShapeListOfShape
/// Map from shape to list of shapes.
#[derive(Clone, Debug, Default)]
pub struct DataMapOfShapeListOfShape {
    map: HashMap<u64, Vec<u64>>,
}

impl DataMapOfShapeListOfShape {
    pub fn new() -> Self { Self::default() }
    pub fn bind(&mut self, key: u64, list: Vec<u64>) { self.map.insert(key, list); }
    pub fn find(&self, key: u64) -> Option<&Vec<u64>> { self.map.get(&key) }
    pub fn append(&mut self, key: u64, val: u64) { self.map.entry(key).or_default().push(val); }
    pub fn extent(&self) -> usize { self.map.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_of_shape_add_contains() {
        let mut m = MapOfShape::new();
        m.add(ShapeKey { id: 1, shape_type: 0 });
        assert!(m.contains(1));
        assert!(!m.contains(2));
        assert_eq!(m.extent(), 1);
    }

    #[test]
    fn indexed_map_add_find() {
        let mut m = IndexedMapOfShape::new();
        let i1 = m.add(ShapeKey { id: 10, shape_type: 1 });
        let i2 = m.add(ShapeKey { id: 20, shape_type: 2 });
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(m.find_index(10), Some(1));
        assert_eq!(m.find_key(1).unwrap().id, 10);
    }

    #[test]
    fn indexed_map_no_duplicate() {
        let mut m = IndexedMapOfShape::new();
        let i1 = m.add(ShapeKey { id: 5, shape_type: 0 });
        let i2 = m.add(ShapeKey { id: 5, shape_type: 0 });
        assert_eq!(i1, i2);
        assert_eq!(m.extent(), 1);
    }

    #[test]
    fn datamap_bind_find() {
        let mut d = DataMapOfShapeShape::new();
        d.bind(1, 2);
        assert_eq!(d.find(1), Some(2));
        assert!(d.is_bound(1));
        assert!(!d.is_bound(99));
    }

    #[test]
    fn datamap_list_append() {
        let mut d = DataMapOfShapeListOfShape::new();
        d.append(1, 10);
        d.append(1, 20);
        assert_eq!(d.find(1).unwrap().len(), 2);
    }
}
