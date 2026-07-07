// FILE: xml_m_function_scope_driver.rs
// occt: XmlMFunction_ScopeDriver

/// XmlMFunction_ScopeDriver handles XML persistence of function scopes.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMFunction_ScopeDriver {
    message_driver: String,
}

impl XmlMFunction_ScopeDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMFunction_ScopeDriver { message_driver }
    }

    pub fn message_driver(&self) -> &str {
        &self.message_driver
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMFunction_ScopeDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
