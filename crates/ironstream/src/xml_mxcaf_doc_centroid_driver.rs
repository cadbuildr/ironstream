// FILE: xml_mxcaf_doc_centroid_driver.rs
// occt: XmlMXCAFDoc_CentroidDriver
//
// Faithful port of OCCT XmlMXCAFDoc_CentroidDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_CentroidDriver.hxx),
// the XmlMDF_ADriver for XCAF centroid attributes.
// Serializes/deserializes XCAFDoc_Centroid (center-of-mass) data: a 3D point (X,Y,Z).

/// Local model of centroid data (3D point).
#[derive(Debug, Clone, PartialEq)]
pub struct CentroidData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CentroidData {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// XmlMDF_ADriver for centroid attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocCentroidDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocCentroidDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Centroid";

    pub fn new() -> Self {
        Self {
            type_name: Self::TYPE_NAME.to_string(),
            version: 1,
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn version_number(&self) -> u32 {
        self.version
    }

    /// Read centroid from XML element text.
    /// Format: "x y z" (space-separated floats).
    pub fn read_from_xml(&self, element_text: &str) -> Result<CentroidData, String> {
        let parts: Result<Vec<f64>, _> = element_text
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();

        match parts {
            Ok(coords) if coords.len() == 3 => Ok(CentroidData::new(coords[0], coords[1], coords[2])),
            Ok(_) => Err(format!(
                "Centroid requires 3 coordinates, got {}",
                element_text.split_whitespace().count()
            )),
            Err(e) => Err(format!("Failed to parse coordinate: {}", e)),
        }
    }

    /// Write centroid to XML element text.
    pub fn write_to_xml(&self, data: &CentroidData) -> String {
        format!("{} {} {}", data.x, data.y, data.z)
    }
}

impl Default for XmlMXCAFDocCentroidDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centroid_data_new() {
        let c = CentroidData::new(1.0, 2.0, 3.0);
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 2.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn test_centroid_data_zero() {
        let c = CentroidData::zero();
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 0.0);
        assert_eq!(c.z, 0.0);
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocCentroidDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Centroid");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_valid() {
        let driver = XmlMXCAFDocCentroidDriver::new();
        let result = driver.read_from_xml("10.5 20.3 30.1");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!((data.x - 10.5).abs() < 1e-10);
        assert!((data.y - 20.3).abs() < 1e-10);
        assert!((data.z - 30.1).abs() < 1e-10);
    }

    #[test]
    fn test_read_from_xml_invalid_count() {
        let driver = XmlMXCAFDocCentroidDriver::new();
        let result = driver.read_from_xml("1.0 2.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_invalid_number() {
        let driver = XmlMXCAFDocCentroidDriver::new();
        let result = driver.read_from_xml("1.0 not_a_number 3.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocCentroidDriver::new();
        let data = CentroidData::new(5.5, 10.2, 15.8);
        let xml_text = driver.write_to_xml(&data);
        assert_eq!(xml_text, "5.5 10.2 15.8");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocCentroidDriver::new();
        let original = CentroidData::new(1.234, 5.678, 9.012);
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert!((original.x - restored.x).abs() < 1e-10);
        assert!((original.y - restored.y).abs() < 1e-10);
        assert!((original.z - restored.z).abs() < 1e-10);
    }
}
