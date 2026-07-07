// FILE: xml_m_data_std_real_driver.rs
// occt: XmlMDataStd_RealDriver

/// XML serialization driver for real attributes.
/// Handles serialization and deserialization of double-precision scalar values (TDataStd_Real).
pub struct XmlMDataStdRealDriver {
    type_name: String,
}

impl XmlMDataStdRealDriver {
    /// Create a new real driver.
    pub fn new() -> Self {
        XmlMDataStdRealDriver {
            type_name: "TDataStd_Real".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty real attribute (0.0).
    pub fn new_empty(&self) -> f64 {
        0.0
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self, value: f64) -> Result<f64, String> {
        Ok(value)
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self, value: f64) -> Result<f64, String> {
        Ok(value)
    }

    /// Parse real value from string.
    pub fn parse_real(&self, s: &str) -> Result<f64, String> {
        s.parse::<f64>()
            .map_err(|_| "Invalid float value".to_string())
    }

    /// Convert real to string.
    pub fn real_to_string(&self, value: f64) -> String {
        value.to_string()
    }

    /// Check if value is finite.
    pub fn is_finite(&self, value: f64) -> bool {
        value.is_finite()
    }
}

impl Default for XmlMDataStdRealDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataStdRealDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_Real");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdRealDriver::new();
        assert_eq!(driver.new_empty(), 0.0);
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDataStdRealDriver::new();
        assert_eq!(driver.paste_from_persistent(3.14).unwrap(), 3.14);
        assert_eq!(driver.paste_from_persistent(-2.5).unwrap(), -2.5);
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDataStdRealDriver::new();
        assert_eq!(driver.paste_to_persistent(1.23).unwrap(), 1.23);
        assert_eq!(driver.paste_to_persistent(0.0).unwrap(), 0.0);
    }

    #[test]
    fn test_parse_real_valid() {
        let driver = XmlMDataStdRealDriver::new();
        assert_eq!(driver.parse_real("3.14").unwrap(), 3.14);
        assert_eq!(driver.parse_real("-123.456").unwrap(), -123.456);
        assert_eq!(driver.parse_real("0").unwrap(), 0.0);
        assert_eq!(driver.parse_real("1e-5").unwrap(), 1e-5);
    }

    #[test]
    fn test_parse_real_invalid() {
        let driver = XmlMDataStdRealDriver::new();
        assert!(driver.parse_real("not_a_number").is_err());
        assert!(driver.parse_real("").is_err());
    }

    #[test]
    fn test_real_to_string() {
        let driver = XmlMDataStdRealDriver::new();
        let s = driver.real_to_string(3.14);
        assert!(s.contains("3.14"));
    }

    #[test]
    fn test_is_finite() {
        let driver = XmlMDataStdRealDriver::new();
        assert!(driver.is_finite(3.14));
        assert!(driver.is_finite(-2.5));
        assert!(driver.is_finite(0.0));
        assert!(!driver.is_finite(f64::INFINITY));
        assert!(!driver.is_finite(f64::NEG_INFINITY));
        assert!(!driver.is_finite(f64::NAN));
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataStdRealDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataStdRealDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_Real");
    }

    #[test]
    fn test_roundtrip_real() {
        let driver = XmlMDataStdRealDriver::new();
        let original = 3.141592653589793;
        let persistent = driver.paste_to_persistent(original).unwrap();
        let transient = driver.paste_from_persistent(persistent).unwrap();
        assert_eq!(transient, original);
    }

    #[test]
    fn test_parse_and_convert() {
        let driver = XmlMDataStdRealDriver::new();
        let s = "2.718";
        let parsed = driver.parse_real(s).unwrap();
        let converted = driver.real_to_string(parsed);
        let reparsed = driver.parse_real(&converted).unwrap();
        assert!((parsed - reparsed).abs() < 1e-10);
    }

    #[test]
    fn test_large_and_small_values() {
        let driver = XmlMDataStdRealDriver::new();
        assert!(driver.is_finite(1e100));
        assert!(driver.is_finite(1e-100));
        assert_eq!(driver.paste_from_persistent(1e100).unwrap(), 1e100);
    }
}
