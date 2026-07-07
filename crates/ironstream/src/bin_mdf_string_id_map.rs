// FILE: bin_mdf_string_id_map.rs
// occt: BinMDF_StringIdMap

use std::collections::HashMap;

pub struct BinmdfStringIdMap {
    data: HashMap<String, usize>,
}

impl BinmdfStringIdMap {
    pub fn new() -> Self {
        BinmdfStringIdMap {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, id: usize) {
        self.data.insert(key, id);
    }

    pub fn get(&self, key: &str) -> Option<usize> {
        self.data.get(key).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, key: &str) -> Option<usize> {
        self.data.remove(key)
    }
}

impl Default for BinmdfStringIdMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut map = BinmdfStringIdMap::new();
        map.add("test".to_string(), 42);
        assert_eq!(map.get("test"), Some(42));
    }

    #[test]
    fn test_len() {
        let mut map = BinmdfStringIdMap::new();
        map.add("key1".to_string(), 1);
        map.add("key2".to_string(), 2);
        assert_eq!(map.len(), 2);
    }
}
