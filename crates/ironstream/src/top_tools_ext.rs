// FILE: top_tools_ext.rs
// occt-ref: TopTools_DataMapOfShapeShape, TopTools_ListOfShape, TopTools_SequenceOfShape

use std::collections::HashMap;

// occt-ref: TopTools_ListOfShape // — ordered list of shape IDs (Vec-backed)
#[derive(Clone, Debug, Default)]
pub struct ListOfShape {
    data: Vec<u32>,
}

impl ListOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, shape_id: u32) { self.data.push(shape_id); }
    pub fn prepend(&mut self, shape_id: u32) { self.data.insert(0, shape_id); }
    pub fn first(&self) -> Option<u32> { self.data.first().copied() }
    pub fn last(&self) -> Option<u32> { self.data.last().copied() }
    pub fn size(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }
    pub fn contains(&self, shape_id: u32) -> bool { self.data.contains(&shape_id) }
    pub fn remove_first(&mut self) { if !self.data.is_empty() { self.data.remove(0); } }
    pub fn as_slice(&self) -> &[u32] { &self.data }

    pub fn append_list(&mut self, other: &ListOfShape) {
        for &id in &other.data { self.data.push(id); }
    }
}

// occt-ref: TopTools_SequenceOfShape // — indexed sequence of shape IDs (1-based)
#[derive(Clone, Debug, Default)]
pub struct SequenceOfShape {
    data: Vec<u32>,
}

impl SequenceOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, shape_id: u32) { self.data.push(shape_id); }
    pub fn prepend(&mut self, shape_id: u32) { self.data.insert(0, shape_id); }
    pub fn length(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }

    /// 1-based access.
    pub fn value(&self, i: usize) -> Option<u32> {
        if i == 0 { return None; }
        self.data.get(i - 1).copied()
    }

    /// 1-based set.
    pub fn set_value(&mut self, i: usize, shape_id: u32) {
        if i > 0 { if let Some(x) = self.data.get_mut(i - 1) { *x = shape_id; } }
    }

    pub fn first(&self) -> Option<u32> { self.data.first().copied() }
    pub fn last(&self) -> Option<u32> { self.data.last().copied() }

    pub fn remove(&mut self, i: usize) {
        if i > 0 && i <= self.data.len() { self.data.remove(i - 1); }
    }
}

// occt-ref: TopTools_DataMapOfShapeShape // — maps shape IDs to shape IDs
#[derive(Clone, Debug, Default)]
pub struct DataMapOfShapeShape {
    map: HashMap<u32, u32>,
}

impl DataMapOfShapeShape {
    pub fn new() -> Self { Self::default() }
    pub fn bind(&mut self, key: u32, value: u32) { self.map.insert(key, value); }
    pub fn find(&self, key: u32) -> Option<u32> { self.map.get(&key).copied() }
    pub fn is_bound(&self, key: u32) -> bool { self.map.contains_key(&key) }
    pub fn unbind(&mut self, key: u32) -> bool { self.map.remove(&key).is_some() }
    pub fn size(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn clear(&mut self) { self.map.clear(); }
    pub fn keys(&self) -> Vec<u32> { self.map.keys().copied().collect() }
    pub fn values(&self) -> Vec<u32> { self.map.values().copied().collect() }
}

// occt: TopTools_DataMapOfShapeListOfShape // — maps shape IDs to lists of shape IDs
#[derive(Clone, Debug, Default)]
pub struct DataMapOfShapeListOfShape {
    map: HashMap<u32, Vec<u32>>,
}

impl DataMapOfShapeListOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key: u32, values: Vec<u32>) { self.map.insert(key, values); }
    pub fn find(&self, key: u32) -> Option<&[u32]> { self.map.get(&key).map(|v| v.as_slice()) }
    pub fn is_bound(&self, key: u32) -> bool { self.map.contains_key(&key) }
    pub fn unbind(&mut self, key: u32) -> bool { self.map.remove(&key).is_some() }
    pub fn size(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn clear(&mut self) { self.map.clear(); }

    pub fn add_to(&mut self, key: u32, shape_id: u32) {
        self.map.entry(key).or_default().push(shape_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_shape_basics() {
        let mut l = ListOfShape::new();
        l.append(1);
        l.append(2);
        l.prepend(0);
        assert_eq!(l.first(), Some(0));
        assert_eq!(l.size(), 3);
        assert!(l.contains(2));
        l.remove_first();
        assert_eq!(l.first(), Some(1));
    }

    #[test]
    fn list_of_shape_append_list() {
        let mut l1 = ListOfShape::new();
        l1.append(10);
        let mut l2 = ListOfShape::new();
        l2.append(20);
        l2.append(30);
        l1.append_list(&l2);
        assert_eq!(l1.size(), 3);
        assert_eq!(l1.last(), Some(30));
    }

    #[test]
    fn sequence_of_shape_1based() {
        let mut s = SequenceOfShape::new();
        s.append(100);
        s.append(200);
        s.append(300);
        assert_eq!(s.value(1), Some(100));
        assert_eq!(s.value(3), Some(300));
        assert_eq!(s.value(0), None);
        s.set_value(2, 999);
        assert_eq!(s.value(2), Some(999));
    }

    #[test]
    fn sequence_of_shape_remove() {
        let mut s = SequenceOfShape::new();
        s.append(1); s.append(2); s.append(3);
        s.remove(2);
        assert_eq!(s.length(), 2);
        assert_eq!(s.value(2), Some(3));
    }

    #[test]
    fn data_map_of_shape_shape() {
        let mut m = DataMapOfShapeShape::new();
        m.bind(1, 100);
        m.bind(2, 200);
        assert_eq!(m.find(1), Some(100));
        assert!(m.is_bound(2));
        m.unbind(1);
        assert!(!m.is_bound(1));
        assert_eq!(m.size(), 1);
    }

    #[test]
    fn data_map_of_shape_list() {
        let mut m = DataMapOfShapeListOfShape::new();
        m.add_to(1, 10);
        m.add_to(1, 20);
        m.bind(2, vec![30, 40]);
        assert_eq!(m.find(1).unwrap().len(), 2);
        assert_eq!(m.find(2).unwrap(), &[30, 40]);
        assert_eq!(m.size(), 2);
    }
}
