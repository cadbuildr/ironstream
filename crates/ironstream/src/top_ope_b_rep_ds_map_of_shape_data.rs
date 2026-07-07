// FILE: top_ope_b_rep_ds_map_of_shape_data.rs
// occt: TopOpeBRepDS_MapOfShapeData, TopOpeBRepDS_ShapeData

use std::collections::HashMap;

/// ShapeKey: Simple shape identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ShapeData: Data associated with a shape.
#[derive(Clone, Debug)]
pub struct ShapeData {
    value: usize,
}

impl ShapeData {
    pub fn new(value: usize) -> Self {
        ShapeData { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
}

/// MapOfShapeData: Maps Shape to ShapeData.
#[derive(Clone, Debug)]
pub struct MapOfShapeData {
    data: HashMap<ShapeKey, ShapeData>,
}

impl MapOfShapeData {
    pub fn new() -> Self {
        MapOfShapeData {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, data: ShapeData) -> bool {
        self.data.insert(shape, data).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&ShapeData> {
        self.data.get(shape)
    }

    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut ShapeData> {
        self.data.get_mut(shape)
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &ShapeData)> {
        self.data.iter()
    }
}

impl Default for MapOfShapeData {
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
        assert_eq!(data.value(), 42);
    }

    #[test]
    fn test_map_bind() {
        let mut map = MapOfShapeData::new();
        let shape = ShapeKey::new(5);
        assert!(map.bind(shape.clone(), ShapeData::new(50)));
        assert!(!map.bind(shape, ShapeData::new(51)));
    }

    #[test]
    fn test_map_find() {
        let mut map = MapOfShapeData::new();
        let shape = ShapeKey::new(3);
        map.bind(shape.clone(), ShapeData::new(30));
        let found = map.find(&shape).unwrap();
        assert_eq!(found.value(), 30);
    }

    #[test]
    fn test_map_remove() {
        let mut map = MapOfShapeData::new();
        let shape = ShapeKey::new(7);
        map.bind(shape.clone(), ShapeData::new(70));
        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }
}
