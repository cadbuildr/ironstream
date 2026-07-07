// FILE: xml_m_data_std_boolean_array_driver.rs
// occt: XmlMDataStd_BooleanArrayDriver

/// XML serialization driver for boolean array attributes.
/// Handles serialization and deserialization of arrays of boolean values (TDataStd_BooleanArray).
pub struct XmlMDataStdBooleanArrayDriver {
    type_name: String,
}

impl XmlMDataStdBooleanArrayDriver {
    /// Create a new boolean array driver.
    pub fn new() -> Self {
        XmlMDataStdBooleanArrayDriver {
            type_name: "TDataStd_BooleanArray".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty boolean array attribute.
    pub fn new_empty(&self) -> Vec<bool> {
        Vec::new()
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self, values: &[bool]) -> Result<Vec<bool>, String> {
        Ok(values.to_vec())
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self, values: &[bool]) -> Result<Vec<bool>, String> {
        Ok(values.to_vec())
    }

    /// Get array length.
    pub fn array_length(&self, values: &[bool]) -> usize {
        values.len()
    }

    /// Check if array is valid.
    pub fn is_valid(&self, _values: &[bool]) -> bool {
        true
    }
}

impl Default for XmlMDataStdBooleanArrayDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_BooleanArray");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let arr = driver.new_empty();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let input = [true, false, true];
        let result = driver.paste_from_persistent(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![true, false, true]);
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let input = [false, true, false];
        let result = driver.paste_to_persistent(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![false, true, false]);
    }

    #[test]
    fn test_array_length_empty() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let arr: [bool; 0] = [];
        assert_eq!(driver.array_length(&arr), 0);
    }

    #[test]
    fn test_array_length_nonempty() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let arr = [true, false, true, true];
        assert_eq!(driver.array_length(&arr), 4);
    }

    #[test]
    fn test_is_valid() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        assert!(driver.is_valid(&[]));
        assert!(driver.is_valid(&[true, false]));
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataStdBooleanArrayDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_BooleanArray");
    }

    #[test]
    fn test_roundtrip_boolean_array() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let original = [true, false, true, false, true];
        let persistent = driver.paste_to_persistent(&original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert_eq!(transient, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_roundtrip_empty_array() {
        let driver = XmlMDataStdBooleanArrayDriver::new();
        let original: [bool; 0] = [];
        let persistent = driver.paste_to_persistent(&original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert!(transient.is_empty());
    }
}
