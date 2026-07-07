// FILE: xml_mxcaf_doc_note_bin_data_driver.rs
// occt: XmlMXCAFDoc_NoteBinDataDriver
//
// Faithful port of OCCT XmlMXCAFDoc_NoteBinDataDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_NoteBinDataDriver.hxx),
// the XmlMDF_ADriver for XCAF binary data note attributes.
// Serializes/deserializes XCAFDoc_NoteBinData (base64-encoded binary payload
// with MIME type, e.g. PNG/JPEG image data).

/// Local model of binary data note.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteBinDataData {
    /// MIME type (e.g., "image/png", "image/jpeg")
    pub mime_type: String,
    /// Base64-encoded binary payload
    pub data_base64: String,
}

impl NoteBinDataData {
    pub fn new(mime_type: &str, data_base64: &str) -> Self {
        Self {
            mime_type: mime_type.to_string(),
            data_base64: data_base64.to_string(),
        }
    }

    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }
}

/// XmlMDF_ADriver for binary data note attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocNoteBinDataDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocNoteBinDataDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_NoteBinData";

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

    /// Read binary note from XML element text.
    /// Format: "mime_type data_base64" (space-separated, MIME type first, then base64 payload).
    pub fn read_from_xml(&self, element_text: &str) -> Result<NoteBinDataData, String> {
        let mut parts = element_text.split_whitespace();
        let mime_type = parts
            .next()
            .ok_or_else(|| "Missing MIME type".to_string())?
            .to_string();
        let data_base64 = parts
            .next()
            .ok_or_else(|| "Missing binary data (base64)".to_string())?
            .to_string();

        Ok(NoteBinDataData {
            mime_type,
            data_base64,
        })
    }

    /// Write binary note to XML element text.
    pub fn write_to_xml(&self, data: &NoteBinDataData) -> String {
        format!("{} {}", data.mime_type, data.data_base64)
    }
}

impl Default for XmlMXCAFDocNoteBinDataDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_bin_data_new() {
        let note = NoteBinDataData::new("image/png", "iVBORw0KGgo...");
        assert_eq!(note.mime_type, "image/png");
        assert_eq!(note.data_base64, "iVBORw0KGgo...");
    }

    #[test]
    fn test_note_bin_data_is_image() {
        let png = NoteBinDataData::new("image/png", "data");
        assert!(png.is_image());

        let jpeg = NoteBinDataData::new("image/jpeg", "data");
        assert!(jpeg.is_image());

        let text = NoteBinDataData::new("text/plain", "data");
        assert!(!text.is_image());
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_NoteBinData");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_image() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        let result = driver.read_from_xml("image/png iVBORw0KGgoAAAANSUhEUg");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.mime_type, "image/png");
        assert_eq!(data.data_base64, "iVBORw0KGgoAAAANSUhEUg");
    }

    #[test]
    fn test_read_from_xml_jpeg() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        let result = driver.read_from_xml("image/jpeg /9j/4AAQSkZJRg");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.mime_type, "image/jpeg");
    }

    #[test]
    fn test_read_from_xml_missing_mime_type() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        let result = driver.read_from_xml("data_without_mime");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_missing_data() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        let result = driver.read_from_xml("image/png");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        let note = NoteBinDataData::new("application/octet-stream", "AQIDBA==");
        let xml = driver.write_to_xml(&note);
        assert_eq!(xml, "application/octet-stream AQIDBA==");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocNoteBinDataDriver::new();
        let original = NoteBinDataData::new("image/svg+xml", "PHN2ZyB4bWxu");
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.mime_type, restored.mime_type);
        assert_eq!(original.data_base64, restored.data_base64);
    }
}
