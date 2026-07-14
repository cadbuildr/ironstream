// FILE: topo_list.rs
// occt: TopTools_SequenceOfShape
// occt-ref: TopTools_ListOfShape
//       TopTools_DataMapOfShapeShape, TopTools_DataMapOfShapeInteger,
//       TopTools_MapOfOrientedShape

use crate::brep_tool::{TopoShape, TopAbsShapeEnum, TopAbsOrientation};
use std::collections::HashMap;

/// A list of shapes (ordered, duplicates allowed).
// occt-ref: TopTools_ListOfShape
#[derive(Clone, Debug, Default)]
pub struct TopoListOfShape {
    shapes: Vec<TopoShape>,
}

impl TopoListOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, s: TopoShape) { self.shapes.push(s); }

    pub fn prepend(&mut self, s: TopoShape) { self.shapes.insert(0, s); }

    pub fn append_list(&mut self, other: &Self) {
        for s in &other.shapes { self.shapes.push(s.clone()); }
    }

    pub fn remove_first(&mut self) {
        if !self.shapes.is_empty() { self.shapes.remove(0); }
    }

    pub fn first(&self) -> Option<&TopoShape> { self.shapes.first() }
    pub fn last(&self) -> Option<&TopoShape> { self.shapes.last() }

    pub fn size(&self) -> usize { self.shapes.len() }
    pub fn is_empty(&self) -> bool { self.shapes.is_empty() }

    pub fn clear(&mut self) { self.shapes.clear(); }

    pub fn shapes(&self) -> &[TopoShape] { &self.shapes }

    pub fn iter(&self) -> std::slice::Iter<'_, TopoShape> { self.shapes.iter() }

    /// Remove the first occurrence with the given shape_id.
    pub fn remove(&mut self, shape_id: u32) -> bool {
        if let Some(pos) = self.shapes.iter().position(|s| s.shape_id == shape_id) {
            self.shapes.remove(pos);
            true
        } else {
            false
        }
    }
}

/// A sequence of shapes (1-based random access).
/// occt: TopTools_SequenceOfShape
#[derive(Clone, Debug, Default)]
pub struct TopoSeqOfShape {
    shapes: Vec<TopoShape>,
}

impl TopoSeqOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, s: TopoShape) { self.shapes.push(s); }
    pub fn prepend(&mut self, s: TopoShape) { self.shapes.insert(0, s); }

    pub fn value(&self, i: usize) -> Option<&TopoShape> {
        if i == 0 { None } else { self.shapes.get(i - 1) }
    }

    pub fn set_value(&mut self, i: usize, s: TopoShape) {
        if i >= 1 && i <= self.shapes.len() { self.shapes[i - 1] = s; }
    }

    pub fn insert_before(&mut self, i: usize, s: TopoShape) {
        if i >= 1 && i <= self.shapes.len() + 1 { self.shapes.insert(i - 1, s); }
    }

    pub fn remove(&mut self, i: usize) {
        if i >= 1 && i <= self.shapes.len() { self.shapes.remove(i - 1); }
    }

    pub fn length(&self) -> usize { self.shapes.len() }
    pub fn is_empty(&self) -> bool { self.shapes.is_empty() }
    pub fn first(&self) -> Option<&TopoShape> { self.shapes.first() }
    pub fn last(&self) -> Option<&TopoShape> { self.shapes.last() }
    pub fn clear(&mut self) { self.shapes.clear(); }

    pub fn shapes(&self) -> &[TopoShape] { &self.shapes }
}

/// Map: Shape → Shape (keyed by shape_id).
// occt-ref: TopTools_DataMapOfShapeShape
#[derive(Clone, Debug, Default)]
pub struct DataMapShapeShape {
    map: HashMap<u32, TopoShape>,
}

impl DataMapShapeShape {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key_id: u32, val: TopoShape) {
        self.map.insert(key_id, val);
    }

    pub fn find(&self, key_id: u32) -> Option<&TopoShape> { self.map.get(&key_id) }

    pub fn is_bound(&self, key_id: u32) -> bool { self.map.contains_key(&key_id) }

    pub fn unbind(&mut self, key_id: u32) -> bool { self.map.remove(&key_id).is_some() }

    pub fn extent(&self) -> usize { self.map.len() }

    pub fn is_empty(&self) -> bool { self.map.is_empty() }

    pub fn clear(&mut self) { self.map.clear(); }
}

