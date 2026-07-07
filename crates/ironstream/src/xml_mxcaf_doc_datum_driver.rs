// FILE: xml_mxcaf_doc_datum_driver.rs
// occt: XmlMXCAFDoc_DatumDriver
//
// Faithful port of OCCT XmlMXCAFDoc_DatumDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_DatumDriver.hxx),
// the XmlMDF_ADriver for XCAF datum attributes.
// Serializes/deserializes XCAFDoc_Datum data (geometric datum for GD&T:
// e.g. axis, plane, coordinate system).

/// Local model of datum type enumeration (plane, axis, point, etc.).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DatumType {
    Plane,
    Axis,
    Point,
    CoordinateSystem,
}

impl DatumType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatumType::Plane => "Plane",
            DatumType::Axis => "Axis",
            DatumType::Point => "Point",
            DatumType::CoordinateSystem => "CoordinateSystem",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Plane" => Some(DatumType::Plane),
            "Axis" => Some(DatumType::Axis),
            "Point" => Some(DatumType::Point),
            "CoordinateSystem" => Some(DatumType::CoordinateSystem),
            _ => None,
        }
    }
}

/// Local model of datum data.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DatumData {
    pub datum_type: DatumType,
    pub name: String,
}

impl DatumData {
    pub fn new(datum_type: DatumType, name: &str) -> Self {
        Self {
            datum_type,
            name: name.to_string(),
        }
    }
}

/// XmlMDF_ADriver for datum attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocDatumDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocDatumDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Datum";

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

    /// Read datum from XML element text.
    /// Format: "datum_type name" (e.g., "Plane axis_A").
    pub fn read_from_xml(&self, element_text: &str) -> Result<DatumData, String> {
        let mut parts = element_text.split_whitespace();
        let type_str = parts
            .next()
            .ok_or_else(|| "Missing datum type".to_string())?;
        let name = parts
            .next()
            .ok_or_else(|| "Missing datum name".to_string())?;

        let datum_type = DatumType::from_str(type_str)
            .ok_or_else(|| format!("Unknown datum type: {}", type_str))?;

        Ok(DatumData::new(datum_type, name))
    }

    /// Write datum to XML element text.
    pub fn write_to_xml(&self, data: &DatumData) -> String {
        format!("{} {}", data.datum_type.as_str(), data.name)
    }
}

impl Default for XmlMXCAFDocDatumDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datum_type_as_str() {
        assert_eq!(DatumType::Plane.as_str(), "Plane");
        assert_eq!(DatumType::Axis.as_str(), "Axis");
        assert_eq!(DatumType::Point.as_str(), "Point");
        assert_eq!(DatumType::CoordinateSystem.as_str(), "CoordinateSystem");
    }

    #[test]
    fn test_datum_type_from_str() {
        assert_eq!(DatumType::from_str("Plane"), Some(DatumType::Plane));
        assert_eq!(DatumType::from_str("Axis"), Some(DatumType::Axis));
        assert_eq!(DatumType::from_str("Point"), Some(DatumType::Point));
        assert_eq!(DatumType::from_str("CoordinateSystem"), Some(DatumType::CoordinateSystem));
        assert_eq!(DatumType::from_str("Invalid"), None);
    }

    #[test]
    fn test_datum_data_new() {
        let d = DatumData::new(DatumType::Plane, "primary_plane");
        assert_eq!(d.datum_type, DatumType::Plane);
        assert_eq!(d.name, "primary_plane");
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocDatumDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Datum");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_valid() {
        let driver = XmlMXCAFDocDatumDriver::new();
        let result = driver.read_from_xml("Plane datum_A");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.datum_type, DatumType::Plane);
        assert_eq!(data.name, "datum_A");
    }

    #[test]
    fn test_read_from_xml_axis() {
        let driver = XmlMXCAFDocDatumDriver::new();
        let result = driver.read_from_xml("Axis axis_ref");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.datum_type, DatumType::Axis);
    }

    #[test]
    fn test_read_from_xml_invalid_type() {
        let driver = XmlMXCAFDocDatumDriver::new();
        let result = driver.read_from_xml("InvalidType name");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_missing_name() {
        let driver = XmlMXCAFDocDatumDriver::new();
        let result = driver.read_from_xml("Plane");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocDatumDriver::new();
        let data = DatumData::new(DatumType::Point, "origin");
        let xml = driver.write_to_xml(&data);
        assert_eq!(xml, "Point origin");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocDatumDriver::new();
        let original = DatumData::new(DatumType::CoordinateSystem, "local_cs");
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original, restored);
    }
}
