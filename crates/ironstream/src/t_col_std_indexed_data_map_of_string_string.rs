// FILE: t_col_std_indexed_data_map_of_string_string.rs
// occt: TColStd_IndexedDataMapOfStringString

use std::collections::HashMap;

/// TColStd_IndexedDataMapOfStringString is a deprecated alias for an indexed data map with String keys and values.
/// This is a Rust port implementing OCCT's indexed map semantics (1-based indexing by insertion order).
pub struct TColStdIndexedDataMapOfStringString {
    keys: Vec<String>,
    values: Vec<String>,
    map: HashMap<String, usize>,
}

impl TColStdIndexedDataMapOfStringString {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        TColStdIndexedDataMapOfStringString {
            keys: Vec::new(),
            values: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Adds a key-value pair to the map, returns the 1-based index.
    ///
    /// Per OCCT NCollection_IndexedDataMap::Add semantics: if the key is
    /// already bound, the existing index is returned and the stored item is
    /// left unchanged.
    pub fn add(&mut self, key: String, value: String) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            (idx + 1) as i32
        } else {
            let idx = self.keys.len();
            self.map.insert(key.clone(), idx);
            self.keys.push(key);
            self.values.push(value);
            (idx + 1) as i32
        }
    }

    /// Gets the 1-based index of a key.
    pub fn index(&self, key: &str) -> Option<i32> {
        self.map.get(key).map(|idx| (*idx + 1) as i32)
    }

    /// Gets a value by 1-based index.
    pub fn at(&self, idx: i32) -> Option<&String> {
        if idx < 1 {
            return None;
        }
        self.values.get((idx - 1) as usize)
    }

    /// Returns the number of elements in the map.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
        self.map.clear();
    }
}

impl Default for TColStdIndexedDataMapOfStringString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_index() {
        let mut map = TColStdIndexedDataMapOfStringString::new();
        let idx1 = map.add("key1".to_string(), "val1".to_string());
        let idx2 = map.add("key2".to_string(), "val2".to_string());

        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(map.index("key1"), Some(1));
        assert_eq!(map.index("key2"), Some(2));
    }

    #[test]
    fn test_at() {
        let mut map = TColStdIndexedDataMapOfStringString::new();
        map.add("k1".to_string(), "v1".to_string());
        map.add("k2".to_string(), "v2".to_string());

        assert_eq!(map.at(1), Some(&"v1".to_string()));
        assert_eq!(map.at(2), Some(&"v2".to_string()));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdIndexedDataMapOfStringString::new();
        assert_eq!(map.size(), 0);

        map.add("a".to_string(), "x".to_string());
        assert_eq!(map.size(), 1);

        map.add("b".to_string(), "y".to_string());
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_clear() {
        let mut map = TColStdIndexedDataMapOfStringString::new();
        map.add("k".to_string(), "v".to_string());
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
