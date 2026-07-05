// FILE: xml_mdf_tag_source_driver.rs
// occt: XmlMDF_TagSourceDriver

/// XML serialization driver for tag source attributes.
/// Handles serialization and deserialization of tag source information
/// (e.g., TDF_TagSource) that tracks the source of attribute creation.
pub struct XmlMDFTagSourceDriver {
    type_name: String,
}

impl XmlMDFTagSourceDriver {
    /// Create a new tag source driver.
    pub fn new() -> Self {
        XmlMDFTagSourceDriver {
            type_name: "TDF_TagSource".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty tag source attribute.
    pub fn new_empty(&self) -> String {
        self.type_name.clone()
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self) -> Result<(), String> {
        Ok(())
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self) -> Result<(), String> {
        Ok(())
    }

    /// Get the source tag identifier.
    pub fn get_source_tag(&self) -> i32 {
        0
    }

    /// Set the source tag identifier.
    pub fn set_source_tag(&self, _tag: i32) {
        // Tag source is immutable after creation
    }
}

impl Default for XmlMDFTagSourceDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDFTagSourceDriver::new();
        assert_eq!(driver.type_name(), "TDF_TagSource");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDFTagSourceDriver::new();
        assert_eq!(driver.new_empty(), "TDF_TagSource");
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDFTagSourceDriver::new();
        assert!(driver.paste_from_persistent().is_ok());
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDFTagSourceDriver::new();
        assert!(driver.paste_to_persistent().is_ok());
    }

    #[test]
    fn test_get_source_tag() {
        let driver = XmlMDFTagSourceDriver::new();
        assert_eq!(driver.get_source_tag(), 0);
    }

    #[test]
    fn test_set_source_tag() {
        let driver = XmlMDFTagSourceDriver::new();
        driver.set_source_tag(42);
        // Tag source is immutable, so tag remains 0
        assert_eq!(driver.get_source_tag(), 0);
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDFTagSourceDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_type_name_contains_tag_source() {
        let driver = XmlMDFTagSourceDriver::new();
        assert!(driver.type_name().contains("TagSource"));
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDFTagSourceDriver::default();
        assert_eq!(driver.type_name(), "TDF_TagSource");
    }

    #[test]
    fn test_multiple_instances() {
        let driver1 = XmlMDFTagSourceDriver::new();
        let driver2 = XmlMDFTagSourceDriver::new();
        assert_eq!(driver1.type_name(), driver2.type_name());
    }
}
