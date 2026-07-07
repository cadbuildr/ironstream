// FILE: bin_m_data_std_real_list_driver.rs
// occt: BinMDataStd_RealListDriver

/// Binary serialization driver for real list attributes.
/// Handles persistent <-> transient conversion for RealList attributes.
pub struct BinMDataStdRealListDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdRealListDriver {
    /// Creates a new RealListDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdRealListDriver {
            message_driver,
            type_name: "TDataStd_RealList".to_string(),
        }
    }

    /// Returns the type name of the attribute object.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the current message driver of this driver.
    pub fn message_driver(&self) -> Option<&str> {
        self.message_driver.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_list_driver_creation() {
        let driver = BinMDataStdRealListDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_RealList");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_real_list_driver_no_messenger() {
        let driver = BinMDataStdRealListDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_RealList");
        assert_eq!(driver.message_driver(), None);
    }
}
