// FILE: geom2d_hatch_map_of_elements.rs
// occt: Geom2dHatch_MapOfElements

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Element {}

#[derive(Clone, Debug)]
pub struct MapOfElements {
    map: HashMap<usize, Element>,
}

impl MapOfElements {
    pub fn new() -> Self { MapOfElements { map: HashMap::new() } }
    pub fn insert(&mut self, key: usize, val: Element) { self.map.insert(key, val); }
    pub fn get(&self, key: usize) -> Option<&Element> { self.map.get(&key) }
    pub fn len(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
}

impl Default for MapOfElements {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_creation() {
        let map = MapOfElements::new();
        assert!(map.is_empty());
    }
    #[test]
    fn test_map_insert() {
        let mut map = MapOfElements::new();
        map.insert(1, Element {});
        assert_eq!(map.len(), 1);
    }
}
