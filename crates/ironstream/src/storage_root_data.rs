// FILE: storage_root_data.rs
// occt: Storage_RootData

use std::collections::HashMap;
use crate::storage_root::StorageRoot;

/// Error status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    NoError,
    UnknownError,
    ReadError,
    WriteError,
}

/// Data container for storage roots
/// Maps to `class Storage_RootData : public Standard_Transient` in OCCT
#[derive(Debug, Clone)]
pub struct StorageRootData {
    roots: HashMap<String, StorageRoot>,
    error_status: StorageError,
    error_status_extension: String,
}

impl StorageRootData {
    /// Create a new empty root data container
    pub fn new() -> Self {
        StorageRootData {
            roots: HashMap::new(),
            error_status: StorageError::NoError,
            error_status_extension: String::new(),
        }
    }

    /// Get the number of roots
    pub fn number_of_roots(&self) -> usize {
        self.roots.len()
    }

    /// Add a root to the container
    pub fn add_root(&mut self, root: StorageRoot) {
        if !root.name().is_empty() {
            self.roots.insert(root.name().to_string(), root);
        }
    }

    /// Get all root names as a vector
    pub fn roots(&self) -> Vec<StorageRoot> {
        self.roots.values().cloned().collect()
    }

    /// Find a root by name
    pub fn find(&self, name: &str) -> Option<StorageRoot> {
        self.roots.get(name).cloned()
    }

    /// Check if a root with the given name exists
    pub fn is_root(&self, name: &str) -> bool {
        self.roots.contains_key(name)
    }

    /// Remove a root by name
    pub fn remove_root(&mut self, name: &str) {
        self.roots.remove(name);
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

    /// Set the error status (internal use)
    pub fn set_error_status(&mut self, error: StorageError) {
        self.error_status = error;
    }

    /// Set the error status extension (internal use)
    pub fn set_error_status_extension(&mut self, ext: String) {
        self.error_status_extension = ext;
    }

    /// Update a root's persistent object
    pub fn update_root(&mut self, name: &str, object_id: usize) {
        if let Some(root) = self.roots.get_mut(name) {
            root.set_object(object_id);
        }
    }
}

impl Default for StorageRootData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_root_data_new() {
        let data = StorageRootData::new();
        assert_eq!(data.number_of_roots(), 0);
        assert_eq!(data.error_status(), StorageError::NoError);
    }

    #[test]
    fn test_storage_root_data_add_root() {
        let mut data = StorageRootData::new();
        let root = StorageRoot::with_name_and_object("Root1".to_string(), 1);
        data.add_root(root);
        assert_eq!(data.number_of_roots(), 1);
    }

    #[test]
    fn test_storage_root_data_find() {
        let mut data = StorageRootData::new();
        let root = StorageRoot::with_name_and_object("TestRoot".to_string(), 42);
        data.add_root(root);

        let found = data.find("TestRoot");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), "TestRoot");
    }

    #[test]
    fn test_storage_root_data_is_root() {
        let mut data = StorageRootData::new();
        let root = StorageRoot::with_name_and_object("ExistsRoot".to_string(), 10);
        data.add_root(root);

        assert!(data.is_root("ExistsRoot"));
        assert!(!data.is_root("NonExistent"));
    }

    #[test]
    fn test_storage_root_data_remove_root() {
        let mut data = StorageRootData::new();
        let root = StorageRoot::with_name_and_object("ToRemove".to_string(), 5);
        data.add_root(root);
        assert_eq!(data.number_of_roots(), 1);

        data.remove_root("ToRemove");
        assert_eq!(data.number_of_roots(), 0);
    }

    #[test]
    fn test_storage_root_data_error_status() {
        let mut data = StorageRootData::new();
        assert_eq!(data.error_status(), StorageError::NoError);

        data.set_error_status(StorageError::ReadError);
        assert_eq!(data.error_status(), StorageError::ReadError);

        data.clear_error_status();
        assert_eq!(data.error_status(), StorageError::NoError);
    }

    #[test]
    fn test_storage_root_data_update_root() {
        let mut data = StorageRootData::new();
        let root = StorageRoot::with_name_and_object("UpdateMe".to_string(), 1);
        data.add_root(root);

        data.update_root("UpdateMe", 999);
        let updated = data.find("UpdateMe").unwrap();
        assert_eq!(updated.object(), Some(999));
    }

    #[test]
    fn test_storage_root_data_roots() {
        let mut data = StorageRootData::new();
        data.add_root(StorageRoot::with_name_and_object("Root1".to_string(), 1));
        data.add_root(StorageRoot::with_name_and_object("Root2".to_string(), 2));

        let all = data.roots();
        assert_eq!(all.len(), 2);
    }
}
