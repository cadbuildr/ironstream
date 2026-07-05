// FILE: xml_m_data_std_variable_driver.rs
// occt: XmlMDataStd_VariableDriver

/// XmlMDataStd_VariableDriver handles XML serialization of variable attributes.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMDataStd_VariableDriver {
    message_driver: String,
}

impl XmlMDataStd_VariableDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMDataStd_VariableDriver { message_driver }
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
        let driver = XmlMDataStd_VariableDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
