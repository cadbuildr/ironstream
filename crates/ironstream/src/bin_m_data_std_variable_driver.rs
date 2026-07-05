// FILE: bin_m_data_std_variable_driver.rs
// occt: BinMDataStd_VariableDriver

/// Binary serialization driver for variable attributes.
/// Handles persistent <-> transient conversion for Variable attributes.
pub struct BinMDataStdVariableDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdVariableDriver {
    /// Creates a new VariableDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdVariableDriver {
            message_driver,
            type_name: "TDataStd_Variable".to_string(),
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
    fn test_variable_driver_creation() {
        let driver = BinMDataStdVariableDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_Variable");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_variable_driver_no_messenger() {
        let driver = BinMDataStdVariableDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_Variable");
        assert_eq!(driver.message_driver(), None);
    }
}
