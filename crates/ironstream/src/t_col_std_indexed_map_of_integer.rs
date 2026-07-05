// FILE: t_col_std_indexed_map_of_integer.rs
// occt: TColStd_IndexedMapOfInteger

use std::collections::HashMap;

/// TColStd_IndexedMapOfInteger is a deprecated alias for an indexed set of integers.
/// This is a Rust port implementing OCCT's indexed map semantics (1-based indexing by insertion order).
pub struct TColStdIndexedMapOfInteger {
    data: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl TColStdIndexedMapOfInteger {
    /// Creates a new empty indexed map.
    pub fn new() -> Self {
        TColStdIndexedMapOfInteger {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Adds a value to the map, returns the 1-based index.
    pub fn add(&mut self, value: i32) -> i32 {
        if !self.map.contains_key(&value) {
            let idx = self.data.len();
            self.map.insert(value, idx);
            self.data.push(value);
            (idx + 1) as i32
        } else {
            (*self.map.get(&value).unwrap() + 1) as i32
        }
    }

    /// Gets the 1-based index of a value.
    pub fn index(&self, value: i32) -> Option<i32> {
        self.map.get(&value).map(|idx| (*idx + 1) as i32)
    }

    /// Gets a value by 1-based index.
    pub fn at(&self, idx: i32) -> Option<&i32> {
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
                self.map.insert(self.data[i], i);
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
    pub fn contains(&self, value: i32) -> bool {
        self.map.contains_key(&value)
    }
}

impl Default for TColStdIndexedMapOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_index() {
        let mut map = TColStdIndexedMapOfInteger::new();
        let idx1 = map.add(10);
        let idx2 = map.add(20);

        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(map.index(10), Some(1));
        assert_eq!(map.index(20), Some(2));
    }

    #[test]
    fn test_at() {
        let mut map = TColStdIndexedMapOfInteger::new();
        map.add(100);
        map.add(200);

        assert_eq!(map.at(1), Some(&100));
        assert_eq!(map.at(2), Some(&200));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdIndexedMapOfInteger::new();
        assert_eq!(map.size(), 0);

        map.add(5);
        assert_eq!(map.size(), 1);

        map.add(10);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_contains() {
        let mut map = TColStdIndexedMapOfInteger::new();
        map.add(42);

        assert!(map.contains(42));
        assert!(!map.contains(99));
    }

    #[test]
    fn test_remove() {
        let mut map = TColStdIndexedMapOfInteger::new();
        map.add(10);
        map.add(20);

        assert!(map.remove(1));
        assert_eq!(map.size(), 1);
        assert_eq!(map.at(1), Some(&20));
    }
}
