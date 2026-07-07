// FILE: xml_m_doc_std_x_link_driver.rs
// occt: XmlMDocStd_XLinkDriver

/// XmlMDocStd_XLinkDriver handles XML serialization of XLink attributes.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMDocStd_XLinkDriver {
    message_driver: String,
}

impl XmlMDocStd_XLinkDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMDocStd_XLinkDriver { message_driver }
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
        let driver = XmlMDocStd_XLinkDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
