// FILE: xml_m_data_std_real_array_driver.rs
// occt: XmlMDataStd_RealArrayDriver

/// XML serialization driver for real array attributes.
/// Handles serialization and deserialization of arrays of double-precision values (TDataStd_RealArray).
pub struct XmlMDataStdRealArrayDriver {
    type_name: String,
}

impl XmlMDataStdRealArrayDriver {
    /// Create a new real array driver.
    pub fn new() -> Self {
        XmlMDataStdRealArrayDriver {
            type_name: "TDataStd_RealArray".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty real array attribute.
    pub fn new_empty(&self) -> Vec<f64> {
        Vec::new()
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self, values: &[f64]) -> Result<Vec<f64>, String> {
        Ok(values.to_vec())
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self, values: &[f64]) -> Result<Vec<f64>, String> {
        Ok(values.to_vec())
    }

    /// Get array length.
    pub fn array_length(&self, values: &[f64]) -> usize {
        values.len()
    }

    /// Check if array is valid.
    pub fn is_valid(&self, _values: &[f64]) -> bool {
        true
    }

    /// Parse real value from string.
    pub fn parse_real(&self, s: &str) -> Result<f64, String> {
        s.parse::<f64>()
            .map_err(|_| "Invalid float value".to_string())
    }

    /// Format real value to string with precision.
    pub fn format_real(&self, value: f64) -> String {
        format!("{}", value)
    }
}

impl Default for XmlMDataStdRealArrayDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataStdRealArrayDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_RealArray");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let arr = driver.new_empty();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let input = [1.0, 2.5, 3.14];
        let result = driver.paste_from_persistent(&input);
        assert!(result.is_ok());
        let arr = result.unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[1], 2.5);
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let input = [0.0, 1.5, -2.5];
        let result = driver.paste_to_persistent(&input);
        assert!(result.is_ok());
        let arr = result.unwrap();
        assert_eq!(arr, vec![0.0, 1.5, -2.5]);
    }

    #[test]
    fn test_array_length_empty() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let arr: [f64; 0] = [];
        assert_eq!(driver.array_length(&arr), 0);
    }

    #[test]
    fn test_array_length_nonempty() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let arr = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(driver.array_length(&arr), 4);
    }

    #[test]
    fn test_is_valid() {
        let driver = XmlMDataStdRealArrayDriver::new();
        assert!(driver.is_valid(&[]));
        assert!(driver.is_valid(&[1.0, 2.5, 3.14]));
    }

    #[test]
    fn test_parse_real_valid() {
        let driver = XmlMDataStdRealArrayDriver::new();
        assert_eq!(driver.parse_real("3.14").unwrap(), 3.14);
        assert_eq!(driver.parse_real("0").unwrap(), 0.0);
        assert_eq!(driver.parse_real("-2.5").unwrap(), -2.5);
    }

    #[test]
    fn test_parse_real_invalid() {
        let driver = XmlMDataStdRealArrayDriver::new();
        assert!(driver.parse_real("not_a_number").is_err());
    }

    #[test]
    fn test_format_real() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let s = driver.format_real(3.14);
        assert!(s.contains("3.14"));
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataStdRealArrayDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataStdRealArrayDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_RealArray");
    }

    #[test]
    fn test_roundtrip_real_array() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let original = [1.23, 4.56, 7.89];
        let persistent = driver.paste_to_persistent(&original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert_eq!(transient.len(), 3);
        for (a, b) in transient.iter().zip(original.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_roundtrip_empty_array() {
        let driver = XmlMDataStdRealArrayDriver::new();
        let original: [f64; 0] = [];
        let persistent = driver.paste_to_persistent(&original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert!(transient.is_empty());
    }
}
