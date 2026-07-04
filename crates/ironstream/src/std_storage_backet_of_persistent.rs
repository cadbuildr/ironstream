// FILE: std_storage_backet_of_persistent.rs
// occt: StdStorage_BacketOfPersistent

use std::collections::HashMap;

/// Container of persistent objects (bucket/collection)
pub struct BacketOfPersistent {
    objects: HashMap<i32, String>,
}

impl BacketOfPersistent {
    /// Create a new bucket
    pub fn new() -> Self {
        BacketOfPersistent {
            objects: HashMap::new(),
        }
    }

    /// Add an object
    pub fn add(&mut self, id: i32, type_name: &str) {
        self.objects.insert(id, type_name.to_string());
    }

    /// Get object count
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Get object type by id
    pub fn get(&self, id: i32) -> Option<&str> {
        self.objects.get(&id).map(|s| s.as_str())
    }
}

impl Default for BacketOfPersistent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let bucket = BacketOfPersistent::new();
        assert!(bucket.is_empty());
    }

    #[test]
    fn test_add() {
        let mut bucket = BacketOfPersistent::new();
        bucket.add(1, "MyObject");
        assert_eq!(bucket.len(), 1);
        assert_eq!(bucket.get(1), Some("MyObject"));
    }
}
