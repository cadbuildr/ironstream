// FILE: bin_m_data_std_reference_array_driver.rs
// occt: BinMDataStd_ReferenceArrayDriver

/// Binary serialization driver for reference array attributes.
/// Handles persistent <-> transient conversion for ReferenceArray attributes.
pub struct BinMDataStdReferenceArrayDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdReferenceArrayDriver {
    /// Creates a new ReferenceArrayDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdReferenceArrayDriver {
            message_driver,
            type_name: "TDataStd_ReferenceArray".to_string(),
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
    fn test_reference_array_driver_creation() {
        let driver = BinMDataStdReferenceArrayDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_ReferenceArray");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_reference_array_driver_no_messenger() {
        let driver = BinMDataStdReferenceArrayDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_ReferenceArray");
        assert_eq!(driver.message_driver(), None);
    }
}
