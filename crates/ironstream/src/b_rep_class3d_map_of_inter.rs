// FILE: b_rep_class3d_map_of_inter.rs
// occt: BRepClass3d_MapOfInter

use std::collections::HashMap;

/// Intersection data for 3D classification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inter {
    edge_id: usize,
    param: f64,
    state: i32,
}

impl Inter {
    pub fn new(edge_id: usize, param: f64, state: i32) -> Self {
        Inter { edge_id, param, state }
    }

    pub fn edge_id(&self) -> usize {
        self.edge_id
    }

    pub fn param(&self) -> f64 {
        self.param
    }

    pub fn state(&self) -> i32 {
        self.state
    }
}

impl std::hash::Hash for Inter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edge_id.hash(state);
    }
}

impl Eq for Inter {}

/// Map for 3D intersection data.
pub struct BrepClass3dMapOfInter {
    data: HashMap<u64, Inter>,
}

impl BrepClass3dMapOfInter {
    pub fn new() -> Self {
        BrepClass3dMapOfInter {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, inter: Inter) -> bool {
        let key = inter.edge_id as u64;
        self.data.insert(key, inter).is_none()
    }

    pub fn get(&self, edge_id: usize) -> Option<&Inter> {
        self.data.get(&(edge_id as u64))
    }

    pub fn contains(&self, edge_id: usize) -> bool {
        self.data.contains_key(&(edge_id as u64))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn remove(&mut self, edge_id: usize) -> Option<Inter> {
        self.data.remove(&(edge_id as u64))
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Inter> {
        self.data.values()
    }
}

impl Default for BrepClass3dMapOfInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inter_creation() {
        let inter = Inter::new(1, 0.5, 1);
        assert_eq!(inter.edge_id(), 1);
        assert_eq!(inter.param(), 0.5);
        assert_eq!(inter.state(), 1);
    }

    #[test]
    fn test_map_add() {
        let mut map = BrepClass3dMapOfInter::new();
        let inter = Inter::new(1, 0.5, 1);
        assert!(map.add(inter));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_get() {
        let mut map = BrepClass3dMapOfInter::new();
        let inter = Inter::new(42, 0.75, 2);
        map.add(inter);
        assert_eq!(map.get(42).unwrap().param(), 0.75);
    }

    #[test]
    fn test_map_contains() {
        let mut map = BrepClass3dMapOfInter::new();
        let inter = Inter::new(5, 0.5, 1);
        map.add(inter);
        assert!(map.contains(5));
        assert!(!map.contains(10));
    }

    #[test]
    fn test_map_remove() {
        let mut map = BrepClass3dMapOfInter::new();
        let inter = Inter::new(1, 0.5, 1);
        map.add(inter);
        assert_eq!(map.len(), 1);
        map.remove(1);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = BrepClass3dMapOfInter::new();
        map.add(Inter::new(1, 0.5, 1));
        map.add(Inter::new(2, 0.6, 1));
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iter() {
        let mut map = BrepClass3dMapOfInter::new();
        for i in 1..=3 {
            map.add(Inter::new(i, i as f64 * 0.1, 1));
        }
        assert_eq!(map.iter().count(), 3);
    }
}
