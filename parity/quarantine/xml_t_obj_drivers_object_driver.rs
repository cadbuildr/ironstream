// FILE: xml_t_obj_drivers_object_driver.rs
// occt: XmlTObjDrivers_ObjectDriver

use std::collections::HashMap;

/// XML driver for TObj Object persistence.
/// Handles serialization/deserialization of individual transient objects,
/// their attributes, and relationships.
pub struct XmlTObjDriversObjectDriver {
    version: i32,
}

impl XmlTObjDriversObjectDriver {
    /// Create a new object driver.
    pub fn new() -> Self {
        XmlTObjDriversObjectDriver { version: 1 }
    }

    /// Get the driver version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Write a TObj Object to XML with attributes.
    pub fn write_object(
        &self,
        object_id: &str,
        object_type: &str,
        attributes: &HashMap<String, String>,
    ) -> String {
        let mut xml = format!("<Object id=\"{}\" type=\"{}\">", object_id, object_type);
        for (key, value) in attributes.iter() {
            xml.push_str(&format!("\n  <Attr name=\"{}\" value=\"{}\"/>", key, value));
        }
        xml.push_str("\n</Object>");
        xml
    }

    /// Read a TObj Object from XML.
    /// Returns (object_id, object_type, attributes_map).
    pub fn read_object(&self, xml: &str) -> Result<(String, String, HashMap<String, String>), String> {
        let mut id = String::new();
        let mut obj_type = String::new();
        let mut attrs = HashMap::new();

        for line in xml.lines() {
            if line.contains("<Object") {
                for part in line.split_whitespace() {
                    if let Some(i) = part.strip_prefix("id=\"").and_then(|s| s.strip_suffix("\"")) {
                        id = i.to_string();
                    } else if let Some(t) = part.strip_prefix("type=\"").and_then(|s| s.strip_suffix("\"")) {
                        obj_type = t.to_string();
                    }
                }
            } else if line.contains("<Attr") {
                let mut attr_name = String::new();
                let mut attr_value = String::new();
                for part in line.split_whitespace() {
                    if let Some(n) = part.strip_prefix("name=\"").and_then(|s| s.strip_suffix("\"")) {
                        attr_name = n.to_string();
                    } else if let Some(v) = part.strip_prefix("value=\"").and_then(|s| s.strip_suffix("\"")) {
                        attr_value = v.to_string();
                    }
                }
                if !attr_name.is_empty() {
                    attrs.insert(attr_name, attr_value);
                }
            }
        }

        if id.is_empty() {
            return Err("Missing object ID".to_string());
        }
        if obj_type.is_empty() {
            return Err("Missing object type".to_string());
        }

        Ok((id, obj_type, attrs))
    }

    /// Add or update an attribute on an object.
    pub fn set_attribute(&self, attributes: &mut HashMap<String, String>, key: &str, value: &str) -> Result<(), String> {
        if key.is_empty() {
            return Err("Empty attribute key".to_string());
        }
        attributes.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Get an attribute value from the map.
    pub fn get_attribute(&self, attributes: &HashMap<String, String>, key: &str) -> Option<String> {
        attributes.get(key).cloned()
    }

    /// Count attributes on an object.
    pub fn count_attributes(&self, attributes: &HashMap<String, String>) -> usize {
        attributes.len()
    }

    /// Validate object structure: must have ID and type.
    pub fn validate_object(&self, object_id: &str, object_type: &str) -> Result<(), String> {
        if object_id.is_empty() {
            return Err("Empty object ID".to_string());
        }
        if object_type.is_empty() {
            return Err("Empty object type".to_string());
        }
        Ok(())
    }
}

impl Default for XmlTObjDriversObjectDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_version() {
        let driver = XmlTObjDriversObjectDriver::new();
        assert_eq!(driver.version(), 1);
    }

    #[test]
    fn test_write_object_no_attributes() {
        let driver = XmlTObjDriversObjectDriver::new();
        let attrs = HashMap::new();
        let xml = driver.write_object("obj_1", "Part", &attrs);
        assert!(xml.contains("id=\"obj_1\""));
        assert!(xml.contains("type=\"Part\""));
    }

