// FILE: t_col_std_indexed_data_map_of_transient_transient.rs
// occt: TColStd_IndexedDataMapOfTransientTransient

use std::collections::HashMap;

/// TColStd_IndexedDataMapOfTransientTransient is a deprecated alias for an indexed data map with String (transient) keys and values.
/// This is a Rust port implementing OCCT's indexed map semantics (1-based indexing by insertion order).
pub struct TColStdIndexedDataMapOfTransientTransient {
    keys: Vec<String>,
    map: HashMap<String, usize>,
}

impl TColStdIndexedDataMapOfTransientTransient {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        TColStdIndexedDataMapOfTransientTransient {
            keys: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Adds a key-value pair to the map, returns the 1-based index.
    pub fn add(&mut self, key: String, value: String) -> i32 {
        if !self.map.contains_key(&key) {
            let idx = self.keys.len();
            self.map.insert(key.clone(), idx);
            self.keys.push(key);
            self.keys.push(value);
            (idx / 2 + 1) as i32
        } else {
            let idx = *self.map.get(&key).unwrap();
            self.keys[idx * 2 + 1] = value;
            (idx + 1) as i32
        }
    }

    /// Gets the 1-based index of a key.
    pub fn index(&self, key: &str) -> Option<i32> {
        self.map.get(key).map(|idx| (*idx + 1) as i32)
    }

    /// Gets a value by 1-based index.
    pub fn at(&self, idx: i32) -> Option<&String> {
        let pos = (idx - 1) as usize;
        if pos < self.keys.len() / 2 {
            Some(&self.keys[pos * 2 + 1])
        } else {
            None
        }
    }

    /// Returns the number of elements in the map.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.keys.clear();
        self.map.clear();
    }
}

impl Default for TColStdIndexedDataMapOfTransientTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_index() {
        let mut map = TColStdIndexedDataMapOfTransientTransient::new();
        let idx1 = map.add("obj1".to_string(), "val1".to_string());
        let idx2 = map.add("obj2".to_string(), "val2".to_string());

        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(map.index("obj1"), Some(1));
    }

    #[test]
    fn test_at() {
        let mut map = TColStdIndexedDataMapOfTransientTransient::new();
        map.add("a".to_string(), "x".to_string());
        map.add("b".to_string(), "y".to_string());

        assert_eq!(map.at(1), Some(&"x".to_string()));
        assert_eq!(map.at(2), Some(&"y".to_string()));
    }

    #[test]
    fn test_size() {
        let mut map = TColStdIndexedDataMapOfTransientTransient::new();
        assert_eq!(map.size(), 0);

        map.add("k1".to_string(), "v1".to_string());
        assert_eq!(map.size(), 1);
    }
}
