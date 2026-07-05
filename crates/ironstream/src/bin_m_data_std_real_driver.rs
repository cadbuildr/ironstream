// FILE: bin_m_data_std_real_driver.rs
// occt: BinMDataStd_RealDriver

/// Binary serialization driver for real-valued attributes.
/// Handles persistent <-> transient conversion for Real attributes.
pub struct BinMDataStdRealDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdRealDriver {
    /// Creates a new RealDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdRealDriver {
            message_driver,
            type_name: "TDataStd_Real".to_string(),
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
    fn test_real_driver_creation() {
        let driver = BinMDataStdRealDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_Real");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_real_driver_no_messenger() {
        let driver = BinMDataStdRealDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_Real");
        assert_eq!(driver.message_driver(), None);
    }
}
