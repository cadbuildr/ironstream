// FILE: t_col_std_indexed_map_of_real.rs
// occt: TColStd_IndexedMapOfReal

use std::collections::HashMap;

/// TColStd_IndexedMapOfReal is a deprecated alias for an indexed set of real numbers.
/// This is a Rust port implementing OCCT's indexed map semantics (1-based indexing by insertion order).
pub struct TColStdIndexedMapOfReal {
    data: Vec<f64>,
    map: HashMap<u64, usize>,
}

impl TColStdIndexedMapOfReal {
    /// Creates a new empty indexed map.
    pub fn new() -> Self {
        TColStdIndexedMapOfReal {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Adds a value to the map, returns the 1-based index.
    /// Note: f64 values are converted to u64 for hashing purposes.
    pub fn add(&mut self, value: f64) -> i32 {
        let key = value.to_bits();
        if !self.map.contains_key(&key) {
            let idx = self.data.len();
            self.map.insert(key, idx);
            self.data.push(value);
            (idx + 1) as i32
        } else {
            (*self.map.get(&key).unwrap() + 1) as i32
        }
    }

    /// Gets the 1-based index of a value.
    pub fn index(&self, value: f64) -> Option<i32> {
        let key = value.to_bits();
        self.map.get(&key).map(|idx| (*idx + 1) as i32)
    }

    /// Gets a value by 1-based index.
    pub fn at(&self, idx: i32) -> Option<&f64> {
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
            self.map.remove(&value.to_bits());
            // Update indices for remaining elements
            for i in pos..self.data.len() {
                self.map.insert(self.data[i].to_bits(), i);
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
    pub fn contains(&self, value: f64) -> bool {
        self.map.contains_key(&value.to_bits())
    }
}

impl Default for TColStdIndexedMapOfReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_index() {
        let mut map = TColStdIndexedMapOfReal::new();
        let idx1 = map.add(1.5);
        let idx2 = map.add(2.5);

        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(map.index(1.5), Some(1));
    }

    #[test]
    fn test_at() {
        let mut map = TColStdIndexedMapOfReal::new();
        map.add(1.1);
        map.add(2.2);

        assert_eq!(map.at(1), Some(&1.1));
        assert_eq!(map.at(2), Some(&2.2));
        assert_eq!(map.at(3), None);
    }

    #[test]
    fn test_size() {
        let mut map = TColStdIndexedMapOfReal::new();
        assert_eq!(map.size(), 0);

        map.add(0.5);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_contains() {
        let mut map = TColStdIndexedMapOfReal::new();
        map.add(3.14);

        assert!(map.contains(3.14));
        assert!(!map.contains(2.71));
    }
}
