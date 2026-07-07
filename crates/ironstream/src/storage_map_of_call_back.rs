// FILE: storage_map_of_call_back.rs
// occt: Storage_MapOfCallBack

use std::collections::HashMap;

/// Storage_MapOfCallBack: a map from String keys to Storage_CallBack handle values.
///
/// This is a deprecated OCCT typedef for backward compatibility.
#[derive(Debug, Clone)]
pub struct Storage_MapOfCallBack {
    inner: HashMap<String, u64>,
}

impl Storage_MapOfCallBack {
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

impl Default for Storage_MapOfCallBack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_insert_and_find() {
        let mut map = Storage_MapOfCallBack::new();
        map.insert("key1".to_string(), 100);

        assert_eq!(map.find("key1"), Some(100));
    }

    #[test]
    fn test_map_remove() {
        let mut map = Storage_MapOfCallBack::new();
        map.insert("key1".to_string(), 100);

        let removed = map.remove("key1");
        assert_eq!(removed, Some(100));
        assert_eq!(map.find("key1"), None);
    }

    #[test]
    fn test_map_contains() {
        let mut map = Storage_MapOfCallBack::new();
        map.insert("test".to_string(), 42);

        assert!(map.contains("test"));
        assert!(!map.contains("missing"));
    }

    #[test]
    fn test_map_len() {
        let mut map = Storage_MapOfCallBack::new();
        assert_eq!(map.len(), 0);

        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_map_clear() {
        let mut map = Storage_MapOfCallBack::new();
        map.insert("key".to_string(), 99);
        assert!(!map.is_empty());

        map.clear();
        assert!(map.is_empty());
    }
}
