// FILE: loc_ope_data_map_of_shape_pnt.rs
// occt: LocOpe_DataMapOfShapePnt
// occt-ref: LocOpe_DataMapIteratorOfDataMapOfShapePnt

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<TopoDS_Shape, gp_Pnt, TopTools_ShapeMapHasher>.
/// Maintains backward compatibility.
pub struct LocOpeDataMapOfShapePnt {
    items: BTreeMap<u64, (f64, f64, f64)>, // u64 for shape key, (f64, f64, f64) for gp_Pnt coordinates
}

impl LocOpeDataMapOfShapePnt {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: u64, value: (f64, f64, f64)) {
        self.items.insert(key, value);
    }

    pub fn unbind(&mut self, key: u64) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn find(&self, key: u64) -> Option<(f64, f64, f64)> {
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

    pub fn iterator(&self) -> LocOpeDataMapIteratorOfDataMapOfShapePnt {
        LocOpeDataMapIteratorOfDataMapOfShapePnt {
            items: self.items.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for LocOpeDataMapOfShapePnt {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for LocOpe_DataMapOfShapePnt.
pub struct LocOpeDataMapIteratorOfDataMapOfShapePnt {
    items: Vec<(u64, (f64, f64, f64))>,
    index: usize,
}

impl LocOpeDataMapIteratorOfDataMapOfShapePnt {
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn key(&self) -> Option<u64> {
        if self.index < self.items.len() {
            Some(self.items[self.index].0)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<(f64, f64, f64)> {
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
        let mut map = LocOpeDataMapOfShapePnt::new();
        assert!(map.is_empty());

        map.bind(1, (1.0, 2.0, 3.0));
        map.bind(2, (4.0, 5.0, 6.0));
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1), Some((1.0, 2.0, 3.0)));
        assert_eq!(map.find(2), Some((4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = LocOpeDataMapOfShapePnt::new();
        map.bind(42, (10.0, 20.0, 30.0));
        assert_eq!(map.find(42), Some((10.0, 20.0, 30.0)));

        assert!(map.unbind(42));
        assert_eq!(map.find(42), None);
        assert!(!map.unbind(42));
    }

    #[test]
    fn test_map_clear() {
        let mut map = LocOpeDataMapOfShapePnt::new();
        map.bind(1, (0.0, 0.0, 0.0));
        map.bind(2, (1.0, 1.0, 1.0));
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = LocOpeDataMapOfShapePnt::new();
        map.bind(1, (1.0, 2.0, 3.0));
        map.bind(2, (4.0, 5.0, 6.0));
        map.bind(3, (7.0, 8.0, 9.0));

        let mut iter = map.iterator();
        let mut pairs = Vec::new();
        while iter.more() {
            if let (Some(k), Some(v)) = (iter.key(), iter.value()) {
                pairs.push((k, v));
            }
            iter.next();
        }

        assert_eq!(pairs.len(), 3);
        assert!(pairs.contains(&(1, (1.0, 2.0, 3.0))));
        assert!(pairs.contains(&(2, (4.0, 5.0, 6.0))));
        assert!(pairs.contains(&(3, (7.0, 8.0, 9.0))));
    }

    #[test]
    fn test_default() {
        let map = LocOpeDataMapOfShapePnt::default();
        assert!(map.is_empty());
    }
}
