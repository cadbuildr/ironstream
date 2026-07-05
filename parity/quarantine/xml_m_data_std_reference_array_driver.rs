// FILE: xml_m_data_std_reference_array_driver.rs
// occt: XmlMDataStd_ReferenceArrayDriver

/// XmlMDataStd_ReferenceArrayDriver handles XML serialization of reference arrays.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMDataStd_ReferenceArrayDriver {
    message_driver: String,
}

impl XmlMDataStd_ReferenceArrayDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMDataStd_ReferenceArrayDriver { message_driver }
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
        let driver = XmlMDataStd_ReferenceArrayDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
