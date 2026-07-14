// FILE: interface_data_map_of_transient_integer.rs
// occt: Interface_DataMapOfTransientInteger
// occt-ref: Interface_DataMapIteratorOfDataMapOfTransientInteger

use std::collections::BTreeMap;

/// Deprecated alias for NCollection_DataMap<opencascade::handle<Standard_Transient>, int>.
/// Maintains backward compatibility.
pub struct InterfaceDataMapOfTransientInteger {
    items: BTreeMap<u64, i32>, // u64 placeholder for transient handle, i32 for integer value
}

impl InterfaceDataMapOfTransientInteger {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: u64, value: i32) {
        self.items.insert(key, value);
    }

    pub fn unbind(&mut self, key: u64) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn find(&self, key: u64) -> Option<i32> {
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

    pub fn iterator(&self) -> InterfaceDataMapIteratorOfDataMapOfTransientInteger {
        InterfaceDataMapIteratorOfDataMapOfTransientInteger {
            items: self.items.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for InterfaceDataMapOfTransientInteger {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for Interface_DataMapOfTransientInteger.
pub struct InterfaceDataMapIteratorOfDataMapOfTransientInteger {
    items: Vec<(u64, i32)>,
    index: usize,
}

impl InterfaceDataMapIteratorOfDataMapOfTransientInteger {
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
        let mut map = InterfaceDataMapOfTransientInteger::new();
        assert!(map.is_empty());

        map.bind(1, 100);
        map.bind(2, 200);
        assert_eq!(map.len(), 2);
        assert_eq!(map.find(1), Some(100));
        assert_eq!(map.find(2), Some(200));
    }

    #[test]
    fn test_map_unbind() {
        let mut map = InterfaceDataMapOfTransientInteger::new();
        map.bind(42, 999);
        assert_eq!(map.find(42), Some(999));

        assert!(map.unbind(42));
        assert_eq!(map.find(42), None);
        assert!(!map.unbind(42));
    }

    #[test]
    fn test_map_clear() {
        let mut map = InterfaceDataMapOfTransientInteger::new();
        map.bind(1, 10);
        map.bind(2, 20);
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = InterfaceDataMapOfTransientInteger::new();
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
        let map = InterfaceDataMapOfTransientInteger::default();
        assert!(map.is_empty());
    }
}
