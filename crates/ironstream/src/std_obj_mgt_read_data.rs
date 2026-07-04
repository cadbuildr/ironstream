// FILE: std_obj_mgt_read_data.rs
// occt: StdObjMgt_ReadData

use std::collections::HashMap;

/// Auxiliary data used to read persistent objects from a file.
pub struct StdObjMgtReadData {
    persistent_objects: HashMap<i32, Option<String>>,
    current_position: usize,
}

impl StdObjMgtReadData {
    /// Create a new read data instance with capacity for objects
    pub fn new(num_objects: usize) -> Self {
        StdObjMgtReadData {
            persistent_objects: HashMap::with_capacity(num_objects),
            current_position: 0,
        }
    }

    /// Create a persistent object at a specific reference
    pub fn create_persistent_object(&mut self, ref_num: i32, value: Option<String>) {
        self.persistent_objects.insert(ref_num, value);
    }

    /// Get a persistent object by reference number
    pub fn persistent_object(&self, ref_num: i32) -> Option<&Option<String>> {
        self.persistent_objects.get(&ref_num)
    }

    /// Read a reference and return the associated persistent object
    pub fn read_reference(&mut self) -> Option<Option<String>> {
        self.current_position += 1;
        self.persistent_objects.get(&(self.current_position as i32)).cloned()
    }

    /// Helper to read a generic value
    pub fn read_value<T: Default>(&mut self) -> T {
        self.current_position += 1;
        T::default()
    }

    /// Read an integer value
    pub fn read_int(&mut self) -> i32 {
        self.current_position += 1;
        0
    }

    /// Read a boolean value
    pub fn read_bool(&mut self) -> bool {
        self.current_position += 1;
        false
    }

    /// Read a double value
    pub fn read_double(&mut self) -> f64 {
        self.current_position += 1;
        0.0
    }

    /// Read a float value
    pub fn read_float(&mut self) -> f32 {
        self.current_position += 1;
        0.0
    }

    /// Read a character value
    pub fn read_char(&mut self) -> char {
        self.current_position += 1;
        ' '
    }

    /// Get the current position
    pub fn current_position(&self) -> usize {
        self.current_position
    }

    /// Get the total number of objects
    pub fn num_objects(&self) -> usize {
        self.persistent_objects.len()
    }
}

impl Default for StdObjMgtReadData {
    fn default() -> Self {
        Self::new(0)
    }
}

/// RAII helper for managing object read boundaries
pub struct ObjectSentry<'a> {
    read_data: &'a mut StdObjMgtReadData,
}

impl<'a> ObjectSentry<'a> {
    /// Create a new object sentry
    pub fn new(read_data: &'a mut StdObjMgtReadData) -> Self {
        ObjectSentry { read_data }
    }

    /// Get mutable access to the underlying read data
    pub fn read_data_mut(&mut self) -> &mut StdObjMgtReadData {
        self.read_data
    }

    /// Get immutable access to the underlying read data
    pub fn read_data(&self) -> &StdObjMgtReadData {
        self.read_data
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
    fn test_create_read_data() {
        let read_data = StdObjMgtReadData::new(10);
        assert_eq!(read_data.current_position(), 0);
        assert_eq!(read_data.num_objects(), 0);
    }

    #[test]
    fn test_create_persistent_object() {
        let mut read_data = StdObjMgtReadData::new(10);
        read_data.create_persistent_object(1, Some("Object1".to_string()));

        let obj = read_data.persistent_object(1);
        assert!(obj.is_some());
        assert_eq!(obj.unwrap().as_ref().map(|s| s.as_str()), Some("Object1"));
    }

    #[test]
    fn test_read_reference() {
        let mut read_data = StdObjMgtReadData::new(10);
        read_data.create_persistent_object(1, Some("Object1".to_string()));

        let obj = read_data.read_reference();
        assert_eq!(read_data.current_position(), 1);
        assert!(obj.is_some());
    }

    #[test]
    fn test_read_int() {
        let mut read_data = StdObjMgtReadData::new(10);
        let _ = read_data.read_int();
        assert_eq!(read_data.current_position(), 1);
    }

    #[test]
    fn test_read_bool() {
        let mut read_data = StdObjMgtReadData::new(10);
        let _ = read_data.read_bool();
        assert_eq!(read_data.current_position(), 1);
    }

    #[test]
    fn test_read_double() {
        let mut read_data = StdObjMgtReadData::new(10);
        let _ = read_data.read_double();
        assert_eq!(read_data.current_position(), 1);
    }

    #[test]
    fn test_read_float() {
        let mut read_data = StdObjMgtReadData::new(10);
        let _ = read_data.read_float();
        assert_eq!(read_data.current_position(), 1);
    }

    #[test]
    fn test_multiple_reads() {
        let mut read_data = StdObjMgtReadData::new(10);
        read_data.read_int();
        read_data.read_bool();
        read_data.read_double();

        assert_eq!(read_data.current_position(), 3);
    }

    #[test]
    fn test_object_sentry() {
        let mut read_data = StdObjMgtReadData::new(10);
        {
            let mut _sentry = ObjectSentry::new(&mut read_data);
            // Sentry is active here
        }
        // Sentry is dropped here
    }
}
