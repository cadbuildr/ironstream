// FILE: mat2d_data_map_of_integer_sequence_of_connexion.rs
// occt: MAT2d_DataMapOfIntegerSequenceOfConnexion
// occt-ref: MAT2d_DataMapIteratorOfDataMapOfIntegerSequenceOfConnexion

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<int, MAT2d_SequenceOfConnexion>.
/// Maintains backward compatibility.
pub struct MAT2dDataMapOfIntegerSequenceOfConnexion {
    items: BTreeMap<i32, Vec<u32>>, // integer key -> sequence of Connexion handles (opaque)
}

impl MAT2dDataMapOfIntegerSequenceOfConnexion {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, value: Vec<u32>) {
        self.items.insert(key, value);
    }

    pub fn unbind(&mut self, key: i32) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn find(&self, key: i32) -> Option<Vec<u32>> {
        self.items.get(&key).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn iterator(&self) -> MAT2dDataMapIteratorOfDataMapOfIntegerSequenceOfConnexion {
        MAT2dDataMapIteratorOfDataMapOfIntegerSequenceOfConnexion {
            items: self.items.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for MAT2dDataMapOfIntegerSequenceOfConnexion {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for MAT2d_DataMapOfIntegerSequenceOfConnexion.
pub struct MAT2dDataMapIteratorOfDataMapOfIntegerSequenceOfConnexion {
    items: Vec<(i32, Vec<u32>)>,
    index: usize,
}

impl MAT2dDataMapIteratorOfDataMapOfIntegerSequenceOfConnexion {
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn key(&self) -> Option<i32> {
        if self.index < self.items.len() {
            Some(self.items[self.index].0)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<Vec<u32>> {
        if self.index < self.items.len() {
            Some(self.items[self.index].1.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_bind_and_find() {
        let mut map = MAT2dDataMapOfIntegerSequenceOfConnexion::new();
        assert!(map.is_empty());

        map.bind(1, vec![10, 20, 30]);
        map.bind(2, vec![40, 50]);
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1), Some(vec![10, 20, 30]));
        assert_eq!(map.find(2), Some(vec![40, 50]));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = MAT2dDataMapOfIntegerSequenceOfConnexion::new();
        map.bind(42, vec![100, 200]);
        assert_eq!(map.find(42), Some(vec![100, 200]));

        assert!(map.unbind(42));
        assert_eq!(map.find(42), None);
        assert!(!map.unbind(42));
    }

    #[test]
    fn test_map_clear() {
        let mut map = MAT2dDataMapOfIntegerSequenceOfConnexion::new();
        map.bind(1, vec![1]);
        map.bind(2, vec![2]);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = MAT2dDataMapOfIntegerSequenceOfConnexion::new();
        map.bind(1, vec![10]);
        map.bind(2, vec![20, 30]);

        let mut iter = map.iterator();
        let mut count = 0;
        while iter.more() {
            if let (Some(_k), Some(_v)) = (iter.key(), iter.value()) {
                count += 1;
            }
            iter.next();
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn test_default() {
        let map = MAT2dDataMapOfIntegerSequenceOfConnexion::default();
        assert!(map.is_empty());
    }
}
