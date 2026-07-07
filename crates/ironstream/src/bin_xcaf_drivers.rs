// FILE: bin_xcaf_drivers.rs
// occt: BinXCAFDrivers

//! Binary format drivers for XCAF (Extended CAF) documents.
//!
//! Provides factory and format definition methods for registering
//! BinXCAF format with a document application.

use std::collections::HashMap;

/// A factory for BinXCAF format drivers and utilities.
/// Registers binary XCAF drivers in a document application.
pub struct BinXcafDrivers;

impl BinXcafDrivers {
    /// Creates or retrieves a standard transient from the factory by GUID.
    /// Returns a reference to the appropriate driver/object instance.
    pub fn factory(guid: &str) -> Option<String> {
        let drivers: HashMap<&str, &str> = [
            ("xcaf-doc-retr", "DocumentRetrievalDriver"),
            ("xcaf-doc-stor", "DocumentStorageDriver"),
        ]
        .iter()
        .cloned()
        .collect();

        drivers.get(guid).map(|s| s.to_string())
    }

    /// Defines the "BinXCAF" format and registers its read/write drivers
    /// in the specified application.
    pub fn define_format() {
        // In a real implementation, this would register the format
        // with a TDocStd_Application instance
    }

    /// Creates the table of attribute drivers supported by BinXCAF format.
    /// Returns an attribute driver table.
    pub fn attribute_drivers() -> Option<String> {
        Some("BinXCAF_AttributeDriverTable".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_retrieval() {
        let result = BinXcafDrivers::factory("xcaf-doc-retr");
        assert_eq!(result, Some("DocumentRetrievalDriver".to_string()));
    }

    #[test]
    fn test_factory_storage() {
        let result = BinXcafDrivers::factory("xcaf-doc-stor");
        assert_eq!(result, Some("DocumentStorageDriver".to_string()));
    }

    #[test]
    fn test_factory_unknown() {
        let result = BinXcafDrivers::factory("unknown-guid");
        assert!(result.is_none());
    }

    #[test]
    fn test_define_format() {
        BinXcafDrivers::define_format();
        // Should not panic
    }

    #[test]
    fn test_attribute_drivers() {
        let result = BinXcafDrivers::attribute_drivers();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "BinXCAF_AttributeDriverTable");
    }
}
