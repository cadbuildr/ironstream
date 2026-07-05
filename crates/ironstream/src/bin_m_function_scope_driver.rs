// FILE: bin_m_function_scope_driver.rs
// occt: BinMFunction_ScopeDriver

/// Binary serialization driver for scope attributes.
/// Handles persistent <-> transient conversion for Scope attributes.
pub struct BinMFunctionScopeDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMFunctionScopeDriver {
    /// Creates a new ScopeDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMFunctionScopeDriver {
            message_driver,
            type_name: "TFunction_Scope".to_string(),
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
    fn test_scope_driver_creation() {
        let driver = BinMFunctionScopeDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TFunction_Scope");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_scope_driver_no_messenger() {
        let driver = BinMFunctionScopeDriver::new(None);
        assert_eq!(driver.type_name(), "TFunction_Scope");
        assert_eq!(driver.message_driver(), None);
    }
}
