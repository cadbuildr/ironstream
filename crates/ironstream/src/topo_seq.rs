// FILE: topo_seq.rs
// occt-ref: TopTools_ListOfShape, TopTools_SequenceOfShape, TopTools_IndexedMapOfShape
//       TopTools_DataMapOfShapeShape, TopTools_MapOfShape

use std::collections::HashMap;

/// Shape handle with a type tag (mirrors TopoDS_Shape identity).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeHandle {
    pub id: u64,
    pub shape_type: u8,  // 0=Vertex 1=Edge 2=Wire 3=Face 4=Shell 5=Solid 6=Compound
}

impl ShapeHandle {
    pub fn new(id: u64, shape_type: u8) -> Self { Self { id, shape_type } }
    pub fn is_null(&self) -> bool { self.id == 0 }
}

// occt-ref: TopTools_ListOfShape
#[derive(Clone, Debug, Default)]
pub struct ListOfShape {
    items: Vec<ShapeHandle>,
}

impl ListOfShape {
    pub fn new() -> Self { Self::default() }
    pub fn append(&mut self, s: ShapeHandle) { self.items.push(s); }
    pub fn prepend(&mut self, s: ShapeHandle) { self.items.insert(0, s); }
    pub fn remove_first(&mut self) { if !self.items.is_empty() { self.items.remove(0); } }
    pub fn size(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn first(&self) -> Option<&ShapeHandle> { self.items.first() }
    pub fn last(&self) -> Option<&ShapeHandle> { self.items.last() }
    pub fn iter(&self) -> impl Iterator<Item = &ShapeHandle> { self.items.iter() }
    pub fn clear(&mut self) { self.items.clear(); }
}

// occt-ref: TopTools_SequenceOfShape
#[derive(Clone, Debug, Default)]
pub struct SequenceOfShape {
    items: Vec<ShapeHandle>,
}

impl SequenceOfShape {
    pub fn new() -> Self { Self::default() }
    pub fn append(&mut self, s: ShapeHandle) { self.items.push(s); }
    pub fn prepend(&mut self, s: ShapeHandle) { self.items.insert(0, s); }
    pub fn length(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn value(&self, index: usize) -> Option<&ShapeHandle> { self.items.get(index) }
    pub fn exchange(&mut self, i: usize, j: usize) { self.items.swap(i, j); }
    pub fn remove(&mut self, index: usize) { if index < self.items.len() { self.items.remove(index); } }
    pub fn iter(&self) -> impl Iterator<Item = &ShapeHandle> { self.items.iter() }
}

// occt-ref: TopTools_MapOfShape
#[derive(Clone, Debug, Default)]
pub struct MapOfShape {
    inner: std::collections::HashSet<ShapeHandle>,
}

impl MapOfShape {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, s: ShapeHandle) -> bool { self.inner.insert(s) }
    pub fn contains(&self, s: &ShapeHandle) -> bool { self.inner.contains(s) }
    pub fn remove(&mut self, s: &ShapeHandle) -> bool { self.inner.remove(s) }
    pub fn size(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn clear(&mut self) { self.inner.clear(); }
}

// occt-ref: TopTools_DataMapOfShapeShape
#[derive(Clone, Debug, Default)]
pub struct DataMapOfShapeShape {
    inner: HashMap<ShapeHandle, ShapeHandle>,
}

impl DataMapOfShapeShape {
    pub fn new() -> Self { Self::default() }
    pub fn bind(&mut self, key: ShapeHandle, val: ShapeHandle) { self.inner.insert(key, val); }
    pub fn find(&self, key: &ShapeHandle) -> Option<&ShapeHandle> { self.inner.get(key) }
    pub fn is_bound(&self, key: &ShapeHandle) -> bool { self.inner.contains_key(key) }
    pub fn unbind(&mut self, key: &ShapeHandle) { self.inner.remove(key); }
    pub fn size(&self) -> usize { self.inner.len() }
    pub fn iter(&self) -> impl Iterator<Item = (&ShapeHandle, &ShapeHandle)> { self.inner.iter() }
}

// occt-ref: TopTools_IndexedMapOfShape
#[derive(Clone, Debug, Default)]
pub struct IndexedMapOfShape {
    items: Vec<ShapeHandle>,
    index: HashMap<ShapeHandle, usize>,
}

impl IndexedMapOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, s: ShapeHandle) -> usize {
        if let Some(&i) = self.index.get(&s) { return i; }
        let i = self.items.len();
        self.index.insert(s.clone(), i);
        self.items.push(s);
        i
    }

    pub fn find_index(&self, s: &ShapeHandle) -> Option<usize> { self.index.get(s).copied() }
    pub fn find_key(&self, index: usize) -> Option<&ShapeHandle> { self.items.get(index) }
    pub fn extent(&self) -> usize { self.items.len() }
    pub fn contains(&self, s: &ShapeHandle) -> bool { self.index.contains_key(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_shape() {
        let mut l = ListOfShape::new();
        l.append(ShapeHandle::new(1, 3));
        l.append(ShapeHandle::new(2, 3));
        assert_eq!(l.size(), 2);
        l.remove_first();
        assert_eq!(l.size(), 1);
        assert_eq!(l.first().unwrap().id, 2);
    }

    #[test]
    fn sequence_of_shape() {
        let mut s = SequenceOfShape::new();
        s.append(ShapeHandle::new(10, 1));
        s.append(ShapeHandle::new(20, 1));
        assert_eq!(s.length(), 2);
        assert_eq!(s.value(1).unwrap().id, 20);
    }

    #[test]
    fn map_of_shape() {
        let mut m = MapOfShape::new();
        m.add(ShapeHandle::new(1, 0));
        m.add(ShapeHandle::new(1, 0));
        assert_eq!(m.size(), 1);
        assert!(m.contains(&ShapeHandle::new(1, 0)));
    }

    #[test]
    fn indexed_map_of_shape() {
        let mut m = IndexedMapOfShape::new();
        let i0 = m.add(ShapeHandle::new(5, 3));
        let i1 = m.add(ShapeHandle::new(6, 3));
        assert_ne!(i0, i1);
        assert_eq!(m.find_key(i0).unwrap().id, 5);
    }

    #[test]
    fn data_map_of_shape() {
        let mut m = DataMapOfShapeShape::new();
        m.bind(ShapeHandle::new(1,3), ShapeHandle::new(2,3));
        assert!(m.is_bound(&ShapeHandle::new(1,3)));
        assert_eq!(m.find(&ShapeHandle::new(1,3)).unwrap().id, 2);
    }
}
