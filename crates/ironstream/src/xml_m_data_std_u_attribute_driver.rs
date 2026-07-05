// FILE: xml_m_data_std_u_attribute_driver.rs
// occt: XmlMDataStd_UAttributeDriver

/// XmlMDataStd_UAttributeDriver handles XML serialization of UAttribute attributes.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMDataStd_UAttributeDriver {
    message_driver: String,
}

impl XmlMDataStd_UAttributeDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMDataStd_UAttributeDriver { message_driver }
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
        let driver = XmlMDataStd_UAttributeDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
