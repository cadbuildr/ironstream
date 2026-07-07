// FILE: std_storage_type_data.rs
// occt: StdStorage_TypeData

use std::collections::HashMap;

/// Type information storage
pub struct TypeData {
    types: HashMap<i32, String>,
}

impl TypeData {
    /// Create a new type data container
    pub fn new() -> Self {
        TypeData {
            types: HashMap::new(),
        }
    }

    /// Register a type
    pub fn register_type(&mut self, type_num: i32, type_name: &str) {
        self.types.insert(type_num, type_name.to_string());
    }

    /// Get type name
    pub fn get_type_name(&self, type_num: i32) -> Option<&str> {
        self.types.get(&type_num).map(|s| s.as_str())
    }

    /// Get number of types
    pub fn len(&self) -> usize {
        self.types.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

impl Default for TypeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let data = TypeData::new();
        assert!(data.is_empty());
    }

    #[test]
    fn test_register_type() {
        let mut data = TypeData::new();
        data.register_type(1, "MyType");
        assert_eq!(data.get_type_name(1), Some("MyType"));
    }

    #[test]
    fn test_multiple_types() {
        let mut data = TypeData::new();
        data.register_type(1, "Type1");
        data.register_type(2, "Type2");

        assert_eq!(data.len(), 2);
        assert_eq!(data.get_type_name(1), Some("Type1"));
    }
}
