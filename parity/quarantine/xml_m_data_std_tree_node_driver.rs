// FILE: xml_m_data_std_tree_node_driver.rs
// occt: XmlMDataStd_TreeNodeDriver

/// XmlMDataStd_TreeNodeDriver handles XML serialization of tree node attributes.
/// Derives from XmlMDF_ADriver (attribute driver base).
pub struct XmlMDataStd_TreeNodeDriver {
    message_driver: String,
}

impl XmlMDataStd_TreeNodeDriver {
    pub fn new(message_driver: String) -> Self {
        XmlMDataStd_TreeNodeDriver { message_driver }
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
        let driver = XmlMDataStd_TreeNodeDriver::new("test".to_string());
        assert_eq!(driver.message_driver(), "test");
    }
}
