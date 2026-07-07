// FILE: std_storage_map_of_types.rs
// occt: StdStorage_MapOfTypes

use std::collections::BTreeMap;

pub struct StdStorageMapOfTypes {
    data: BTreeMap<String, String>,
}

impl StdStorageMapOfTypes {
    pub fn new() -> Self {
        StdStorageMapOfTypes {
            data: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

impl Default for StdStorageMapOfTypes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = StdStorageMapOfTypes::new();
        map.bind("type1".to_string(), "data".to_string());
        assert!(map.contains("type1"));
    }
}
