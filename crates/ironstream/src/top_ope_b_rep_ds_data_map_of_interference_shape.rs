// FILE: top_ope_b_rep_ds_data_map_of_interference_shape.rs
// occt: TopOpeBRepDS_DataMapOfInterferenceShape

use std::collections::HashMap;

/// Interference: Key type for the map.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Interference {
    id: usize,
}

impl Interference {
    pub fn new(id: usize) -> Self {
        Interference { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ShapeSimple: Simplified shape value.
#[derive(Clone, Debug)]
pub struct ShapeSimple {
    id: usize,
}

impl ShapeSimple {
    pub fn new(id: usize) -> Self {
        ShapeSimple { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// DataMapOfInterferenceShape: Maps Interference to Shape.
#[derive(Clone, Debug)]
pub struct DataMapOfInterferenceShape {
    data: HashMap<Interference, ShapeSimple>,
}

impl DataMapOfInterferenceShape {
    pub fn new() -> Self {
        DataMapOfInterferenceShape {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: Interference, shape: ShapeSimple) -> bool {
        self.data.insert(key, shape).is_none()
    }

    pub fn contains(&self, key: &Interference) -> bool {
        self.data.contains_key(key)
    }

    pub fn find(&self, key: &Interference) -> Option<&ShapeSimple> {
        self.data.get(key)
    }

    pub fn find_mut(&mut self, key: &Interference) -> Option<&mut ShapeSimple> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &Interference) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Interference, &ShapeSimple)> {
        self.data.iter()
    }
}

impl Default for DataMapOfInterferenceShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_new() {
        let interf = Interference::new(42);
        assert_eq!(interf.id(), 42);
    }

    #[test]
    fn test_shape_simple_new() {
        let shape = ShapeSimple::new(10);
        assert_eq!(shape.id(), 10);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfInterferenceShape::new();
        let key = Interference::new(5);
        let shape = ShapeSimple::new(50);

        assert!(map.bind(key.clone(), shape));
        assert!(!map.bind(key, ShapeSimple::new(51)));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfInterferenceShape::new();
        let key = Interference::new(3);
        let shape = ShapeSimple::new(30);
        map.bind(key.clone(), shape);

        let found = map.find(&key).unwrap();
        assert_eq!(found.id(), 30);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfInterferenceShape::new();
        let key = Interference::new(7);
        map.bind(key.clone(), ShapeSimple::new(70));

        assert_eq!(map.size(), 1);
        assert!(map.remove(&key));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_clear() {
        let mut map = DataMapOfInterferenceShape::new();
        map.bind(Interference::new(1), ShapeSimple::new(10));
        map.bind(Interference::new(2), ShapeSimple::new(20));
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
