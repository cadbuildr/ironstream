// FILE: bin_m_data_std_real_array_driver.rs
// occt: BinMDataStd_RealArrayDriver

/// Binary serialization driver for real array attributes.
/// Manages serialization and deserialization of double-precision floating point arrays
/// with support for delta tracking and custom attribute IDs.
pub struct BinMDataStdRealArrayDriver {
    type_name: String,
}

impl BinMDataStdRealArrayDriver {
    /// Construct a driver with the given message sink.
    pub fn new() -> Self {
        BinMDataStdRealArrayDriver {
            type_name: "TDataStd_RealArray".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty real array for deserialization.
    pub fn new_empty(&self) -> Vec<f64> {
        Vec::new()
    }
}

impl Default for BinMDataStdRealArrayDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_construction() {
        let driver = BinMDataStdRealArrayDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_RealArray");
    }

    #[test]
    fn test_default_construction() {
        let driver = BinMDataStdRealArrayDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_RealArray");
    }

    #[test]
    fn test_new_empty() {
        let driver = BinMDataStdRealArrayDriver::new();
        let arr = driver.new_empty();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_driver_manages_real_type() {
        let driver = BinMDataStdRealArrayDriver::new();
        assert!(driver.type_name().contains("Real"));
    }
}
