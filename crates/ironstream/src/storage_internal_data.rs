// FILE: storage_internal_data.rs
// occt: Storage_InternalData

//! Internal data storage for persistent objects.

use std::sync::Arc;
use std::collections::HashMap;

/// Internal data structure for storage operations.
pub struct StorageInternalData {
    obj_id: i32,
    type_id: i32,
    type_binding: HashMap<String, Arc<dyn std::any::Any>>,
    read_array: Vec<Arc<dyn std::any::Any>>,
}

impl StorageInternalData {
    /// Create a new storage internal data object.
    pub fn new() -> Self {
        StorageInternalData {
            obj_id: 0,
            type_id: 0,
            type_binding: HashMap::new(),
            read_array: Vec::new(),
        }
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.obj_id = 0;
        self.type_id = 0;
        self.type_binding.clear();
        self.read_array.clear();
    }

    /// Get the current object ID.
    pub fn obj_id(&self) -> i32 {
        self.obj_id
    }

    /// Set the object ID.
    pub fn set_obj_id(&mut self, id: i32) {
        self.obj_id = id;
    }

    /// Get the current type ID.
    pub fn type_id(&self) -> i32 {
        self.type_id
    }

    /// Set the type ID.
    pub fn set_type_id(&mut self, id: i32) {
        self.type_id = id;
    }

    /// Add a type binding.
    pub fn add_type_binding(&mut self, name: String, callback: Arc<dyn std::any::Any>) {
        self.type_binding.insert(name, callback);
    }

    /// Get a type binding.
    pub fn get_type_binding(&self, name: &str) -> Option<Arc<dyn std::any::Any>> {
        self.type_binding.get(name).map(Arc::clone)
    }

    /// Append to read array.
    pub fn append_read_array(&mut self, item: Arc<dyn std::any::Any>) {
        self.read_array.push(item);
    }

    /// Get the read array.
    pub fn read_array(&self) -> &[Arc<dyn std::any::Any>] {
        &self.read_array
    }
}

impl Default for StorageInternalData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_data_creation() {
        let data = StorageInternalData::new();
        assert_eq!(data.obj_id(), 0);
        assert_eq!(data.type_id(), 0);
    }

    #[test]
    fn test_internal_data_ids() {
        let mut data = StorageInternalData::new();
        data.set_obj_id(42);
        data.set_type_id(10);
        assert_eq!(data.obj_id(), 42);
        assert_eq!(data.type_id(), 10);
    }

    #[test]
    fn test_internal_data_type_binding() {
        let mut data = StorageInternalData::new();
        let callback = Arc::new("test_callback");
        data.add_type_binding("Type1".to_string(), callback);
        assert!(data.get_type_binding("Type1").is_some());
        assert!(data.get_type_binding("Type2").is_none());
    }

    #[test]
    fn test_internal_data_read_array() {
        let mut data = StorageInternalData::new();
        data.append_read_array(Arc::new(1i32));
        data.append_read_array(Arc::new(2i32));
        assert_eq!(data.read_array().len(), 2);
    }

    #[test]
    fn test_internal_data_clear() {
        let mut data = StorageInternalData::new();
        data.set_obj_id(100);
        data.append_read_array(Arc::new(42i32));
        data.clear();
        assert_eq!(data.obj_id(), 0);
        assert!(data.read_array().is_empty());
    }

    #[test]
    fn test_internal_data_default() {
        let data = StorageInternalData::default();
        assert_eq!(data.obj_id(), 0);
    }
}