/// Map: Shape → Integer (keyed by shape_id).
/// occt: TopTools_DataMapOfShapeInteger
#[derive(Clone, Debug, Default)]
pub struct DataMapShapeInt {
    map: HashMap<u32, i32>,
}

impl DataMapShapeInt {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key_id: u32, val: i32) { self.map.insert(key_id, val); }

    pub fn find(&self, key_id: u32) -> Option<i32> { self.map.get(&key_id).copied() }

    pub fn is_bound(&self, key_id: u32) -> bool { self.map.contains_key(&key_id) }

    pub fn unbind(&mut self, key_id: u32) -> bool { self.map.remove(&key_id).is_some() }

    pub fn extent(&self) -> usize { self.map.len() }

    pub fn change_find_or_insert(&mut self, key_id: u32, default: i32) -> i32 {
        *self.map.entry(key_id).or_insert(default)
    }
}

/// Map of oriented shapes (dedup by shape_id + orientation).
/// occt: TopTools_MapOfOrientedShape
#[derive(Clone, Debug, Default)]
pub struct MapOfOrientedShape {
    items: Vec<(u32, TopAbsOrientation)>,
}

impl MapOfOrientedShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, shape_id: u32, orient: TopAbsOrientation) -> bool {
        let key = (shape_id, orient);
        if self.items.contains(&key) { return false; }
        self.items.push(key);
        true
    }

    pub fn contains(&self, shape_id: u32, orient: TopAbsOrientation) -> bool {
        self.items.contains(&(shape_id, orient))
    }

    pub fn extent(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_tool::{TopoShape, TopAbsShapeEnum, TopAbsOrientation};

    fn make_shape(id: u32) -> TopoShape { TopoShape::new(id, TopAbsShapeEnum::Edge) }

    #[test]
    fn list_of_shape_basic() {
        let mut l = TopoListOfShape::new();
        l.append(make_shape(1));
        l.append(make_shape(2));
        l.prepend(make_shape(0));
        assert_eq!(l.size(), 3);
        assert_eq!(l.first().unwrap().shape_id, 0);
        assert_eq!(l.last().unwrap().shape_id, 2);
        l.remove_first();
        assert_eq!(l.size(), 2);
        assert_eq!(l.first().unwrap().shape_id, 1);
    }

    #[test]
    fn list_of_shape_remove_by_id() {
        let mut l = TopoListOfShape::new();
        l.append(make_shape(10));
        l.append(make_shape(20));
        assert!(l.remove(10));
        assert!(!l.remove(99));
        assert_eq!(l.size(), 1);
    }

    #[test]
    fn seq_of_shape_1based() {
        let mut s = TopoSeqOfShape::new();
        s.append(make_shape(1));
        s.append(make_shape(2));
        s.append(make_shape(3));
        assert_eq!(s.length(), 3);
        assert_eq!(s.value(2).unwrap().shape_id, 2);
        assert!(s.value(0).is_none());
        s.remove(2);
        assert_eq!(s.length(), 2);
        assert_eq!(s.value(2).unwrap().shape_id, 3);
    }

    #[test]
    fn data_map_shape_shape() {
        let mut m = DataMapShapeShape::new();
        m.bind(1, make_shape(10));
        m.bind(2, make_shape(20));
        assert!(m.is_bound(1));
        assert_eq!(m.find(1).unwrap().shape_id, 10);
        assert!(!m.is_bound(99));
        assert!(m.unbind(1));
        assert!(!m.is_bound(1));
        assert_eq!(m.extent(), 1);
    }

    #[test]
    fn data_map_shape_int() {
        let mut m = DataMapShapeInt::new();
        m.bind(5, 42);
        assert_eq!(m.find(5), Some(42));
        assert_eq!(m.find(99), None);
        let v = m.change_find_or_insert(5, 0);
        assert_eq!(v, 42);
        let v2 = m.change_find_or_insert(6, 99);
        assert_eq!(v2, 99);
    }

    #[test]
    fn map_of_oriented_shape() {
        let mut m = MapOfOrientedShape::new();
        assert!(m.add(1, TopAbsOrientation::Forward));
        assert!(!m.add(1, TopAbsOrientation::Forward)); // duplicate
        assert!(m.add(1, TopAbsOrientation::Reversed)); // same id, diff orient
        assert_eq!(m.extent(), 2);
        assert!(m.contains(1, TopAbsOrientation::Forward));
        assert!(!m.contains(2, TopAbsOrientation::Forward));
    }
}
