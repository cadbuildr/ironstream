// FILE: storage_default_call_back.rs
// occt: Storage_DefaultCallBack

//! Default callback implementation for storage operations.

use std::sync::Arc;

/// Default callback implementation for storage operations.
pub struct StorageDefaultCallBack;

impl StorageDefaultCallBack {
    /// Create a new default callback.
    pub fn new() -> Self {
        StorageDefaultCallBack
    }

    /// Create a new persistent object.
    pub fn new_object(&self) -> Arc<dyn std::any::Any> {
        Arc::new(())
    }

    /// Add a persistent object.
    pub fn add(&self, _obj: Arc<dyn std::any::Any>) {
        // Default implementation: no-op
    }

    /// Write a persistent object.
    pub fn write(&self, _obj: Arc<dyn std::any::Any>) {
        // Default implementation: no-op
    }

    /// Read a persistent object.
    pub fn read(&self, _obj: Arc<dyn std::any::Any>) {
        // Default implementation: no-op
    }
}

impl Default for StorageDefaultCallBack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_default_callback_creation() {
        let callback = StorageDefaultCallBack::new();
        let _obj = callback.new_object();
    }

    #[test]
    fn test_storage_default_callback_default() {
        let callback = StorageDefaultCallBack::default();
        let _obj = callback.new_object();
    }

    #[test]
    fn test_storage_default_callback_operations() {
        let callback = StorageDefaultCallBack::new();
        let obj = callback.new_object();
        callback.add(obj.clone());
        callback.write(obj.clone());
        callback.read(obj);
    }
}
