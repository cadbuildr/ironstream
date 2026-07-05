// FILE: bin_m_data_std_u_attribute_driver.rs
// occt: BinMDataStd_UAttributeDriver

/// Binary serialization driver for user-defined attributes.
/// Handles persistent <-> transient conversion for UAttribute attributes.
pub struct BinMDataStdUAttributeDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdUAttributeDriver {
    /// Creates a new UAttributeDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdUAttributeDriver {
            message_driver,
            type_name: "TDataStd_UAttribute".to_string(),
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
    fn test_u_attribute_driver_creation() {
        let driver = BinMDataStdUAttributeDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_UAttribute");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_u_attribute_driver_no_messenger() {
        let driver = BinMDataStdUAttributeDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_UAttribute");
        assert_eq!(driver.message_driver(), None);
    }
}
