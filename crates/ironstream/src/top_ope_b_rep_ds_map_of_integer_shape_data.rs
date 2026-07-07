// FILE: top_ope_b_rep_ds_map_of_integer_shape_data.rs
// occt: TopOpeBRepDS_MapOfIntegerShapeData, TopOpeBRepDS_ShapeData

use std::collections::HashMap;

/// ShapeData: Data associated with a shape.
#[derive(Clone, Debug)]
pub struct ShapeData {
    id: usize,
}

impl ShapeData {
    pub fn new(id: usize) -> Self {
        ShapeData { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// MapOfIntegerShapeData: Maps integer to ShapeData.
#[derive(Clone, Debug)]
pub struct MapOfIntegerShapeData {
    data: HashMap<i32, ShapeData>,
}

impl MapOfIntegerShapeData {
    pub fn new() -> Self {
        MapOfIntegerShapeData {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, data: ShapeData) -> bool {
        self.data.insert(key, data).is_none()
    }

    pub fn contains(&self, key: i32) -> bool {
        self.data.contains_key(&key)
    }

    pub fn find(&self, key: i32) -> Option<&ShapeData> {
        self.data.get(&key)
    }

    pub fn remove(&mut self, key: i32) -> bool {
        self.data.remove(&key).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (i32, &ShapeData)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }
}

impl Default for MapOfIntegerShapeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_data() {
        let data = ShapeData::new(42);
        assert_eq!(data.id(), 42);
    }

    #[test]
    fn test_map_bind() {
        let mut map = MapOfIntegerShapeData::new();
        assert!(map.bind(5, ShapeData::new(50)));
        assert!(!map.bind(5, ShapeData::new(51)));
    }

    #[test]
    fn test_map_contains() {
        let mut map = MapOfIntegerShapeData::new();
        assert!(!map.contains(5));
        map.bind(5, ShapeData::new(50));
        assert!(map.contains(5));
    }

    #[test]
    fn test_map_find() {
        let mut map = MapOfIntegerShapeData::new();
        map.bind(3, ShapeData::new(30));
        let found = map.find(3).unwrap();
        assert_eq!(found.id(), 30);
    }

    #[test]
    fn test_map_remove() {
        let mut map = MapOfIntegerShapeData::new();
        map.bind(7, ShapeData::new(70));
        assert_eq!(map.size(), 1);
        assert!(map.remove(7));
        assert_eq!(map.size(), 0);
    }
}
