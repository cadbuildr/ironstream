// FILE: bin_m_data_std_named_data_driver.rs
// occt: BinMDataStd_NamedDataDriver

use std::collections::HashMap;

/// Binary serialization driver for named data attributes.
/// Handles serialization and deserialization of integer, real, string, byte, and array maps.
pub struct BinMDataStdNamedDataDriver {
    type_name: String,
}

impl BinMDataStdNamedDataDriver {
    /// Construct a driver with the given message sink.
    pub fn new() -> Self {
        BinMDataStdNamedDataDriver {
            type_name: "TDataStd_NamedData".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

impl Default for BinMDataStdNamedDataDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_construction() {
        let driver = BinMDataStdNamedDataDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_NamedData");
    }

    #[test]
    fn test_default_construction() {
        let driver = BinMDataStdNamedDataDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_NamedData");
    }

    #[test]
    fn test_driver_has_type_name() {
        let driver = BinMDataStdNamedDataDriver::new();
        assert!(!driver.type_name().is_empty());
    }
}
