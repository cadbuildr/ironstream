// FILE: xml_mxcaf_doc_dim_tol_driver.rs
// occt: XmlMXCAFDoc_DimTolDriver
//
// Faithful port of OCCT XmlMXCAFDoc_DimTolDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_DimTolDriver.hxx),
// the XmlMDF_ADriver for XCAF dimension and tolerance attributes.
// Serializes/deserializes XCAFDoc_DimTol data (tolerances and limits).

/// Local model of dimension/tolerance type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DimTolType {
    LinearTolerance,
    AngularTolerance,
    Dimension,
    Limit,
}

impl DimTolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DimTolType::LinearTolerance => "LinearTolerance",
            DimTolType::AngularTolerance => "AngularTolerance",
            DimTolType::Dimension => "Dimension",
            DimTolType::Limit => "Limit",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "LinearTolerance" => Some(DimTolType::LinearTolerance),
            "AngularTolerance" => Some(DimTolType::AngularTolerance),
            "Dimension" => Some(DimTolType::Dimension),
            "Limit" => Some(DimTolType::Limit),
            _ => None,
        }
    }
}

/// Local model of dimension/tolerance data.
#[derive(Debug, Clone, PartialEq)]
pub struct DimTolData {
    pub dimtol_type: DimTolType,
    pub value: f64,
    pub name: String,
}

impl DimTolData {
    pub fn new(dimtol_type: DimTolType, value: f64, name: &str) -> Self {
        Self {
            dimtol_type,
            value,
            name: name.to_string(),
        }
    }
}

/// XmlMDF_ADriver for dimension/tolerance attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocDimTolDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocDimTolDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_DimTol";

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

    /// Read dimension/tolerance from XML element text.
    /// Format: "dimtol_type value name" (e.g., "LinearTolerance 0.005 tol_hole").
    pub fn read_from_xml(&self, element_text: &str) -> Result<DimTolData, String> {
        let mut parts = element_text.split_whitespace();
        let type_str = parts
            .next()
            .ok_or_else(|| "Missing DimTol type".to_string())?;
        let value_str = parts
            .next()
            .ok_or_else(|| "Missing DimTol value".to_string())?;
        let name = parts
            .next()
            .ok_or_else(|| "Missing DimTol name".to_string())?;

        let dimtol_type = DimTolType::from_str(type_str)
            .ok_or_else(|| format!("Unknown DimTol type: {}", type_str))?;
        let value = value_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse value: {}", e))?;

        Ok(DimTolData::new(dimtol_type, value, name))
    }

    /// Write dimension/tolerance to XML element text.
    pub fn write_to_xml(&self, data: &DimTolData) -> String {
        format!("{} {} {}", data.dimtol_type.as_str(), data.value, data.name)
    }
}

impl Default for XmlMXCAFDocDimTolDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimtol_type_as_str() {
        assert_eq!(DimTolType::LinearTolerance.as_str(), "LinearTolerance");
        assert_eq!(DimTolType::AngularTolerance.as_str(), "AngularTolerance");
        assert_eq!(DimTolType::Dimension.as_str(), "Dimension");
        assert_eq!(DimTolType::Limit.as_str(), "Limit");
    }

    #[test]
    fn test_dimtol_type_from_str() {
        assert_eq!(DimTolType::from_str("LinearTolerance"), Some(DimTolType::LinearTolerance));
        assert_eq!(DimTolType::from_str("AngularTolerance"), Some(DimTolType::AngularTolerance));
        assert_eq!(DimTolType::from_str("Dimension"), Some(DimTolType::Dimension));
        assert_eq!(DimTolType::from_str("Invalid"), None);
    }

    #[test]
    fn test_dimtol_data_new() {
        let dt = DimTolData::new(DimTolType::LinearTolerance, 0.01, "tol_x");
        assert_eq!(dt.dimtol_type, DimTolType::LinearTolerance);
        assert!((dt.value - 0.01).abs() < 1e-10);
        assert_eq!(dt.name, "tol_x");
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_DimTol");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_valid() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        let result = driver.read_from_xml("LinearTolerance 0.005 tol_hole");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.dimtol_type, DimTolType::LinearTolerance);
        assert!((data.value - 0.005).abs() < 1e-10);
        assert_eq!(data.name, "tol_hole");
    }

    #[test]
    fn test_read_from_xml_angular() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        let result = driver.read_from_xml("AngularTolerance 2.5 ang_tol");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.dimtol_type, DimTolType::AngularTolerance);
    }

    #[test]
    fn test_read_from_xml_invalid_value() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        let result = driver.read_from_xml("LinearTolerance not_a_number name");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_missing_field() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        let result = driver.read_from_xml("LinearTolerance 0.01");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        let data = DimTolData::new(DimTolType::Dimension, 25.5, "dim_shaft");
        let xml = driver.write_to_xml(&data);
        assert_eq!(xml, "Dimension 25.5 dim_shaft");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocDimTolDriver::new();
        let original = DimTolData::new(DimTolType::Limit, 100.0, "max_length");
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.dimtol_type, restored.dimtol_type);
        assert!((original.value - restored.value).abs() < 1e-10);
        assert_eq!(original.name, restored.name);
    }
}
