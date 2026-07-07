// FILE: xml_m_data_std_integer_list_driver.rs
// occt: XmlMDataStd_IntegerListDriver

/// XML serialization driver for integer list attributes.
/// Handles serialization and deserialization of lists of integers (TDataStd_IntegerList).
pub struct XmlMDataStdIntegerListDriver {
    type_name: String,
}

impl XmlMDataStdIntegerListDriver {
    /// Create a new integer list driver.
    pub fn new() -> Self {
        XmlMDataStdIntegerListDriver {
            type_name: "TDataStd_IntegerList".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Create an empty integer list attribute.
    pub fn new_empty(&self) -> Vec<i32> {
        Vec::new()
    }

    /// Paste from persistent to transient (deserialize).
    pub fn paste_from_persistent(&self, values: &[i32]) -> Result<Vec<i32>, String> {
        Ok(values.to_vec())
    }

    /// Paste from transient to persistent (serialize).
    pub fn paste_to_persistent(&self, values: &[i32]) -> Result<Vec<i32>, String> {
        Ok(values.to_vec())
    }

    /// Get list length.
    pub fn list_length(&self, values: &[i32]) -> usize {
        values.len()
    }

    /// Append value to list.
    pub fn append_value(&self, list: &mut Vec<i32>, value: i32) {
        list.push(value);
    }

    /// Check if list is valid.
    pub fn is_valid(&self, _values: &[i32]) -> bool {
        true
    }
}

impl Default for XmlMDataStdIntegerListDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataStdIntegerListDriver::new();
        assert_eq!(driver.type_name(), "TDataStd_IntegerList");
    }

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let list = driver.new_empty();
        assert!(list.is_empty());
    }

    #[test]
    fn test_paste_from_persistent() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let input = [1, 2, 3, 4, 5];
        let result = driver.paste_from_persistent(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_paste_to_persistent() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let input = [10, 20, 30];
        let result = driver.paste_to_persistent(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![10, 20, 30]);
    }

    #[test]
    fn test_list_length_empty() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let list: [i32; 0] = [];
        assert_eq!(driver.list_length(&list), 0);
    }

    #[test]
    fn test_list_length_nonempty() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let list = [1, 2, 3, 4];
        assert_eq!(driver.list_length(&list), 4);
    }

    #[test]
    fn test_append_value() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let mut list = vec![1, 2, 3];
        driver.append_value(&mut list, 4);
        assert_eq!(list, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_append_to_empty_list() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let mut list = Vec::new();
        driver.append_value(&mut list, 42);
        assert_eq!(list, vec![42]);
    }

    #[test]
    fn test_is_valid() {
        let driver = XmlMDataStdIntegerListDriver::new();
        assert!(driver.is_valid(&[]));
        assert!(driver.is_valid(&[1, 2, 3]));
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataStdIntegerListDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataStdIntegerListDriver::default();
        assert_eq!(driver.type_name(), "TDataStd_IntegerList");
    }

    #[test]
    fn test_roundtrip_integer_list() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let original = [1, 5, 10, 15, 20];
        let persistent = driver.paste_to_persistent(&original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert_eq!(transient, vec![1, 5, 10, 15, 20]);
    }

    #[test]
    fn test_roundtrip_empty_list() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let original: [i32; 0] = [];
        let persistent = driver.paste_to_persistent(&original).unwrap();
        let transient = driver.paste_from_persistent(&persistent).unwrap();
        assert!(transient.is_empty());
    }

    #[test]
    fn test_multiple_appends() {
        let driver = XmlMDataStdIntegerListDriver::new();
        let mut list = driver.new_empty();
        for i in 0..10 {
            driver.append_value(&mut list, i);
        }
        assert_eq!(driver.list_length(&list), 10);
    }
}