    #[test]
    fn test_write_object_with_attributes() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut attrs = HashMap::new();
        attrs.insert("color".to_string(), "red".to_string());
        attrs.insert("size".to_string(), "large".to_string());

        let xml = driver.write_object("obj_2", "Assembly", &attrs);
        assert!(xml.contains("id=\"obj_2\""));
        assert!(xml.contains("type=\"Assembly\""));
        assert!(xml.contains("color"));
        assert!(xml.contains("size"));
    }

    #[test]
    fn test_read_object_valid() {
        let driver = XmlTObjDriversObjectDriver::new();
        let xml = "<Object id=\"test_1\" type=\"Component\">\n  <Attr name=\"label\" value=\"Main\"/>\n</Object>";
        let result = driver.read_object(xml);
        assert!(result.is_ok());

        let (id, obj_type, attrs) = result.unwrap();
        assert_eq!(id, "test_1");
        assert_eq!(obj_type, "Component");
        assert_eq!(attrs.get("label"), Some(&"Main".to_string()));
    }

    #[test]
    fn test_read_object_missing_id() {
        let driver = XmlTObjDriversObjectDriver::new();
        let xml = "<Object type=\"Component\"/>";
        let result = driver.read_object(xml);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing object ID");
    }

    #[test]
    fn test_read_object_missing_type() {
        let driver = XmlTObjDriversObjectDriver::new();
        let xml = "<Object id=\"test_1\"/>";
        let result = driver.read_object(xml);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing object type");
    }

    #[test]
    fn test_set_attribute_valid() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut attrs = HashMap::new();
        let result = driver.set_attribute(&mut attrs, "color", "blue");
        assert!(result.is_ok());
        assert_eq!(attrs.get("color"), Some(&"blue".to_string()));
    }

    #[test]
    fn test_set_attribute_empty_key() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut attrs = HashMap::new();
        let result = driver.set_attribute(&mut attrs, "", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_attribute_exists() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut attrs = HashMap::new();
        attrs.insert("key1".to_string(), "val1".to_string());
        assert_eq!(driver.get_attribute(&attrs, "key1"), Some("val1".to_string()));
    }

    #[test]
    fn test_get_attribute_not_exists() {
        let driver = XmlTObjDriversObjectDriver::new();
        let attrs = HashMap::new();
        assert_eq!(driver.get_attribute(&attrs, "missing"), None);
    }

    #[test]
    fn test_count_attributes() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut attrs = HashMap::new();
        assert_eq!(driver.count_attributes(&attrs), 0);
        attrs.insert("a".to_string(), "1".to_string());
        attrs.insert("b".to_string(), "2".to_string());
        assert_eq!(driver.count_attributes(&attrs), 2);
    }

    #[test]
    fn test_validate_object_valid() {
        let driver = XmlTObjDriversObjectDriver::new();
        assert!(driver.validate_object("id_123", "Type").is_ok());
    }

    #[test]
    fn test_validate_object_empty_id() {
        let driver = XmlTObjDriversObjectDriver::new();
        assert!(driver.validate_object("", "Type").is_err());
    }

    #[test]
    fn test_validate_object_empty_type() {
        let driver = XmlTObjDriversObjectDriver::new();
        assert!(driver.validate_object("id_123", "").is_err());
    }

    #[test]
    fn test_roundtrip_object() {
        let driver = XmlTObjDriversObjectDriver::new();
        let mut orig_attrs = HashMap::new();
        orig_attrs.insert("param1".to_string(), "value1".to_string());

        let xml = driver.write_object("roundtrip_obj", "TestType", &orig_attrs);
        let (id, obj_type, attrs) = driver.read_object(&xml).unwrap();

        assert_eq!(id, "roundtrip_obj");
        assert_eq!(obj_type, "TestType");
        assert_eq!(attrs.get("param1"), Some(&"value1".to_string()));
    }
}
