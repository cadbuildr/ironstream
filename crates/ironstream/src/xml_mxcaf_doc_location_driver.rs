// FILE: xml_mxcaf_doc_location_driver.rs
// occt: XmlMXCAFDoc_LocationDriver
//
// Faithful port of OCCT XmlMXCAFDoc_LocationDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_LocationDriver.hxx),
// the XmlMDF_ADriver for XCAF location attributes.
// Serializes/deserializes XCAFDoc_Location data (3D transformations: position and orientation).

/// Local model of a 3D location (position + orientation via Trsf matrix).
#[derive(Debug, Clone, PartialEq)]
pub struct LocationData {
    /// Translation: tx, ty, tz
    pub translation: (f64, f64, f64),
    /// Rotation matrix (row-major): 9 f64 values for 3x3 orientation
    pub rotation_matrix: [f64; 9],
    /// Scale (uniform)
    pub scale: f64,
}

impl LocationData {
    pub fn new(tx: f64, ty: f64, tz: f64) -> Self {
        Self {
            translation: (tx, ty, tz),
            rotation_matrix: [
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
            scale: 1.0,
        }
    }

    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

/// XmlMDF_ADriver for location attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocLocationDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocLocationDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Location";

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

    /// Read location from XML element text.
    /// Format: "tx ty tz r11 r12 r13 r21 r22 r23 r31 r32 r33 scale"
    /// (translation + rotation matrix + scale, 13 space-separated floats).
    pub fn read_from_xml(&self, element_text: &str) -> Result<LocationData, String> {
        let parts: Result<Vec<f64>, _> = element_text
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();

        match parts {
            Ok(values) if values.len() == 13 => {
                let mut matrix = [0.0; 9];
                for i in 0..9 {
                    matrix[i] = values[3 + i];
                }
                Ok(LocationData {
                    translation: (values[0], values[1], values[2]),
                    rotation_matrix: matrix,
                    scale: values[12],
                })
            }
            Ok(_) => Err(format!(
                "Location requires 13 values (tx ty tz + 9 rotation + scale), got {}",
                element_text.split_whitespace().count()
            )),
            Err(e) => Err(format!("Failed to parse coordinate: {}", e)),
        }
    }

    /// Write location to XML element text.
    pub fn write_to_xml(&self, data: &LocationData) -> String {
        let mut parts = vec![
            data.translation.0.to_string(),
            data.translation.1.to_string(),
            data.translation.2.to_string(),
        ];
        for &val in &data.rotation_matrix {
            parts.push(val.to_string());
        }
        parts.push(data.scale.to_string());
        parts.join(" ")
    }
}

impl Default for XmlMXCAFDocLocationDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_data_new() {
        let loc = LocationData::new(10.0, 20.0, 30.0);
        assert_eq!(loc.translation, (10.0, 20.0, 30.0));
        assert_eq!(loc.rotation_matrix[0], 1.0);
        assert_eq!(loc.scale, 1.0);
    }

    #[test]
    fn test_location_data_identity() {
        let loc = LocationData::identity();
        assert_eq!(loc.translation, (0.0, 0.0, 0.0));
        assert_eq!(loc.scale, 1.0);
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocLocationDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Location");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_identity() {
        let driver = XmlMXCAFDocLocationDriver::new();
        let xml = "0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 1.0";
        let result = driver.read_from_xml(xml);
        assert!(result.is_ok());
        let loc = result.unwrap();
        assert_eq!(loc.translation, (0.0, 0.0, 0.0));
        assert_eq!(loc.scale, 1.0);
    }

    #[test]
    fn test_read_from_xml_translated() {
        let driver = XmlMXCAFDocLocationDriver::new();
        let xml = "5.0 10.0 15.0 1.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 1.0";
        let result = driver.read_from_xml(xml);
        assert!(result.is_ok());
        let loc = result.unwrap();
        assert_eq!(loc.translation, (5.0, 10.0, 15.0));
    }

    #[test]
    fn test_read_from_xml_scaled() {
        let driver = XmlMXCAFDocLocationDriver::new();
        let xml = "0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 0.5";
        let result = driver.read_from_xml(xml);
        assert!(result.is_ok());
        let loc = result.unwrap();
        assert_eq!(loc.scale, 0.5);
    }

    #[test]
    fn test_read_from_xml_invalid_count() {
        let driver = XmlMXCAFDocLocationDriver::new();
        let xml = "0.0 0.0 0.0 1.0";
        assert!(driver.read_from_xml(xml).is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocLocationDriver::new();
        let loc = LocationData::new(5.0, 10.0, 15.0);
        let xml = driver.write_to_xml(&loc);
        let parts: Vec<&str> = xml.split_whitespace().collect();
        assert_eq!(parts.len(), 13);
        assert_eq!(parts[0], "5");
        assert_eq!(parts[12], "1");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocLocationDriver::new();
        let original = LocationData::new(100.5, 200.3, 300.7);
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert!((restored.translation.0 - original.translation.0).abs() < 1e-10);
        assert!((restored.translation.1 - original.translation.1).abs() < 1e-10);
        assert!((restored.translation.2 - original.translation.2).abs() < 1e-10);
        assert_eq!(restored.scale, original.scale);
    }
}
