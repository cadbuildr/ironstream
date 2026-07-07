// FILE: xml_m_function_function_driver.rs
// occt: XmlMFunction_FunctionDriver

/// XmlMFunction_FunctionDriver handles XML serialization of function attributes.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMFunction_FunctionDriver {
    message_driver: String,
}

impl XmlMFunction_FunctionDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMFunction_FunctionDriver { message_driver }
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
        let driver = XmlMFunction_FunctionDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
