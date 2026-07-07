// FILE: bin_m_function_function_driver.rs
// occt: BinMFunction_FunctionDriver

/// Binary serialization driver for function attributes.
/// Handles persistent <-> transient conversion for Function attributes.
pub struct BinMFunctionFunctionDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMFunctionFunctionDriver {
    /// Creates a new FunctionDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMFunctionFunctionDriver {
            message_driver,
            type_name: "TFunction_Function".to_string(),
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
    fn test_function_driver_creation() {
        let driver = BinMFunctionFunctionDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TFunction_Function");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_function_driver_no_messenger() {
        let driver = BinMFunctionFunctionDriver::new(None);
        assert_eq!(driver.type_name(), "TFunction_Function");
        assert_eq!(driver.message_driver(), None);
    }
}
