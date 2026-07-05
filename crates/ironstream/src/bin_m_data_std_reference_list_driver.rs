// FILE: bin_m_data_std_reference_list_driver.rs
// occt: BinMDataStd_ReferenceListDriver

/// Binary serialization driver for reference list attributes.
/// Handles persistent <-> transient conversion for ReferenceList attributes.
pub struct BinMDataStdReferenceListDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdReferenceListDriver {
    /// Creates a new ReferenceListDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdReferenceListDriver {
            message_driver,
            type_name: "TDataStd_ReferenceList".to_string(),
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
    fn test_reference_list_driver_creation() {
        let driver = BinMDataStdReferenceListDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_ReferenceList");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_reference_list_driver_no_messenger() {
        let driver = BinMDataStdReferenceListDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_ReferenceList");
        assert_eq!(driver.message_driver(), None);
    }
}
