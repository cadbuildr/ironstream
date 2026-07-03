// FILE: storage_root.rs
// occt: Storage_Root

use std::collections::HashMap;

/// A root object extracted from storage
/// Maps to `class Storage_Root : public Standard_Transient` in OCCT
#[derive(Debug, Clone)]
pub struct StorageRoot {
    name: String,
    type_name: String,
    reference: i32,
    object_id: Option<usize>, // References a persistent object by ID
}

impl StorageRoot {
    /// Create an empty root
    pub fn new() -> Self {
        StorageRoot {
            name: String::new(),
            type_name: String::new(),
            reference: 0,
            object_id: None,
        }
    }

    /// Create a root with name and object ID
    pub fn with_name_and_object(name: String, object_id: usize) -> Self {
        StorageRoot {
            name,
            type_name: String::new(),
            reference: 0,
            object_id: Some(object_id),
        }
    }

    /// Create a root with name, reference, and type
    pub fn with_reference(name: String, reference: i32, type_name: String) -> Self {
        StorageRoot {
            name,
            type_name,
            reference,
            object_id: None,
        }
    }

    /// Set the name of this root
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get the name of this root
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the object reference
    pub fn set_object(&mut self, object_id: usize) {
        self.object_id = Some(object_id);
    }

    /// Get the object ID if set
    pub fn object(&self) -> Option<usize> {
        self.object_id
    }

    /// Get the type name
    pub fn get_type(&self) -> &str {
        &self.type_name
    }

    /// Set the reference number
    pub fn set_reference(&mut self, reference: i32) {
        self.reference = reference;
    }

    /// Get the reference number
    pub fn reference(&self) -> i32 {
        self.reference
    }

    /// Set the type name
    pub fn set_type(&mut self, type_name: String) {
        self.type_name = type_name;
    }
}

impl Default for StorageRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_root_new() {
        let root = StorageRoot::new();
        assert_eq!(root.name(), "");
        assert_eq!(root.get_type(), "");
        assert_eq!(root.reference(), 0);
        assert_eq!(root.object(), None);
    }

    #[test]
    fn test_storage_root_with_name_and_object() {
        let root = StorageRoot::with_name_and_object("MyRoot".to_string(), 42);
        assert_eq!(root.name(), "MyRoot");
        assert_eq!(root.object(), Some(42));
    }

    #[test]
    fn test_storage_root_with_reference() {
        let root = StorageRoot::with_reference(
            "RefRoot".to_string(),
            123,
            "MyType".to_string(),
        );
        assert_eq!(root.name(), "RefRoot");
        assert_eq!(root.reference(), 123);
        assert_eq!(root.get_type(), "MyType");
    }

    #[test]
    fn test_storage_root_setters() {
        let mut root = StorageRoot::new();
        root.set_name("TestRoot".to_string());
        root.set_type("TestType".to_string());
        root.set_reference(99);
        root.set_object(55);

        assert_eq!(root.name(), "TestRoot");
        assert_eq!(root.get_type(), "TestType");
        assert_eq!(root.reference(), 99);
        assert_eq!(root.object(), Some(55));
    }

    #[test]
    fn test_storage_root_default() {
        let root = StorageRoot::default();
        assert_eq!(root.name(), "");
    }

    #[test]
    fn test_storage_root_clone() {
        let root = StorageRoot::with_name_and_object("Original".to_string(), 77);
        let cloned = root.clone();
        assert_eq!(root.name(), cloned.name());
        assert_eq!(root.object(), cloned.object());
    }
}
