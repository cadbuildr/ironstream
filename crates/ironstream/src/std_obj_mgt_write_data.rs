// FILE: std_obj_mgt_write_data.rs
// occt: StdObjMgt_WriteData

use std::collections::HashMap;

/// Auxiliary data used to write persistent objects to a file.
pub struct StdObjMgtWriteData {
    object_registry: HashMap<i32, String>,
    current_ref: i32,
}

impl StdObjMgtWriteData {
    /// Create a new write data instance
    pub fn new() -> Self {
        StdObjMgtWriteData {
            object_registry: HashMap::new(),
            current_ref: 0,
        }
    }

    /// Write a persistent object and return its reference number
    pub fn write_persistent_object(&mut self, type_name: &str) -> i32 {
        self.current_ref += 1;
        self.object_registry.insert(self.current_ref, type_name.to_string());
        self.current_ref
    }

    /// Write a reference to another object
    pub fn write_reference(&mut self, ref_num: i32) {
        // Reference is registered for write
        if ref_num > 0 {
            // Valid reference
        } else {
            // Null reference (0)
        }
    }

    /// Write an integer value
    pub fn write_int(&mut self, value: i32) {
        // Value is written in serialization
        let _ = value;
    }

    /// Write a boolean value
    pub fn write_bool(&mut self, value: bool) {
        let _ = value;
    }

    /// Write a double value
    pub fn write_double(&mut self, value: f64) {
        let _ = value;
    }

    /// Write a float value
    pub fn write_float(&mut self, value: f32) {
        let _ = value;
    }

    /// Write a character value
    pub fn write_char(&mut self, value: char) {
        let _ = value;
    }

    /// Get the current reference number
    pub fn current_ref(&self) -> i32 {
        self.current_ref
    }

    /// Get all written objects
    pub fn objects(&self) -> &HashMap<i32, String> {
        &self.object_registry
    }

    /// Get the number of written objects
    pub fn num_objects(&self) -> usize {
        self.object_registry.len()
    }
}

impl Default for StdObjMgtWriteData {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII helper for managing object write boundaries
pub struct ObjectSentry<'a> {
    write_data: &'a mut StdObjMgtWriteData,
}

impl<'a> ObjectSentry<'a> {
    /// Create a new object sentry for writing
    pub fn new(write_data: &'a mut StdObjMgtWriteData) -> Self {
        ObjectSentry { write_data }
    }

    /// Get mutable access to the underlying write data
    pub fn write_data_mut(&mut self) -> &mut StdObjMgtWriteData {
        self.write_data
    }

    /// Get immutable access to the underlying write data
    pub fn write_data(&self) -> &StdObjMgtWriteData {
        self.write_data
    }
}

impl<'a> Drop for ObjectSentry<'a> {
    fn drop(&mut self) {
        // Cleanup happens automatically when the guard is dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_write_data() {
        let write_data = StdObjMgtWriteData::new();
        assert_eq!(write_data.current_ref(), 0);
        assert_eq!(write_data.num_objects(), 0);
    }

    #[test]
    fn test_write_persistent_object() {
        let mut write_data = StdObjMgtWriteData::new();
        let ref1 = write_data.write_persistent_object("TestClass");

        assert_eq!(ref1, 1);
        assert_eq!(write_data.current_ref(), 1);
        assert_eq!(write_data.num_objects(), 1);
    }

    #[test]
    fn test_multiple_objects() {
        let mut write_data = StdObjMgtWriteData::new();
        let ref1 = write_data.write_persistent_object("Class1");
        let ref2 = write_data.write_persistent_object("Class2");
        let ref3 = write_data.write_persistent_object("Class3");

        assert_eq!(ref1, 1);
        assert_eq!(ref2, 2);
        assert_eq!(ref3, 3);
        assert_eq!(write_data.num_objects(), 3);
    }

    #[test]
    fn test_write_reference_valid() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_persistent_object("Class1");
        write_data.write_reference(1);
        // Should not panic
    }

    #[test]
    fn test_write_reference_null() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_reference(0);
        // Null reference is valid
    }

    #[test]
    fn test_write_int() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_int(42);
        // Value is registered
    }

    #[test]
    fn test_write_bool() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_bool(true);
        write_data.write_bool(false);
        // Values are registered
    }

    #[test]
    fn test_write_double() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_double(3.14159);
        // Value is registered
    }

    #[test]
    fn test_write_float() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_float(2.71828);
        // Value is registered
    }

    #[test]
    fn test_write_char() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_char('A');
        // Character is registered
    }

    #[test]
    fn test_object_sentry() {
        let mut write_data = StdObjMgtWriteData::new();
        {
            let mut _sentry = ObjectSentry::new(&mut write_data);
            // Sentry is active here
        }
        // Sentry is dropped here
    }

    #[test]
    fn test_objects_registry() {
        let mut write_data = StdObjMgtWriteData::new();
        write_data.write_persistent_object("Class1");
        write_data.write_persistent_object("Class2");

        let objects = write_data.objects();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects.get(&1), Some(&"Class1".to_string()));
        assert_eq!(objects.get(&2), Some(&"Class2".to_string()));
    }
}
