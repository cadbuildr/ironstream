// FILE: xml_t_obj_drivers_model_driver.rs
// occt: XmlTObjDrivers_ModelDriver

/// XML driver for TObj Model persistence.
/// Handles the top-level model structure: metadata, object tree, associations.
pub struct XmlTObjDriversModelDriver {
    version: i32,
}

impl XmlTObjDriversModelDriver {
    /// Create a new model driver.
    pub fn new() -> Self {
        XmlTObjDriversModelDriver { version: 1 }
    }

    /// Get the driver version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Write a TObj Model to XML representation.
    /// Encodes the root model structure, name, and metadata.
    pub fn write_model(&self, model_name: &str, object_count: usize) -> String {
        format!(
            "<Model name=\"{}\" version=\"{}\" objects=\"{}\"><!-- Root model element --></Model>",
            model_name, self.version, object_count
        )
    }

    /// Read a TObj Model from XML.
    /// Returns (model_name, object_count).
    pub fn read_model(&self, xml: &str) -> Result<(String, usize), String> {
        let mut name = String::new();
        let mut count = 0usize;

        for part in xml.split_whitespace() {
            if let Some(n) = part.strip_prefix("name=\"").and_then(|s| s.strip_suffix("\"")) {
                name = n.to_string();
            } else if let Some(c) = part.strip_prefix("objects=\"").and_then(|s| s.strip_suffix("\"")) {
                count = c.parse::<usize>().unwrap_or(0);
            }
        }

        if name.is_empty() {
            return Err("Missing model name".to_string());
        }
        Ok((name, count))
    }

    /// Register a child object in the model hierarchy.
    pub fn register_object(&self, parent_name: &str, child_name: &str, child_type: &str) -> Result<String, String> {
        if parent_name.is_empty() || child_name.is_empty() {
            return Err("Empty names not allowed".to_string());
        }
        Ok(format!(
            "<Object parent=\"{}\" name=\"{}\" type=\"{}\"/>",
            parent_name, child_name, child_type
        ))
    }

    /// Get the full model hierarchy as an indented XML structure.
    pub fn build_hierarchy(&self, model_name: &str, children: &[(&str, &str)]) -> String {
        let mut xml = format!("<Model name=\"{}\">", model_name);
        for (child_name, child_type) in children {
            xml.push_str(&format!("\n  <Object name=\"{}\" type=\"{}\"/>", child_name, child_type));
        }
        xml.push_str("\n</Model>");
        xml
    }

    /// Count total objects in a model XML structure.
    pub fn count_objects(&self, xml: &str) -> usize {
        xml.matches("<Object").count()
    }
}

impl Default for XmlTObjDriversModelDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_version() {
        let driver = XmlTObjDriversModelDriver::new();
        assert_eq!(driver.version(), 1);
    }

    #[test]
    fn test_write_model() {
        let driver = XmlTObjDriversModelDriver::new();
        let xml = driver.write_model("MyModel", 5);
        assert!(xml.contains("name=\"MyModel\""));
        assert!(xml.contains("objects=\"5\""));
    }

    #[test]
    fn test_read_model_valid() {
        let driver = XmlTObjDriversModelDriver::new();
        let xml = "<Model name=\"TestModel\" version=\"1\" objects=\"3\"/>";
        let result = driver.read_model(xml);
        assert!(result.is_ok());
        let (name, count) = result.unwrap();
        assert_eq!(name, "TestModel");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_read_model_missing_name() {
        let driver = XmlTObjDriversModelDriver::new();
        let xml = "<Model version=\"1\" objects=\"3\"/>";
        let result = driver.read_model(xml);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing model name");
    }

    #[test]
    fn test_register_object_valid() {
        let driver = XmlTObjDriversModelDriver::new();
        let result = driver.register_object("Root", "Child1", "Part");
        assert!(result.is_ok());
        let xml = result.unwrap();
        assert!(xml.contains("parent=\"Root\""));
        assert!(xml.contains("name=\"Child1\""));
        assert!(xml.contains("type=\"Part\""));
    }

    #[test]
    fn test_register_object_empty_parent() {
        let driver = XmlTObjDriversModelDriver::new();
        let result = driver.register_object("", "Child1", "Part");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_object_empty_child() {
        let driver = XmlTObjDriversModelDriver::new();
        let result = driver.register_object("Root", "", "Part");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_hierarchy_empty() {
        let driver = XmlTObjDriversModelDriver::new();
        let xml = driver.build_hierarchy("Root", &[]);
        assert!(xml.contains("<Model name=\"Root\">"));
        assert!(xml.contains("</Model>"));
    }

    #[test]
    fn test_build_hierarchy_with_children() {
        let driver = XmlTObjDriversModelDriver::new();
        let children = vec![("Object1", "Part"), ("Object2", "Assembly")];
        let xml = driver.build_hierarchy("Root", &children);
        assert!(xml.contains("name=\"Object1\""));
        assert!(xml.contains("name=\"Object2\""));
        assert!(xml.contains("type=\"Part\""));
        assert!(xml.contains("type=\"Assembly\""));
    }

    #[test]
    fn test_count_objects() {
        let driver = XmlTObjDriversModelDriver::new();
        let xml = "<Model><Object/><Object/><Object/></Model>";
        assert_eq!(driver.count_objects(xml), 3);
    }

    #[test]
    fn test_count_objects_zero() {
        let driver = XmlTObjDriversModelDriver::new();
        let xml = "<Model></Model>";
        assert_eq!(driver.count_objects(xml), 0);
    }

    #[test]
    fn test_roundtrip_model() {
        let driver = XmlTObjDriversModelDriver::new();
        let orig_xml = driver.write_model("RoundtripTest", 10);
        let (name, count) = driver.read_model(&orig_xml).unwrap();
        assert_eq!(name, "RoundtripTest");
        assert_eq!(count, 10);
    }
}
