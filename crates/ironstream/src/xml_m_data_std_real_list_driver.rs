// FILE: xml_m_data_std_real_list_driver.rs
// occt: XmlMDataStd_RealListDriver

//! XML serialization driver for TDataStd_RealList.
//! Handles bidirectional conversion between TDataStd_RealList objects and XML representation.

use std::collections::HashMap;

/// XML Driver for serializing/deserializing TDataStd_RealList to/from XML.
///
/// This driver converts between XML persistent representation and in-memory
/// TDataStd_RealList objects. It handles:
/// - Reading real list data from XML attributes and text content
/// - Writing real list data to XML with proper formatting
/// - GUID attribute management
pub struct XmlMDataStdRealListDriver {
    message_driver: Option<String>, // Simplified message messenger
}

impl XmlMDataStdRealListDriver {
    /// Construct the driver with a message messenger.
    pub fn new(message_driver: Option<String>) -> Self {
        Self { message_driver }
    }

    /// Create a new empty TDataStd_RealList instance.
    pub fn new_empty(&self) -> Vec<f64> {
        Vec::new()
    }

    /// Paste (deserialize) from XML to TDataStd_RealList.
    ///
    /// Reads:
    /// - "first" attribute: first index (defaults to 1)
    /// - "last" attribute: last index (required)
    /// - text content: space-separated real values
    /// - "reallistattguid" attribute: optional custom GUID
    ///
    /// Returns the real list and status (true if successful).
    pub fn paste_from_xml(
        &self,
        attributes: &HashMap<String, String>,
        text_content: &str,
    ) -> (Vec<f64>, bool) {
        let mut real_list = Vec::new();

        // Get first and last indices
        let first_idx = attributes
            .get("first")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(1);

        let last_idx = match attributes.get("last").and_then(|s| s.parse::<i32>().ok()) {
            Some(idx) => idx,
            None => {
                self.log_error("Cannot retrieve the last index for RealList attribute");
                return (real_list, false);
            }
        };

        // Handle empty list case
        if last_idx == 0 {
            return (real_list, true);
        }

        // Parse real values from text content
        let values: Vec<&str> = text_content.split_whitespace().collect();
        let expected_count = (last_idx - first_idx + 1) as usize;

        if values.len() != expected_count {
            self.log_error(&format!(
                "RealList expects {} values but got {}",
                expected_count,
                values.len()
            ));
            return (real_list, false);
        }

        for val_str in values {
            match val_str.parse::<f64>() {
                Ok(val) => real_list.push(val),
                Err(_) => {
                    self.log_error(&format!("Cannot parse real value: {}", val_str));
                    return (real_list, false);
                }
            }
        }

        (real_list, true)
    }

    /// Paste (serialize) from TDataStd_RealList to XML.
    ///
    /// Writes:
    /// - "last" attribute: count of elements
    /// - text content: space-separated real values, written with full
    ///   round-trip precision (OCCT uses %.17g; Rust's shortest round-trip
    ///   formatting preserves the exact same values)
    /// - "reallistattguid" attribute: custom GUID if not default
    pub fn paste_to_xml(
        &self,
        real_list: &[f64],
        custom_guid: Option<&str>,
    ) -> (HashMap<String, String>, String) {
        let mut attributes = HashMap::new();
        let mut text = String::new();

        let count = real_list.len() as i32;
        attributes.insert("last".to_string(), count.to_string());

        if !real_list.is_empty() {
            let parts: Vec<String> = real_list
                .iter()
                .map(|v| v.to_string())
                .collect();
            text = parts.join(" ");
        }

        if let Some(guid) = custom_guid {
            attributes.insert("reallistattguid".to_string(), guid.to_string());
        }

        (attributes, text)
    }

    fn log_error(&self, msg: &str) {
        if self.message_driver.is_some() {
            eprintln!("XmlMDataStdRealListDriver: {}", msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let empty = driver.new_empty();
        assert!(empty.is_empty());
    }

    #[test]
    fn test_paste_from_xml_simple() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let mut attrs = HashMap::new();
        attrs.insert("first".to_string(), "1".to_string());
        attrs.insert("last".to_string(), "3".to_string());

        let (list, ok) = driver.paste_from_xml(&attrs, "1.5 2.7 3.14");
        assert!(ok);
        assert_eq!(list.len(), 3);
        assert!((list[0] - 1.5).abs() < 1e-10);
        assert!((list[1] - 2.7).abs() < 1e-10);
        assert!((list[2] - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_paste_from_xml_empty() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let mut attrs = HashMap::new();
        attrs.insert("last".to_string(), "0".to_string());

        let (list, ok) = driver.paste_from_xml(&attrs, "");
        assert!(ok);
        assert!(list.is_empty());
    }

    #[test]
    fn test_paste_from_xml_missing_last() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let attrs = HashMap::new();

        let (list, ok) = driver.paste_from_xml(&attrs, "1.0");
        assert!(!ok);
        assert!(list.is_empty());
    }

    #[test]
    fn test_paste_to_xml() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let list = vec![1.5, 2.7, 3.14];

        let (attrs, text) = driver.paste_to_xml(&list, None);
        assert_eq!(attrs.get("last").unwrap(), "3");
        assert!(text.contains("1.5"));
        assert!(text.contains("2.7"));
        assert!(text.contains("3.14"));
    }

    #[test]
    fn test_paste_to_xml_with_guid() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let list = vec![1.0];
        let guid = "deadbeef-cafe-babe-cafe-deadbeefcafe";

        let (attrs, _text) = driver.paste_to_xml(&list, Some(guid));
        assert_eq!(attrs.get("reallistattguid").unwrap(), guid);
    }

    #[test]
    fn test_roundtrip_precision() {
        let driver = XmlMDataStdRealListDriver::new(None);
        let original = vec![1.23456789012345, -9.8765432109876e-10, 3.14159265358979];

        let (attrs, text) = driver.paste_to_xml(&original, None);
        let (recovered, ok) = driver.paste_from_xml(&attrs, &text);

        assert!(ok);
        assert_eq!(recovered.len(), original.len());
        for (orig, recov) in original.iter().zip(recovered.iter()) {
            assert!((orig - recov).abs() < 1e-15);
        }
    }
}
