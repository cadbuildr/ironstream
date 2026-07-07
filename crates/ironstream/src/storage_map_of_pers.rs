// FILE: storage_map_of_pers.rs
// occt: Storage_MapOfPers

use std::collections::HashMap;

/// Storage_MapOfPers: a map from String keys to Storage_Persistent handle values.
///
/// This is a deprecated OCCT typedef for backward compatibility.
#[derive(Debug, Clone)]
pub struct Storage_MapOfPers {
    inner: HashMap<String, u64>,
}

impl Storage_MapOfPers {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: u64) -> Option<u64> {
        self.inner.insert(key, value)
    }

    pub fn find(&self, key: &str) -> Option<u64> {
        self.inner.get(key).copied()
    }

    pub fn remove(&mut self, key: &str) -> Option<u64> {
        self.inner.remove(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Default for Storage_MapOfPers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_insert_and_find() {
        let mut map = Storage_MapOfPers::new();
        map.insert("pers1".to_string(), 200);

        assert_eq!(map.find("pers1"), Some(200));
    }

    #[test]
    fn test_map_remove() {
        let mut map = Storage_MapOfPers::new();
        map.insert("pers1".to_string(), 200);

        let removed = map.remove("pers1");
        assert_eq!(removed, Some(200));
        assert_eq!(map.find("pers1"), None);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map = Storage_MapOfPers::new();
        map.insert("obj1".to_string(), 10);
        map.insert("obj2".to_string(), 20);
        map.insert("obj3".to_string(), 30);

        assert_eq!(map.len(), 3);
        assert_eq!(map.find("obj2"), Some(20));
    }

    #[test]
    fn test_map_clear() {
        let mut map = Storage_MapOfPers::new();
        map.insert("key".to_string(), 55);
        assert_eq!(map.len(), 1);

        map.clear();
        assert!(map.is_empty());
    }
}
