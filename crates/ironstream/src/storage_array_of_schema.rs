// FILE: storage_array_of_schema.rs
// occt: Storage_ArrayOfSchema

//! Array of storage schemas.

/// Array of storage schemas (placeholder).
/// This module provides array support for storage schema objects.
#[derive(Debug, Clone)]
pub struct StorageArrayOfSchema {
    items: Vec<String>,
}

impl StorageArrayOfSchema {
    /// Create a new storage array of schemas.
    pub fn new() -> Self {
        StorageArrayOfSchema {
            items: Vec::new(),
        }
    }

    /// Create with capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        StorageArrayOfSchema {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Get the length of the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Add an item to the array.
    pub fn append(&mut self, item: String) {
        self.items.push(item);
    }

    /// Clear the array.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for StorageArrayOfSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_array_creation() {
        let array = StorageArrayOfSchema::new();
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_storage_array_with_capacity() {
        let array = StorageArrayOfSchema::with_capacity(10);
        assert!(array.is_empty());
    }

    #[test]
    fn test_storage_array_append() {
        let mut array = StorageArrayOfSchema::new();
        array.append("Schema1".to_string());
        array.append("Schema2".to_string());
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn test_storage_array_clear() {
        let mut array = StorageArrayOfSchema::new();
        array.append("Schema1".to_string());
        array.clear();
        assert!(array.is_empty());
    }

    #[test]
    fn test_storage_array_default() {
        let array = StorageArrayOfSchema::default();
        assert!(array.is_empty());
    }
}
