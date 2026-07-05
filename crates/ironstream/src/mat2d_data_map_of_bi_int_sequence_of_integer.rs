// FILE: mat2d_data_map_of_bi_int_sequence_of_integer.rs
// occt: MAT2d_DataMapOfBiIntSequenceOfInteger, MAT2d_DataMapIteratorOfDataMapOfBiIntSequenceOfInteger

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<MAT2d_BiInt, TColStd_SequenceOfInteger>.
/// Maintains backward compatibility.
pub struct MAT2dDataMapOfBiIntSequenceOfInteger {
    items: BTreeMap<(i32, i32), Vec<i32>>, // (i1, i2) pair key -> sequence of integers
}

impl MAT2dDataMapOfBiIntSequenceOfInteger {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, i1: i32, i2: i32, value: Vec<i32>) {
        self.items.insert((i1, i2), value);
    }

    pub fn unbind(&mut self, i1: i32, i2: i32) -> bool {
        self.items.remove(&(i1, i2)).is_some()
    }

    pub fn find(&self, i1: i32, i2: i32) -> Option<Vec<i32>> {
        self.items.get(&(i1, i2)).cloned()
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

    pub fn iterator(&self) -> MAT2dDataMapIteratorOfDataMapOfBiIntSequenceOfInteger {
        MAT2dDataMapIteratorOfDataMapOfBiIntSequenceOfInteger {
            items: self.items.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for MAT2dDataMapOfBiIntSequenceOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for MAT2d_DataMapOfBiIntSequenceOfInteger.
pub struct MAT2dDataMapIteratorOfDataMapOfBiIntSequenceOfInteger {
    items: Vec<((i32, i32), Vec<i32>)>,
    index: usize,
}

impl MAT2dDataMapIteratorOfDataMapOfBiIntSequenceOfInteger {
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn key(&self) -> Option<(i32, i32)> {
        if self.index < self.items.len() {
            Some(self.items[self.index].0)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<Vec<i32>> {
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
        let mut map = MAT2dDataMapOfBiIntSequenceOfInteger::new();
        assert!(map.is_empty());

        map.bind(1, 2, vec![10, 20, 30]);
        map.bind(3, 4, vec![40, 50]);
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1, 2), Some(vec![10, 20, 30]));
        assert_eq!(map.find(3, 4), Some(vec![40, 50]));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = MAT2dDataMapOfBiIntSequenceOfInteger::new();
        map.bind(5, 6, vec![100, 200]);
        assert_eq!(map.find(5, 6), Some(vec![100, 200]));

        assert!(map.unbind(5, 6));
        assert_eq!(map.find(5, 6), None);
        assert!(!map.unbind(5, 6));
    }

    #[test]
    fn test_map_clear() {
        let mut map = MAT2dDataMapOfBiIntSequenceOfInteger::new();
        map.bind(1, 1, vec![1]);
        map.bind(2, 2, vec![2]);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = MAT2dDataMapOfBiIntSequenceOfInteger::new();
        map.bind(1, 2, vec![10]);
        map.bind(3, 4, vec![20, 30]);
        map.bind(5, 6, vec![40]);

        let mut iter = map.iterator();
        let mut pairs = Vec::new();
        while iter.more() {
            if let (Some(k), Some(v)) = (iter.key(), iter.value()) {
                pairs.push((k, v));
            }
            iter.next();
        }

        assert_eq!(pairs.len(), 3);
    }

    #[test]
    fn test_default() {
        let map = MAT2dDataMapOfBiIntSequenceOfInteger::default();
        assert!(map.is_empty());
    }
}
