// FILE: xml_mxcaf_doc_length_unit_driver.rs
// occt: XmlMXCAFDoc_LengthUnitDriver
//
// Faithful port of OCCT XmlMXCAFDoc_LengthUnitDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_LengthUnitDriver.hxx),
// the XmlMDF_ADriver for XCAF length unit attributes.
// Serializes/deserializes XCAFDoc_LengthUnit data (unit system and scale factor).

/// Local model of length unit enumeration.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
    Inch,
    Foot,
    Micron,
}

impl LengthUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            LengthUnit::Millimeter => "Millimeter",
            LengthUnit::Centimeter => "Centimeter",
            LengthUnit::Meter => "Meter",
            LengthUnit::Inch => "Inch",
            LengthUnit::Foot => "Foot",
            LengthUnit::Micron => "Micron",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Millimeter" => Some(LengthUnit::Millimeter),
            "Centimeter" => Some(LengthUnit::Centimeter),
            "Meter" => Some(LengthUnit::Meter),
            "Inch" => Some(LengthUnit::Inch),
            "Foot" => Some(LengthUnit::Foot),
            "Micron" => Some(LengthUnit::Micron),
            _ => None,
        }
    }

    /// Get conversion factor from this unit to millimeters.
    pub fn to_mm_factor(&self) -> f64 {
        match self {
            LengthUnit::Millimeter => 1.0,
            LengthUnit::Centimeter => 10.0,
            LengthUnit::Meter => 1000.0,
            LengthUnit::Inch => 25.4,
            LengthUnit::Foot => 304.8,
            LengthUnit::Micron => 0.001,
        }
    }
}

/// Local model of length unit data.
#[derive(Debug, Clone, PartialEq)]
pub struct LengthUnitData {
    pub unit: LengthUnit,
    pub scale_factor: f64,
}

impl LengthUnitData {
    pub fn new(unit: LengthUnit, scale_factor: f64) -> Self {
        Self { unit, scale_factor }
    }

    pub fn millimeter_default() -> Self {
        Self {
            unit: LengthUnit::Millimeter,
            scale_factor: 1.0,
        }
    }
}

/// XmlMDF_ADriver for length unit attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocLengthUnitDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocLengthUnitDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_LengthUnit";

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

    /// Read length unit from XML element text.
    /// Format: "unit_name scale_factor" (e.g., "Millimeter 1.0").
    pub fn read_from_xml(&self, element_text: &str) -> Result<LengthUnitData, String> {
        let mut parts = element_text.split_whitespace();
        let unit_str = parts
            .next()
            .ok_or_else(|| "Missing unit name".to_string())?;
        let scale_str = parts
            .next()
            .ok_or_else(|| "Missing scale_factor".to_string())?;

        let unit = LengthUnit::from_str(unit_str)
            .ok_or_else(|| format!("Unknown length unit: {}", unit_str))?;
        let scale_factor = scale_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse scale_factor: {}", e))?;

        Ok(LengthUnitData::new(unit, scale_factor))
    }

    /// Write length unit to XML element text.
    pub fn write_to_xml(&self, data: &LengthUnitData) -> String {
        format!("{} {}", data.unit.as_str(), data.scale_factor)
    }
}

impl Default for XmlMXCAFDocLengthUnitDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_unit_as_str() {
        assert_eq!(LengthUnit::Millimeter.as_str(), "Millimeter");
        assert_eq!(LengthUnit::Inch.as_str(), "Inch");
        assert_eq!(LengthUnit::Meter.as_str(), "Meter");
    }

    #[test]
    fn test_length_unit_from_str() {
        assert_eq!(LengthUnit::from_str("Millimeter"), Some(LengthUnit::Millimeter));
        assert_eq!(LengthUnit::from_str("Inch"), Some(LengthUnit::Inch));
        assert_eq!(LengthUnit::from_str("Unknown"), None);
    }

    #[test]
    fn test_length_unit_to_mm_factor() {
        assert_eq!(LengthUnit::Millimeter.to_mm_factor(), 1.0);
        assert!((LengthUnit::Inch.to_mm_factor() - 25.4).abs() < 1e-10);
        assert_eq!(LengthUnit::Centimeter.to_mm_factor(), 10.0);
        assert_eq!(LengthUnit::Meter.to_mm_factor(), 1000.0);
    }

    #[test]
    fn test_length_unit_data_new() {
        let data = LengthUnitData::new(LengthUnit::Meter, 0.001);
        assert_eq!(data.unit, LengthUnit::Meter);
        assert_eq!(data.scale_factor, 0.001);
    }

    #[test]
    fn test_length_unit_data_millimeter_default() {
        let data = LengthUnitData::millimeter_default();
        assert_eq!(data.unit, LengthUnit::Millimeter);
        assert_eq!(data.scale_factor, 1.0);
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_LengthUnit");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_valid() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        let result = driver.read_from_xml("Millimeter 1.0");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.unit, LengthUnit::Millimeter);
        assert_eq!(data.scale_factor, 1.0);
    }

    #[test]
    fn test_read_from_xml_inch() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        let result = driver.read_from_xml("Inch 25.4");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.unit, LengthUnit::Inch);
    }

    #[test]
    fn test_read_from_xml_invalid_unit() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        let result = driver.read_from_xml("UnknownUnit 1.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_invalid_scale() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        let result = driver.read_from_xml("Meter not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        let data = LengthUnitData::new(LengthUnit::Meter, 0.001);
        let xml = driver.write_to_xml(&data);
        assert_eq!(xml, "Meter 0.001");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocLengthUnitDriver::new();
        let original = LengthUnitData::new(LengthUnit::Foot, 304.8);
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.unit, restored.unit);
        assert!((original.scale_factor - restored.scale_factor).abs() < 1e-10);
    }
}
