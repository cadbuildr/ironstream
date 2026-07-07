// FILE: std_storage_root_data.rs
// occt: StdStorage_RootData

use std::collections::HashMap;

/// Root data collection
pub struct RootData {
    roots: HashMap<String, i32>,
}

impl RootData {
    /// Create a new root data container
    pub fn new() -> Self {
        RootData {
            roots: HashMap::new(),
        }
    }

    /// Add a root
    pub fn add_root(&mut self, name: &str, ref_num: i32) {
        self.roots.insert(name.to_string(), ref_num);
    }

    /// Get a root reference
    pub fn get_root(&self, name: &str) -> Option<i32> {
        self.roots.get(name).copied()
    }

    /// Get number of roots
    pub fn len(&self) -> usize {
        self.roots.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }
}

impl Default for RootData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let data = RootData::new();
        assert!(data.is_empty());
    }

    #[test]
    fn test_add_root() {
        let mut data = RootData::new();
        data.add_root("Root1", 1);
        assert_eq!(data.get_root("Root1"), Some(1));
    }

    #[test]
    fn test_multiple_roots() {
        let mut data = RootData::new();
        data.add_root("Root1", 1);
        data.add_root("Root2", 2);

        assert_eq!(data.len(), 2);
    }
}
