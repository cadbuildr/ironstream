// FILE: storage_typed_call_back.rs
// occt: Storage_TypedCallBack

/// A typed callback wrapper
/// Maps to `class Storage_TypedCallBack : public Standard_Transient` in OCCT
#[derive(Debug, Clone)]
pub struct StorageTypedCallBack {
    type_name: String,
    callback_id: Option<usize>, // References a callback object
    index: i32,
}

impl StorageTypedCallBack {
    /// Create an empty typed callback
    pub fn new() -> Self {
        StorageTypedCallBack {
            type_name: String::new(),
            callback_id: None,
            index: 0,
        }
    }

    /// Create a typed callback with type name and callback ID
    pub fn with_type_and_callback(type_name: String, callback_id: usize) -> Self {
        StorageTypedCallBack {
            type_name,
            callback_id: Some(callback_id),
            index: 0,
        }
    }

    /// Set the type name
    pub fn set_type(&mut self, type_name: String) {
        self.type_name = type_name;
    }

    /// Get the type name
    pub fn get_type(&self) -> &str {
        &self.type_name
    }

    /// Set the callback reference
    pub fn set_callback(&mut self, callback_id: usize) {
        self.callback_id = Some(callback_id);
    }

    /// Get the callback reference
    pub fn callback(&self) -> Option<usize> {
        self.callback_id
    }

    /// Set the index
    pub fn set_index(&mut self, index: i32) {
        self.index = index;
    }

    /// Get the index
    pub fn index(&self) -> i32 {
        self.index
    }
}

impl Default for StorageTypedCallBack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_typed_call_back_new() {
        let tcb = StorageTypedCallBack::new();
        assert_eq!(tcb.get_type(), "");
        assert_eq!(tcb.callback(), None);
        assert_eq!(tcb.index(), 0);
    }

    #[test]
    fn test_storage_typed_call_back_with_type_and_callback() {
        let tcb = StorageTypedCallBack::with_type_and_callback("MyType".to_string(), 42);
        assert_eq!(tcb.get_type(), "MyType");
        assert_eq!(tcb.callback(), Some(42));
    }

    #[test]
    fn test_storage_typed_call_back_setters() {
        let mut tcb = StorageTypedCallBack::new();
        tcb.set_type("TestType".to_string());
        tcb.set_callback(99);
        tcb.set_index(10);

        assert_eq!(tcb.get_type(), "TestType");
        assert_eq!(tcb.callback(), Some(99));
        assert_eq!(tcb.index(), 10);
    }

    #[test]
    fn test_storage_typed_call_back_clone() {
        let tcb = StorageTypedCallBack::with_type_and_callback("OrigType".to_string(), 77);
        let cloned = tcb.clone();
        assert_eq!(tcb.get_type(), cloned.get_type());
        assert_eq!(tcb.callback(), cloned.callback());
    }

    #[test]
    fn test_storage_typed_call_back_default() {
        let tcb = StorageTypedCallBack::default();
        assert_eq!(tcb.get_type(), "");
    }
}
