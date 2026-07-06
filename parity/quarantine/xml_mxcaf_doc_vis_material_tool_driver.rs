// FILE: xml_mxcaf_doc_vis_material_tool_driver.rs
// occt: XmlMXCAFDoc_VisMaterialToolDriver

/// XML persistence driver for visual material tool attributes in XCAF documents.
/// Handles serialization/deserialization of material properties (diffuse, specular, etc.)
/// to/from XML storage.
pub struct XmlMXCAFDocVisMaterialToolDriver {
    driver_version: i32,
}

impl XmlMXCAFDocVisMaterialToolDriver {
    /// Create a new material tool driver instance.
    pub fn new() -> Self {
        XmlMXCAFDocVisMaterialToolDriver {
            driver_version: 1,
        }
    }

    /// Get the driver version.
    pub fn driver_version(&self) -> i32 {
        self.driver_version
    }

    /// Write material attributes to XML element.
    /// Encodes: material name, diffuse color (R,G,B,A), specular color, shininess, etc.
    pub fn write_to_xml(&self, material_name: &str, diffuse: [f64; 4], specular: [f64; 3], shininess: f64) -> String {
        format!(
            "<Material name=\"{}\" diffuse_r=\"{}\" diffuse_g=\"{}\" diffuse_b=\"{}\" diffuse_a=\"{}\" spec_r=\"{}\" spec_g=\"{}\" spec_b=\"{}\" shininess=\"{}\"/>",
            material_name, diffuse[0], diffuse[1], diffuse[2], diffuse[3],
            specular[0], specular[1], specular[2], shininess
        )
    }

    /// Read material attributes from XML element string.
    /// Returns (material_name, diffuse, specular, shininess) or empty defaults.
    pub fn read_from_xml(&self, xml_str: &str) -> (String, [f64; 4], [f64; 3], f64) {
        let mut name = String::new();
        let mut diffuse = [0.8, 0.8, 0.8, 1.0];
        let mut specular = [0.0, 0.0, 0.0];
        let mut shininess = 0.0;

        for part in xml_str.split_whitespace() {
            if let Some(val) = part.strip_prefix("name=\"").and_then(|s| s.strip_suffix("\"")) {
                name = val.to_string();
            } else if let Some(val) = part.strip_prefix("diffuse_r=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { diffuse[0] = f; }
            } else if let Some(val) = part.strip_prefix("diffuse_g=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { diffuse[1] = f; }
            } else if let Some(val) = part.strip_prefix("diffuse_b=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { diffuse[2] = f; }
            } else if let Some(val) = part.strip_prefix("diffuse_a=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { diffuse[3] = f; }
            } else if let Some(val) = part.strip_prefix("spec_r=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { specular[0] = f; }
            } else if let Some(val) = part.strip_prefix("spec_g=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { specular[1] = f; }
            } else if let Some(val) = part.strip_prefix("spec_b=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { specular[2] = f; }
            } else if let Some(val) = part.strip_prefix("shininess=\"").and_then(|s| s.strip_suffix("\"")) {
                if let Ok(f) = val.parse::<f64>() { shininess = f; }
            }
        }

        (name, diffuse, specular, shininess)
    }
}

impl Default for XmlMXCAFDocVisMaterialToolDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_write_read_roundtrip() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        let orig_name = "RedPlastic";
        let orig_diffuse = [0.9, 0.1, 0.1, 1.0];
        let orig_specular = [0.3, 0.3, 0.3];
        let orig_shininess = 32.0;

        let xml = driver.write_to_xml(orig_name, orig_diffuse, orig_specular, orig_shininess);
        let (name, diffuse, specular, shininess) = driver.read_from_xml(&xml);

        assert_eq!(name, orig_name);
        assert_eq!(diffuse, orig_diffuse);
        assert_eq!(specular, orig_specular);
        assert_eq!(shininess, orig_shininess);
    }

    #[test]
    fn test_material_default_values() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        let (name, diffuse, specular, shininess) = driver.read_from_xml("<Material/>");
        assert_eq!(name, "");
        assert_eq!(diffuse, [0.8, 0.8, 0.8, 1.0]);
        assert_eq!(specular, [0.0, 0.0, 0.0]);
        assert_eq!(shininess, 0.0);
    }

    #[test]
    fn test_driver_version() {
        let driver = XmlMXCAFDocVisMaterialToolDriver::new();
        assert_eq!(driver.driver_version(), 1);
    }
}
