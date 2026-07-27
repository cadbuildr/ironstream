// FILE: mat2d_data_map_of_bi_int_integer.rs
// occt: MAT2d_DataMapOfBiIntInteger
// occt-ref: MAT2d_DataMapIteratorOfDataMapOfBiIntInteger

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<MAT2d_BiInt, int>.
/// Maintains backward compatibility.
pub struct MAT2dDataMapOfBiIntInteger {
    items: BTreeMap<(i32, i32), i32>, // (i1, i2) pair key -> integer value
}

impl MAT2dDataMapOfBiIntInteger {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, i1: i32, i2: i32, value: i32) {
        self.items.insert((i1, i2), value);
    }

    pub fn unbind(&mut self, i1: i32, i2: i32) -> bool {
        self.items.remove(&(i1, i2)).is_some()
    }

    pub fn find(&self, i1: i32, i2: i32) -> Option<i32> {
        self.items.get(&(i1, i2)).copied()
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

    pub fn iterator(&self) -> MAT2dDataMapIteratorOfDataMapOfBiIntInteger {
        MAT2dDataMapIteratorOfDataMapOfBiIntInteger {
            items: self.items.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for MAT2dDataMapOfBiIntInteger {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for MAT2d_DataMapOfBiIntInteger.
pub struct MAT2dDataMapIteratorOfDataMapOfBiIntInteger {
    items: Vec<((i32, i32), i32)>,
    index: usize,
}

impl MAT2dDataMapIteratorOfDataMapOfBiIntInteger {
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

    pub fn value(&self) -> Option<i32> {
        if self.index < self.items.len() {
            Some(self.items[self.index].1)
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
        let mut map = MAT2dDataMapOfBiIntInteger::new();
        assert!(map.is_empty());

        map.bind(1, 2, 100);
        map.bind(3, 4, 200);
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1, 2), Some(100));
        assert_eq!(map.find(3, 4), Some(200));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = MAT2dDataMapOfBiIntInteger::new();
        map.bind(5, 6, 999);
        assert_eq!(map.find(5, 6), Some(999));

        assert!(map.unbind(5, 6));
        assert_eq!(map.find(5, 6), None);
        assert!(!map.unbind(5, 6));
    }

    #[test]
    fn test_map_clear() {
        let mut map = MAT2dDataMapOfBiIntInteger::new();
        map.bind(1, 1, 10);
        map.bind(2, 2, 20);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = MAT2dDataMapOfBiIntInteger::new();
        map.bind(1, 2, 10);
        map.bind(3, 4, 20);
        map.bind(5, 6, 30);

        let mut iter = map.iterator();
        let mut pairs = Vec::new();
        while iter.more() {
            if let (Some(k), Some(v)) = (iter.key(), iter.value()) {
                pairs.push((k, v));
            }
            iter.next();
        }

        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&((1, 2), 10)));
        assert!(pairs.contains(&((3, 4), 20)));
        assert!(pairs.contains(&((5, 6), 30)));
    }

    #[test]
    fn test_default() {
        let map = MAT2dDataMapOfBiIntInteger::default();
        assert!(map.is_empty());
    }
}
