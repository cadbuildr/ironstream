// FILE: t_col_std_indexed_map_of_transient.rs
// occt: TColStd_IndexedMapOfTransient

use std::collections::HashMap;

/// TColStd_IndexedMapOfTransient is a deprecated alias for an indexed set of transient objects.
/// This is a Rust port implementing OCCT's indexed map semantics (1-based indexing by insertion order).
pub struct TColStdIndexedMapOfTransient {
    data: Vec<String>,
    map: HashMap<String, usize>,
}

impl TColStdIndexedMapOfTransient {
    /// Creates a new empty indexed map.
    pub fn new() -> Self {
        TColStdIndexedMapOfTransient {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Adds a value to the map, returns the 1-based index.
    pub fn add(&mut self, value: String) -> i32 {
        if !self.map.contains_key(&value) {
            let idx = self.data.len();
            self.map.insert(value.clone(), idx);
            self.data.push(value);
            (idx + 1) as i32
        } else {
            (*self.map.get(&value).unwrap() + 1) as i32
        }
    }

    /// Gets the 1-based index of a value.
    pub fn index(&self, value: &str) -> Option<i32> {
        self.map.get(value).map(|idx| (*idx + 1) as i32)
    }

    /// Gets a value by 1-based index.
    pub fn at(&self, idx: i32) -> Option<&String> {
        let pos = (idx - 1) as usize;
        if pos < self.data.len() {
            Some(&self.data[pos])
        } else {
            None
        }
    }

    /// Removes a value by 1-based index.
    pub fn remove(&mut self, idx: i32) -> bool {
        let pos = (idx - 1) as usize;
        if pos < self.data.len() {
            let value = self.data.remove(pos);
            self.map.remove(&value);
            // Update indices for remaining elements
            for i in pos..self.data.len() {
                self.map.insert(self.data[i].clone(), i);
            }
            true
        } else {
            false
        }
    }

    /// Returns the number of elements in the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
        self.map.clear();
    }

    /// Checks if the map contains a value.
    pub fn contains(&self, value: &str) -> bool {
        self.map.contains_key(value)
    }
}

impl Default for TColStdIndexedMapOfTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_index() {
        let mut map = TColStdIndexedMapOfTransient::new();
        let idx1 = map.add("obj1".to_string());
        let idx2 = map.add("obj2".to_string());

        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(map.index("obj1"), Some(1));
        assert_eq!(map.index("obj2"), Some(2));
    }

    #[test]
    fn test_at() {
        let mut map = TColStdIndexedMapOfTransient::new();
        map.add("first".to_string());
        map.add("second".to_string());

        assert_eq!(map.at(1), Some(&"first".to_string()));
        assert_eq!(map.at(2), Some(&"second".to_string()));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdIndexedMapOfTransient::new();
        assert_eq!(map.size(), 0);

        map.add("item".to_string());
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_contains() {
        let mut map = TColStdIndexedMapOfTransient::new();
        map.add("exists".to_string());

        assert!(map.contains("exists"));
        assert!(!map.contains("missing"));
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdIndexedMapOfTransient::new();
        map.add("a".to_string());
        map.add("b".to_string());

        assert!(map.remove(1));
        assert_eq!(map.size(), 1);
        assert_eq!(map.at(1), Some(&"b".to_string()));
    }
}
