// FILE: xml_mxcaf_doc_note_driver.rs
// occt: XmlMXCAFDoc_NoteDriver
//
// Faithful port of OCCT XmlMXCAFDoc_NoteDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_NoteDriver.hxx),
// the XmlMDF_ADriver for XCAF base note attributes.
// Serializes/deserializes XCAFDoc_Note base data (position and title).

/// Local model of base note data (position + title).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteData {
    /// Position coordinates (x, y, z) in 3D space
    pub position: (f64, f64, f64),
    /// Title/label of the note
    pub title: String,
}

impl NoteData {
    pub fn new(x: f64, y: f64, z: f64, title: &str) -> Self {
        Self {
            position: (x, y, z),
            title: title.to_string(),
        }
    }

    pub fn origin(title: &str) -> Self {
        Self::new(0.0, 0.0, 0.0, title)
    }
}

/// XmlMDF_ADriver for base note attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocNoteDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocNoteDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Note";

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

    /// Read base note from XML element text.
    /// Format: "x y z title" (space-separated, first 3 are floats for position, rest is title).
    pub fn read_from_xml(&self, element_text: &str) -> Result<NoteData, String> {
        let mut parts = element_text.split_whitespace();
        let x_str = parts
            .next()
            .ok_or_else(|| "Missing x coordinate".to_string())?;
        let y_str = parts
            .next()
            .ok_or_else(|| "Missing y coordinate".to_string())?;
        let z_str = parts
            .next()
            .ok_or_else(|| "Missing z coordinate".to_string())?;

        let x = x_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse x: {}", e))?;
        let y = y_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse y: {}", e))?;
        let z = z_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse z: {}", e))?;

        let title = parts.collect::<Vec<_>>().join(" ");

        Ok(NoteData {
            position: (x, y, z),
            title,
        })
    }

    /// Write base note to XML element text.
    pub fn write_to_xml(&self, data: &NoteData) -> String {
        format!(
            "{} {} {} {}",
            data.position.0, data.position.1, data.position.2, data.title
        )
    }
}

impl Default for XmlMXCAFDocNoteDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_data_new() {
        let note = NoteData::new(10.0, 20.0, 30.0, "Note1");
        assert_eq!(note.position, (10.0, 20.0, 30.0));
        assert_eq!(note.title, "Note1");
    }

    #[test]
    fn test_note_data_origin() {
        let note = NoteData::origin("OriginNote");
        assert_eq!(note.position, (0.0, 0.0, 0.0));
        assert_eq!(note.title, "OriginNote");
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocNoteDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Note");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_simple() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let result = driver.read_from_xml("5.0 10.0 15.0 Note_A");
        assert!(result.is_ok());
        let note = result.unwrap();
        assert_eq!(note.position, (5.0, 10.0, 15.0));
        assert_eq!(note.title, "Note_A");
    }

    #[test]
    fn test_read_from_xml_multiword_title() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let result = driver.read_from_xml("1.0 2.0 3.0 Important Design Issue");
        assert!(result.is_ok());
        let note = result.unwrap();
        assert_eq!(note.title, "Important Design Issue");
    }

    #[test]
    fn test_read_from_xml_no_title() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let result = driver.read_from_xml("0.0 0.0 0.0");
        assert!(result.is_ok());
        let note = result.unwrap();
        assert!(note.title.is_empty());
    }

    #[test]
    fn test_read_from_xml_missing_z() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let result = driver.read_from_xml("1.0 2.0 Title");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_invalid_coordinate() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let result = driver.read_from_xml("1.0 not_a_number 3.0 title");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml_simple() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let note = NoteData::new(5.0, 10.0, 15.0, "Label");
        let xml = driver.write_to_xml(&note);
        assert_eq!(xml, "5 10 15 Label");
    }

    #[test]
    fn test_write_to_xml_multiword() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let note = NoteData::new(0.0, 0.0, 0.0, "Important Review Point");
        let xml = driver.write_to_xml(&note);
        assert_eq!(xml, "0 0 0 Important Review Point");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let original = NoteData::new(100.5, 200.3, 300.7, "Design Note");
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert!((restored.position.0 - original.position.0).abs() < 1e-10);
        assert!((restored.position.1 - original.position.1).abs() < 1e-10);
        assert!((restored.position.2 - original.position.2).abs() < 1e-10);
        assert_eq!(original.title, restored.title);
    }
}
