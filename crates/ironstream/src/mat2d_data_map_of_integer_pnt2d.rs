// FILE: mat2d_data_map_of_integer_pnt2d.rs
// occt: MAT2d_DataMapOfIntegerPnt2d, MAT2d_DataMapIteratorOfDataMapOfIntegerPnt2d

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<int, gp_Pnt2d>.
/// Maintains backward compatibility.
pub struct MAT2dDataMapOfIntegerPnt2d {
    items: BTreeMap<i32, (f64, f64)>, // integer key -> gp_Pnt2d (x, y)
}

impl MAT2dDataMapOfIntegerPnt2d {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, value: (f64, f64)) {
        self.items.insert(key, value);
    }

    pub fn unbind(&mut self, key: i32) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn find(&self, key: i32) -> Option<(f64, f64)> {
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

    pub fn iterator(&self) -> MAT2dDataMapIteratorOfDataMapOfIntegerPnt2d {
        MAT2dDataMapIteratorOfDataMapOfIntegerPnt2d {
            items: self.items.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for MAT2dDataMapOfIntegerPnt2d {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for MAT2d_DataMapOfIntegerPnt2d.
pub struct MAT2dDataMapIteratorOfDataMapOfIntegerPnt2d {
    items: Vec<(i32, (f64, f64))>,
    index: usize,
}

impl MAT2dDataMapIteratorOfDataMapOfIntegerPnt2d {
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

    pub fn value(&self) -> Option<(f64, f64)> {
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
        let mut map = MAT2dDataMapOfIntegerPnt2d::new();
        assert!(map.is_empty());

        map.bind(1, (1.0, 2.0));
        map.bind(2, (3.0, 4.0));
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1), Some((1.0, 2.0)));
        assert_eq!(map.find(2), Some((3.0, 4.0)));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = MAT2dDataMapOfIntegerPnt2d::new();
        map.bind(42, (10.0, 20.0));
        assert_eq!(map.find(42), Some((10.0, 20.0)));

        assert!(map.unbind(42));
        assert_eq!(map.find(42), None);
        assert!(!map.unbind(42));
    }

    #[test]
    fn test_map_clear() {
        let mut map = MAT2dDataMapOfIntegerPnt2d::new();
        map.bind(1, (0.0, 0.0));
        map.bind(2, (1.0, 1.0));
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = MAT2dDataMapOfIntegerPnt2d::new();
        map.bind(1, (1.0, 2.0));
        map.bind(2, (3.0, 4.0));
        map.bind(3, (5.0, 6.0));

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
        let map = MAT2dDataMapOfIntegerPnt2d::default();
        assert!(map.is_empty());
    }
}
