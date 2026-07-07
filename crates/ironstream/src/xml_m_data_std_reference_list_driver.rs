// FILE: xml_m_data_std_reference_list_driver.rs
// occt: XmlMDataStd_ReferenceListDriver

/// XmlMDataStd_ReferenceListDriver handles XML serialization of reference lists.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMDataStd_ReferenceListDriver {
    message_driver: String,
}

impl XmlMDataStd_ReferenceListDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMDataStd_ReferenceListDriver { message_driver }
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
        let driver = XmlMDataStd_ReferenceListDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
