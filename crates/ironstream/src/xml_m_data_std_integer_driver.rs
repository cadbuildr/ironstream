// FILE: xml_m_data_std_integer_driver.rs
// occt: XmlMDataStd_IntegerDriver

/// XML serialization driver for integer attributes.
/// Handles serialization and deserialization of integer scalar values (TDataStd_Integer).
pub struct XmlMDataStdIntegerDriver {
    type_name: String,
}

impl XmlMDataStdIntegerDriver {
    /// Create a new integer driver.
    pub fn new() -> Self {
        XmlMDataStdIntegerDriver {
            type_name: "TDataStd_Integer".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty integer attribute (0).
    pub fn new_empty(&self) -> i32 {
        0
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self, value: i32) -> Result<i32, String> {
        Ok(value)
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self, value: i32) -> Result<i32, String> {
        Ok(value)
    }

    /// Parse integer from string.
    pub fn parse_integer(&self, s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|_| "Invalid integer format".to_string())
    }

    /// Convert integer to string.
    pub fn integer_to_string(&self, value: i32) -> String {
        value.to_string()
    }
}

impl Default for XmlMDataStdIntegerDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_Integer");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert_eq!(driver.new_empty(), 0);
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert_eq!(driver.paste_from_persistent(42).unwrap(), 42);
        assert_eq!(driver.paste_from_persistent(-100).unwrap(), -100);
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert_eq!(driver.paste_to_persistent(123).unwrap(), 123);
        assert_eq!(driver.paste_to_persistent(0).unwrap(), 0);
    }

    #[test]
    fn test_parse_integer_valid() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert_eq!(driver.parse_integer("42").unwrap(), 42);
        assert_eq!(driver.parse_integer("-123").unwrap(), -123);
        assert_eq!(driver.parse_integer("0").unwrap(), 0);
    }

    #[test]
    fn test_parse_integer_invalid() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert!(driver.parse_integer("not_a_number").is_err());
        assert!(driver.parse_integer("12.34").is_err());
    }

    #[test]
    fn test_integer_to_string() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert_eq!(driver.integer_to_string(42), "42");
        assert_eq!(driver.integer_to_string(-100), "-100");
        assert_eq!(driver.integer_to_string(0), "0");
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataStdIntegerDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataStdIntegerDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_Integer");
    }

    #[test]
    fn test_roundtrip_integer() {
        let driver = XmlMDataStdIntegerDriver::new();
        let original = 12345;
        let persistent = driver.paste_to_persistent(original).unwrap();
        let transient = driver.paste_from_persistent(persistent).unwrap();
        assert_eq!(transient, original);
    }

    #[test]
    fn test_parse_and_convert() {
        let driver = XmlMDataStdIntegerDriver::new();
        let s = "999";
        let parsed = driver.parse_integer(s).unwrap();
        let converted = driver.integer_to_string(parsed);
        assert_eq!(converted, s);
    }

    #[test]
    fn test_large_integers() {
        let driver = XmlMDataStdIntegerDriver::new();
        let max_val = i32::MAX;
        let min_val = i32::MIN;
        assert_eq!(driver.paste_from_persistent(max_val).unwrap(), max_val);
        assert_eq!(driver.paste_from_persistent(min_val).unwrap(), min_val);
    }
}
