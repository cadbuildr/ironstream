// FILE: b_rep_offset_data_map_of_shape_map_of_shape.rs
// occt: BRepOffset_DataMapOfShapeMapOfShape

use std::collections::HashMap;

pub struct BrepoffsetDataMapOfShapeMapOfShape {
    data: HashMap<usize, HashMap<usize, usize>>,
}

impl BrepoffsetDataMapOfShapeMapOfShape {
    pub fn new() -> Self {
        BrepoffsetDataMapOfShapeMapOfShape {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key_shape: usize, map_key: usize, map_value: usize) {
        self.data.entry(key_shape).or_insert_with(HashMap::new).insert(map_key, map_value);
    }

    pub fn get(&self, key_shape: usize) -> Option<&HashMap<usize, usize>> {
        self.data.get(&key_shape)
    }

    pub fn get_value(&self, key_shape: usize, map_key: usize) -> Option<usize> {
        self.data.get(&key_shape).and_then(|m| m.get(&map_key)).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, key_shape: usize) -> Option<HashMap<usize, usize>> {
        self.data.remove(&key_shape)
    }
}

impl Default for BrepoffsetDataMapOfShapeMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = BrepoffsetDataMapOfShapeMapOfShape::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_add_and_get() {
        let mut map = BrepoffsetDataMapOfShapeMapOfShape::new();
        map.add(1, 10, 20);
        assert_eq!(map.get_value(1, 10), Some(20));
    }

    #[test]
    fn test_get_value() {
        let mut map = BrepoffsetDataMapOfShapeMapOfShape::new();
        map.add(1, 5, 15);
        assert_eq!(map.get_value(1, 5), Some(15));
        assert_eq!(map.get_value(1, 6), None);
    }
}
