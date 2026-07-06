// FILE: xml_mxcaf_doc_note_comment_driver.rs
// occt: XmlMXCAFDoc_NoteCommentDriver
//
// Faithful port of OCCT XmlMXCAFDoc_NoteCommentDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_NoteCommentDriver.hxx),
// the XmlMDF_ADriver for XCAF comment note attributes.
// Serializes/deserializes XCAFDoc_NoteComment data (author, timestamp, text).

/// Local model of comment note data.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoteCommentData {
    pub author: String,
    pub timestamp: u64,
    pub text: String,
}

impl NoteCommentData {
    pub fn new(author: &str, timestamp: u64, text: &str) -> Self {
        Self {
            author: author.to_string(),
            timestamp,
            text: text.to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

/// XmlMDF_ADriver for comment note attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocNoteCommentDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocNoteCommentDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_NoteComment";

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

    /// Read comment note from XML element text.
    /// Format: "author timestamp text" (space-separated, timestamp is an integer, text is the rest).
    pub fn read_from_xml(&self, element_text: &str) -> Result<NoteCommentData, String> {
        let mut parts = element_text.split_whitespace();
        let author = parts
            .next()
            .ok_or_else(|| "Missing author".to_string())?
            .to_string();
        let timestamp_str = parts
            .next()
            .ok_or_else(|| "Missing timestamp".to_string())?;

        let timestamp = timestamp_str
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse timestamp: {}", e))?;

        let text = parts.collect::<Vec<_>>().join(" ");

        Ok(NoteCommentData {
            author,
            timestamp,
            text,
        })
    }

    /// Write comment note to XML element text.
    pub fn write_to_xml(&self, data: &NoteCommentData) -> String {
        format!("{} {} {}", data.author, data.timestamp, data.text)
    }
}

impl Default for XmlMXCAFDocNoteCommentDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_comment_data_new() {
        let note = NoteCommentData::new("john_doe", 1609459200, "Review needed");
        assert_eq!(note.author, "john_doe");
        assert_eq!(note.timestamp, 1609459200);
        assert_eq!(note.text, "Review needed");
    }

    #[test]
    fn test_note_comment_data_is_empty() {
        let note1 = NoteCommentData::new("author", 0, "text");
        assert!(!note1.is_empty());

        let note2 = NoteCommentData::new("author", 0, "");
        assert!(note2.is_empty());
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_NoteComment");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_simple() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let result = driver.read_from_xml("alice 1620000000 Fix_design_issue");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.author, "alice");
        assert_eq!(data.timestamp, 1620000000);
        assert_eq!(data.text, "Fix_design_issue");
    }

    #[test]
    fn test_read_from_xml_multiword_text() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let result = driver.read_from_xml("bob 1630000000 This needs revision and approval");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.author, "bob");
        assert_eq!(data.text, "This needs revision and approval");
    }

    #[test]
    fn test_read_from_xml_no_text() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let result = driver.read_from_xml("charlie 1640000000");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.author, "charlie");
        assert!(data.text.is_empty());
    }

    #[test]
    fn test_read_from_xml_invalid_timestamp() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let result = driver.read_from_xml("dave not_a_timestamp text");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_missing_author() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let result = driver.read_from_xml("1234567890 text");
        // "1234567890" is parsed as author, next part as timestamp
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_to_xml() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let note = NoteCommentData::new("inspector", 1700000000, "Approved");
        let xml = driver.write_to_xml(&note);
        assert_eq!(xml, "inspector 1700000000 Approved");
    }

    #[test]
    fn test_write_to_xml_multiword() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let note = NoteCommentData::new("team", 1700000000, "Ready for production");
        let xml = driver.write_to_xml(&note);
        assert_eq!(xml, "team 1700000000 Ready for production");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let original = NoteCommentData::new("reviewer", 1700000000, "All checks passed");
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original.author, restored.author);
        assert_eq!(original.timestamp, restored.timestamp);
        assert_eq!(original.text, restored.text);
    }
}
