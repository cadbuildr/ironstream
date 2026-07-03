// FILE: storage_type_data.rs
// occt: Storage_TypeData

use std::collections::HashMap;

/// Error status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    NoError,
    UnknownError,
    ReadError,
    WriteError,
}

/// Data container for storage type information
/// Maps to `class Storage_TypeData : public Standard_Transient` in OCCT
#[derive(Debug, Clone)]
pub struct StorageTypeData {
    /// Map from type name to type number
    type_map: HashMap<String, i32>,
    /// Reverse map from type number to type name
    reverse_map: HashMap<i32, String>,
    next_type_num: i32,
    error_status: StorageError,
    error_status_extension: String,
}

impl StorageTypeData {
    /// Create a new empty type data container
    pub fn new() -> Self {
        StorageTypeData {
            type_map: HashMap::new(),
            reverse_map: HashMap::new(),
            next_type_num: 0,
            error_status: StorageError::NoError,
            error_status_extension: String::new(),
        }
    }

    /// Get the number of registered types
    pub fn number_of_types(&self) -> i32 {
        self.type_map.len() as i32
    }

    /// Add a type to the registry
    pub fn add_type(&mut self, name: &str, type_num: i32) {
        self.type_map.insert(name.to_string(), type_num);
        self.reverse_map.insert(type_num, name.to_string());
        if type_num >= self.next_type_num {
            self.next_type_num = type_num + 1;
        }
    }

    /// Get the type name by type number
    pub fn get_type_by_num(&self, type_num: i32) -> Option<&str> {
        self.reverse_map.get(&type_num).map(|s| s.as_str())
    }

    /// Get the type number by type name
    pub fn get_type_by_name(&self, name: &str) -> Option<i32> {
        self.type_map.get(name).copied()
    }

    /// Check if a type with the given name exists
    pub fn is_type(&self, name: &str) -> bool {
        self.type_map.contains_key(name)
    }

    /// Get all type names as a vector
    pub fn types(&self) -> Vec<String> {
        self.type_map.keys().cloned().collect()
    }

    /// Get the current error status
    pub fn error_status(&self) -> StorageError {
        self.error_status
    }

    /// Get the error status extension message
    pub fn error_status_extension(&self) -> &str {
        &self.error_status_extension
    }

    /// Clear the error status
    pub fn clear_error_status(&mut self) {
        self.error_status = StorageError::NoError;
        self.error_status_extension.clear();
    }

    /// Clear all types
    pub fn clear(&mut self) {
        self.type_map.clear();
        self.reverse_map.clear();
        self.next_type_num = 0;
    }

    /// Set the error status (internal use)
    pub fn set_error_status(&mut self, error: StorageError) {
        self.error_status = error;
    }

    /// Set the error status extension (internal use)
    pub fn set_error_status_extension(&mut self, ext: String) {
        self.error_status_extension = ext;
    }
}

impl Default for StorageTypeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_type_data_new() {
        let data = StorageTypeData::new();
        assert_eq!(data.number_of_types(), 0);
        assert_eq!(data.error_status(), StorageError::NoError);
    }

    #[test]
    fn test_storage_type_data_add_type() {
        let mut data = StorageTypeData::new();
        data.add_type("MyType", 1);
        assert_eq!(data.number_of_types(), 1);
    }

    #[test]
    fn test_storage_type_data_get_type_by_num() {
        let mut data = StorageTypeData::new();
        data.add_type("TypeA", 10);
        assert_eq!(data.get_type_by_num(10), Some("TypeA"));
        assert_eq!(data.get_type_by_num(99), None);
    }

    #[test]
    fn test_storage_type_data_get_type_by_name() {
        let mut data = StorageTypeData::new();
        data.add_type("TypeB", 20);
        assert_eq!(data.get_type_by_name("TypeB"), Some(20));
        assert_eq!(data.get_type_by_name("NonExistent"), None);
    }

    #[test]
    fn test_storage_type_data_is_type() {
        let mut data = StorageTypeData::new();
        data.add_type("ExistingType", 5);
        assert!(data.is_type("ExistingType"));
        assert!(!data.is_type("MissingType"));
    }

    #[test]
    fn test_storage_type_data_types() {
        let mut data = StorageTypeData::new();
        data.add_type("Type1", 1);
        data.add_type("Type2", 2);
        let types = data.types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"Type1".to_string()));
        assert!(types.contains(&"Type2".to_string()));
    }

    #[test]
    fn test_storage_type_data_clear() {
        let mut data = StorageTypeData::new();
        data.add_type("Type1", 1);
        data.add_type("Type2", 2);
        assert_eq!(data.number_of_types(), 2);

        data.clear();
        assert_eq!(data.number_of_types(), 0);
    }

    #[test]
    fn test_storage_type_data_error_status() {
        let mut data = StorageTypeData::new();
        data.set_error_status(StorageError::WriteError);
        assert_eq!(data.error_status(), StorageError::WriteError);

        data.clear_error_status();
        assert_eq!(data.error_status(), StorageError::NoError);
    }
}
