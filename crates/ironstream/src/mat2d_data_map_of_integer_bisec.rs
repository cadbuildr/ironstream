// FILE: mat2d_data_map_of_integer_bisec.rs
// occt: MAT2d_DataMapOfIntegerBisec
// occt-ref: MAT2d_DataMapIteratorOfDataMapOfIntegerBisec

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<int, Bisector_Bisec>.
/// Maintains backward compatibility.
pub struct MAT2dDataMapOfIntegerBisec {
    items: BTreeMap<i32, u32>, // integer key -> Bisector_Bisec value (opaque)
}

impl MAT2dDataMapOfIntegerBisec {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, value: u32) {
        self.items.insert(key, value);
    }

    pub fn unbind(&mut self, key: i32) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn find(&self, key: i32) -> Option<u32> {
        self.items.get(&key).copied()
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

    pub fn iterator(&self) -> MAT2dDataMapIteratorOfDataMapOfIntegerBisec {
        MAT2dDataMapIteratorOfDataMapOfIntegerBisec {
            items: self.items.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for MAT2dDataMapOfIntegerBisec {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for MAT2d_DataMapOfIntegerBisec.
pub struct MAT2dDataMapIteratorOfDataMapOfIntegerBisec {
    items: Vec<(i32, u32)>,
    index: usize,
}

impl MAT2dDataMapIteratorOfDataMapOfIntegerBisec {
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

    pub fn value(&self) -> Option<u32> {
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
        let mut map = MAT2dDataMapOfIntegerBisec::new();
        assert!(map.is_empty());

        map.bind(1, 100);
        map.bind(2, 200);
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1), Some(100));
        assert_eq!(map.find(2), Some(200));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = MAT2dDataMapOfIntegerBisec::new();
        map.bind(42, 999);
        assert_eq!(map.find(42), Some(999));

        assert!(map.unbind(42));
        assert_eq!(map.find(42), None);
        assert!(!map.unbind(42));
    }

    #[test]
    fn test_map_clear() {
        let mut map = MAT2dDataMapOfIntegerBisec::new();
        map.bind(1, 10);
        map.bind(2, 20);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = MAT2dDataMapOfIntegerBisec::new();
        map.bind(1, 10);
        map.bind(2, 20);
        map.bind(3, 30);

        let mut iter = map.iterator();
        let mut pairs = Vec::new();
        while iter.more() {
            if let (Some(k), Some(v)) = (iter.key(), iter.value()) {
                pairs.push((k, v));
            }
            iter.next();
        }

        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&(1, 10)));
        assert!(pairs.contains(&(2, 20)));
        assert!(pairs.contains(&(3, 30)));
    }

    #[test]
    fn test_default() {
        let map = MAT2dDataMapOfIntegerBisec::default();
        assert!(map.is_empty());
    }
}
