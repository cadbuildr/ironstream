// FILE: ddf_data.rs
// occt: DDF_Data

//! DDF_Data: Utilities for working with TDF data.

use std::sync::Arc;

/// TDF_Data placeholder.
#[derive(Clone, Debug)]
pub struct TdfData {
    id: u32,
}

impl TdfData {
    pub fn new(id: u32) -> Self {
        TdfData { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// DDF_Data: Data framework utilities.
pub struct DdfData;

impl DdfData {
    /// Create a new data framework.
    pub fn create() -> Arc<TdfData> {
        Arc::new(TdfData::new(1))
    }

    /// Clear all data in a framework.
    pub fn clear(_data: &TdfData) {
        // In real implementation: clear all labels and attributes
    }

    /// Copy a data framework.
    pub fn copy(source: &TdfData) -> Arc<TdfData> {
        Arc::new(TdfData::new(source.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tdf_data_creation() {
        let data = TdfData::new(1);
        assert_eq!(data.id(), 1);
    }

    #[test]
    fn test_create() {
        let data = DdfData::create();
        assert_eq!(data.id(), 1);
    }

    #[test]
    fn test_copy() {
        let original = TdfData::new(42);
        let copied = DdfData::copy(&original);
        assert_eq!(copied.id(), 42);
    }
}
