// FILE: storage_call_back.rs
// occt: Storage_CallBack

//! Callback interface for storage operations.

use std::sync::Arc;

/// Callback interface for storage operations.
pub trait StorageCallBack: Send + Sync {
    /// Create a new persistent object.
    fn new_object(&self) -> Arc<dyn std::any::Any>;

    /// Add a persistent object.
    fn add(&self, obj: Arc<dyn std::any::Any>);

    /// Write a persistent object.
    fn write(&self, obj: Arc<dyn std::any::Any>);

    /// Read a persistent object.
    fn read(&self, obj: Arc<dyn std::any::Any>);
}

/// Default callback implementation.
pub struct DefaultStorageCallBack;

impl StorageCallBack for DefaultStorageCallBack {
    fn new_object(&self) -> Arc<dyn std::any::Any> {
        Arc::new(())
    }

    fn add(&self, _obj: Arc<dyn std::any::Any>) {
        // Default implementation: no-op
    }

    fn write(&self, _obj: Arc<dyn std::any::Any>) {
        // Default implementation: no-op
    }

    fn read(&self, _obj: Arc<dyn std::any::Any>) {
        // Default implementation: no-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_storage_callback_creation() {
        let callback = DefaultStorageCallBack;
        let _obj = callback.new_object();
    }

    #[test]
    fn test_default_storage_callback_add() {
        let callback = DefaultStorageCallBack;
        let obj = callback.new_object();
        callback.add(obj);
    }

    #[test]
    fn test_default_storage_callback_write() {
        let callback = DefaultStorageCallBack;
        let obj = callback.new_object();
        callback.write(obj);
    }

    #[test]
    fn test_default_storage_callback_read() {
        let callback = DefaultStorageCallBack;
        let obj = callback.new_object();
        callback.read(obj);
    }
}
