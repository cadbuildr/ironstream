// FILE: xml_mxcaf_doc_color_driver.rs
// occt: XmlMXCAFDoc_ColorDriver
//
// Faithful port of OCCT XmlMXCAFDoc_ColorDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_ColorDriver.hxx),
// the XmlMDF_ADriver for XCAF color attributes.
// Serializes/deserializes XCAFDoc_Color data (a Quantity_Color RGBA value).

/// Local model of color data (RGBA).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ColorData {
    /// Red channel [0-255]
    pub r: u8,
    /// Green channel [0-255]
    pub g: u8,
    /// Blue channel [0-255]
    pub b: u8,
    /// Alpha channel [0-255], typically 255 (opaque)
    pub a: u8,
}

impl ColorData {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

/// XmlMDF_ADriver for color attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocColorDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocColorDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Color";

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

    /// Read color from XML element text.
    /// Format: "r g b a" (unsigned integers in [0,255]).
    pub fn read_from_xml(&self, element_text: &str) -> Result<ColorData, String> {
        let parts: Result<Vec<u8>, _> = element_text
            .split_whitespace()
            .map(|s| s.parse::<u8>())
            .collect();

        match parts {
            Ok(channels) if channels.len() == 4 => Ok(ColorData::new(
                channels[0], channels[1], channels[2], channels[3],
            )),
            Ok(channels) if channels.len() == 3 => Ok(ColorData::rgb(channels[0], channels[1], channels[2])),
            Ok(_) => Err(format!(
                "Color requires 3 or 4 channels, got {}",
                element_text.split_whitespace().count()
            )),
            Err(e) => Err(format!("Failed to parse color channel: {}", e)),
        }
    }

    /// Write color to XML element text.
    pub fn write_to_xml(&self, data: &ColorData) -> String {
        format!("{} {} {} {}", data.r, data.g, data.b, data.a)
    }
}

impl Default for XmlMXCAFDocColorDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_data_new() {
        let c = ColorData::new(255, 128, 64, 200);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 64);
        assert_eq!(c.a, 200);
    }

    #[test]
    fn test_color_data_rgb() {
        let c = ColorData::rgb(100, 150, 200);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_color_data_black() {
        let c = ColorData::black();
        assert_eq!(c.r, 0);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_color_data_white() {
        let c = ColorData::white();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 255);
        assert_eq!(c.b, 255);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocColorDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Color");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_rgba() {
        let driver = XmlMXCAFDocColorDriver::new();
        let result = driver.read_from_xml("255 128 64 200");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.r, 255);
        assert_eq!(data.g, 128);
        assert_eq!(data.b, 64);
        assert_eq!(data.a, 200);
    }

    #[test]
    fn test_read_from_xml_rgb() {
        let driver = XmlMXCAFDocColorDriver::new();
        let result = driver.read_from_xml("100 150 200");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.a, 255);
    }

    #[test]
    fn test_read_from_xml_invalid_count() {
        let driver = XmlMXCAFDocColorDriver::new();
        assert!(driver.read_from_xml("255 128").is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocColorDriver::new();
        let data = ColorData::new(50, 100, 150, 255);
        let xml = driver.write_to_xml(&data);
        assert_eq!(xml, "50 100 150 255");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocColorDriver::new();
        let original = ColorData::new(200, 100, 50, 180);
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original, restored);
    }
}
