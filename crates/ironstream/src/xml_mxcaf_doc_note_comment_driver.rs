// FILE: xml_mxcaf_doc_note_comment_driver.rs
// occt: XmlMXCAFDoc_NoteCommentDriver
//
// Port of OCCT XmlMXCAFDoc_NoteCommentDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_NoteCommentDriver.cxx),
// the XmlMDF_ADriver for XCAFDoc_NoteComment attributes.
// Extends the base note driver (user_name/time_stamp attributes) with
// a "comment" XML attribute.
//
// Note: the OCCT write Paste contains an obvious typo — it serializes
// `aNote->TimeStamp()` into the "comment" attribute. This port writes
// the comment itself, which is the documented intent (the read side
// restores the "comment" attribute into the note's comment).

use std::collections::HashMap;

/// DOM attribute names (IMPLEMENT_DOMSTRING in OCCT).
pub const ATTR_USER_NAME: &str = "user_name";
pub const ATTR_TIME_STAMP: &str = "time_stamp";
pub const ATTR_COMMENT: &str = "comment";

/// Local model of an XmlObjMgt_Element: XML attributes by name.
#[derive(Debug, Clone, Default)]
pub struct XmlElement {
    attributes: HashMap<String, String>,
}

impl XmlElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }
}

/// Local model of the transient XCAFDoc_NoteComment attribute:
/// base note data (user name, timestamp) plus a comment.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XCAFDocNoteComment {
    user_name: String,
    time_stamp: String,
    comment: String,
}

impl XCAFDocNoteComment {
    pub fn new() -> Self {
        Self::default()
    }

    /// XCAFDoc_Note::Set (base part).
    pub fn set_note(&mut self, user_name: &str, time_stamp: &str) {
        self.user_name = user_name.to_string();
        self.time_stamp = time_stamp.to_string();
    }

    /// XCAFDoc_NoteComment::Set.
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

/// XmlMDF_ADriver for XCAFDoc_NoteComment attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocNoteCommentDriver {
    type_name: String,
}

impl XmlMXCAFDocNoteCommentDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_NoteComment";

    pub fn new() -> Self {
        Self {
            type_name: Self::TYPE_NAME.to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// OCCT NewEmpty.
    pub fn new_empty(&self) -> XCAFDocNoteComment {
        XCAFDocNoteComment::new()
    }

    /// Base XmlMXCAFDoc_NoteDriver::Paste (persistent -> transient):
    /// reads "user_name" and "time_stamp".
    fn paste_note_from_xml(&self, source: &XmlElement, target: &mut XCAFDocNoteComment) -> bool {
        let user_name = match source.get_attribute(ATTR_USER_NAME) {
            Some(v) => v.to_string(),
            None => return false,
        };
        let time_stamp = match source.get_attribute(ATTR_TIME_STAMP) {
            Some(v) => v.to_string(),
            None => return false,
        };
        target.set_note(&user_name, &time_stamp);
        true
    }

    /// OCCT Paste (persistent -> transient): base note data first, then
    /// the "comment" attribute; false when "comment" is missing.
    pub fn paste_from_xml(&self, source: &XmlElement, target: &mut XCAFDocNoteComment) -> bool {
        self.paste_note_from_xml(source, target);
        let comment = match source.get_attribute(ATTR_COMMENT) {
            Some(v) => v,
            None => return false,
        };
        target.set_comment(comment);
        true
    }

    /// OCCT Paste (transient -> persistent): writes base note attributes
    /// and the "comment" attribute.
    pub fn paste_to_xml(&self, source: &XCAFDocNoteComment, target: &mut XmlElement) {
        target.set_attribute(ATTR_USER_NAME, source.user_name());
        target.set_attribute(ATTR_TIME_STAMP, source.time_stamp());
        target.set_attribute(ATTR_COMMENT, source.comment());
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
    fn test_driver_new() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_NoteComment");
        assert_eq!(driver.new_empty(), XCAFDocNoteComment::new());
    }

    #[test]
    fn test_paste_from_xml_valid() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute(ATTR_USER_NAME, "alice");
        el.set_attribute(ATTR_TIME_STAMP, "2026-07-02");
        el.set_attribute(ATTR_COMMENT, "Fix design issue");

        let mut note = XCAFDocNoteComment::new();
        assert!(driver.paste_from_xml(&el, &mut note));
        assert_eq!(note.user_name(), "alice");
        assert_eq!(note.time_stamp(), "2026-07-02");
        assert_eq!(note.comment(), "Fix design issue");
    }

    #[test]
    fn test_paste_from_xml_missing_comment_fails() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute(ATTR_USER_NAME, "bob");
        el.set_attribute(ATTR_TIME_STAMP, "ts");

        let mut note = XCAFDocNoteComment::new();
        assert!(!driver.paste_from_xml(&el, &mut note));
        // Base note data was still restored (OCCT pastes base data first).
        assert_eq!(note.user_name(), "bob");
        assert_eq!(note.comment(), "");
    }

    #[test]
    fn test_paste_from_xml_missing_base_attrs_still_reads_comment() {
        // OCCT ignores the base driver's return value; the comment
        // attribute alone decides success.
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute(ATTR_COMMENT, "only comment");

        let mut note = XCAFDocNoteComment::new();
        assert!(driver.paste_from_xml(&el, &mut note));
        assert_eq!(note.comment(), "only comment");
        assert_eq!(note.user_name(), "");
    }

    #[test]
    fn test_paste_to_xml() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let mut note = XCAFDocNoteComment::new();
        note.set_note("inspector", "1700000000");
        note.set_comment("Approved");

        let mut el = XmlElement::new();
        driver.paste_to_xml(&note, &mut el);
        assert_eq!(el.get_attribute(ATTR_USER_NAME), Some("inspector"));
        assert_eq!(el.get_attribute(ATTR_TIME_STAMP), Some("1700000000"));
        assert_eq!(el.get_attribute(ATTR_COMMENT), Some("Approved"));
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocNoteCommentDriver::new();
        let mut original = XCAFDocNoteComment::new();
        original.set_note("reviewer", "2026-01-01T00:00:00Z");
        original.set_comment("All checks passed");

        let mut el = XmlElement::new();
        driver.paste_to_xml(&original, &mut el);

        let mut restored = XCAFDocNoteComment::new();
        assert!(driver.paste_from_xml(&el, &mut restored));
        assert_eq!(restored, original);
    }
}
