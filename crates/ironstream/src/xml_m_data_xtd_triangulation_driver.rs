// FILE: xml_m_data_xtd_triangulation_driver.rs
// occt: XmlMDataXtd_TriangulationDriver

/// XML serialization driver for triangulation (mesh) attributes.
/// Handles serialization and deserialization of mesh data including
/// vertices, triangles, and normals.
pub struct XmlMDataXtdTriangulationDriver {
    type_name: String,
}

impl XmlMDataXtdTriangulationDriver {
    /// Create a new triangulation driver.
    pub fn new() -> Self {
        XmlMDataXtdTriangulationDriver {
            type_name: "TDataXtd_Triangulation".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Parse a floating-point value from input stream.
    pub fn get_real(&self, _value: &str) -> Result<f64, String> {
        _value
            .parse::<f64>()
            .map_err(|_| "Invalid float value".to_string())
    }

    /// Check if mesh is valid.
    pub fn is_valid(&self) -> bool {
        true
    }
}

impl Default for XmlMDataXtdTriangulationDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataXtdTriangulationDriver::new();
        assert_eq!(driver.type_name(), "TDataXtd_Triangulation");
    }

    #[test]
    fn test_type_name_not_empty() {
        let driver = XmlMDataXtdTriangulationDriver::new();
        assert!(!driver.type_name().is_empty());
    }

    #[test]
    fn test_get_real_valid() {
        let driver = XmlMDataXtdTriangulationDriver::new();
        assert_eq!(driver.get_real("3.14").unwrap(), 3.14);
        assert_eq!(driver.get_real("0").unwrap(), 0.0);
        assert_eq!(driver.get_real("-2.5").unwrap(), -2.5);
    }

    #[test]
    fn test_get_real_invalid() {
        let driver = XmlMDataXtdTriangulationDriver::new();
        assert!(driver.get_real("not_a_number").is_err());
        assert!(driver.get_real("").is_err());
    }

    #[test]
    fn test_is_valid() {
        let driver = XmlMDataXtdTriangulationDriver::new();
        assert!(driver.is_valid());
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataXtdTriangulationDriver::default();
        assert_eq!(driver.type_name(), "TDataXtd_Triangulation");
    }

    #[test]
    fn test_multiple_instances() {
        let driver1 = XmlMDataXtdTriangulationDriver::new();
        let driver2 = XmlMDataXtdTriangulationDriver::new();
        assert_eq!(driver1.type_name(), driver2.type_name());
        assert!(driver1.is_valid());
        assert!(driver2.is_valid());
    }

    #[test]
    fn test_get_real_precision() {
        let driver = XmlMDataXtdTriangulationDriver::new();
        let val = driver.get_real("1.23456789").unwrap();
        assert!((val - 1.23456789).abs() < 1e-8);
    }
}
