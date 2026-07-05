// FILE: xml_m_data_std_ascii_string_driver.rs
// occt: XmlMDataStd_AsciiStringDriver

/// XML serialization driver for ASCII string attributes.
/// Handles serialization and deserialization of ASCII string values (TDataStd_AsciiString).
pub struct XmlMDataStdAsciiStringDriver {
    type_name: String,
}

impl XmlMDataStdAsciiStringDriver {
    /// Create a new ASCII string driver.
    pub fn new() -> Self {
        XmlMDataStdAsciiStringDriver {
            type_name: "TDataStd_AsciiString".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty ASCII string attribute.
    pub fn new_empty(&self) -> String {
        String::new()
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self, value: &str) -> Result<String, String> {
        Ok(value.to_string())
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self, value: &str) -> Result<String, String> {
        Ok(value.to_string())
    }

    /// Validate ASCII string content.
    pub fn validate(&self, value: &str) -> bool {
        value.chars().all(|c| c.is_ascii())
    }
}

impl Default for XmlMDataStdAsciiStringDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_AsciiString");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        assert_eq!(driver.new_empty(), "");
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        let result = driver.paste_from_persistent("hello world");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        let result = driver.paste_to_persistent("test string");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test string");
    }

    #[test]
    fn test_validate_ascii() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        assert!(driver.validate("simple ascii"));
        assert!(driver.validate("123"));
        assert!(driver.validate("Special!@#$%^&*()"));
    }

    #[test]
    fn test_validate_non_ascii() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        assert!(!driver.validate("café"));
        assert!(!driver.validate("日本語"));
        assert!(!driver.validate("emoji 😀"));
    }

    #[test]
    fn test_validate_empty() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        assert!(driver.validate(""));
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataStdAsciiStringDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_AsciiString");
    }

    #[test]
    fn test_roundtrip_simple_string() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        let original = "test value";
        let persistent = driver.paste_to_persistent(original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert_eq!(transient, original);
    }

    #[test]
    fn test_roundtrip_empty_string() {
        let driver = XmlMDataStdAsciiStringDriver::new();
        let original = "";
        let persistent = driver.paste_to_persistent(original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert_eq!(transient, original);
    }
}
