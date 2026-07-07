// FILE: graphic3d_map_iterator_of_map_of_structure.rs
// occt: Graphic3d_MapIteratorOfMapOfStructure

//! Deprecated: Use std::collections::HashMap iterator directly.
//! Iterator for map of graphic structures.

use std::collections::HashMap;

pub type StructureId = usize;

pub struct MapIterator {
    indices: Vec<StructureId>,
    current: usize,
}

impl MapIterator {
    pub fn new(map: &HashMap<StructureId, String>) -> Self {
        let indices: Vec<StructureId> = map.keys().copied().collect();
        MapIterator { indices, current: 0 }
    }

    pub fn more(&self) -> bool {
        self.current < self.indices.len()
    }

    pub fn next(&mut self) {
        if self.more() {
            self.current += 1;
        }
    }

    pub fn key(&self) -> Option<StructureId> {
        if self.more() {
            Some(self.indices[self.current])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_creation() {
        let mut map = HashMap::new();
        map.insert(1, "struct1".to_string());
        map.insert(2, "struct2".to_string());

        let iter = MapIterator::new(&map);
        assert!(iter.more());
    }

    #[test]
    fn test_iterator_traversal() {
        let mut map = HashMap::new();
        map.insert(10, "a".to_string());
        map.insert(20, "b".to_string());
        map.insert(30, "c".to_string());

        let mut iter = MapIterator::new(&map);
        let mut count = 0;

        while iter.more() {
            assert!(iter.key().is_some());
            iter.next();
            count += 1;
        }

        assert_eq!(count, 3);
    }

    #[test]
    fn test_iterator_empty() {
        let map: HashMap<StructureId, String> = HashMap::new();
        let iter = MapIterator::new(&map);
        assert!(!iter.more());
    }
}
