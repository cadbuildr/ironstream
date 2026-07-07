// FILE: xml_mxcaf_doc_note_driver.rs
// occt: XmlMXCAFDoc_NoteDriver
//
// Port of OCCT XmlMXCAFDoc_NoteDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_NoteDriver.cxx),
// the XmlMDF_ADriver for XCAFDoc_Note base attributes.
// Serializes/deserializes the note's user name and timestamp as the
// XML attributes "user_name" and "time_stamp".

use std::collections::HashMap;

/// DOM attribute names (IMPLEMENT_DOMSTRING in OCCT).
pub const ATTR_USER_NAME: &str = "user_name";
pub const ATTR_TIME_STAMP: &str = "time_stamp";

/// Local model of an XmlObjMgt_Element: XML attributes by name.
#[derive(Debug, Clone, Default)]
pub struct XmlElement {
    attributes: HashMap<String, String>,
}

impl XmlElement {
    pub fn new() -> Self {
        Self::default()
    }

    /// getAttribute analogue: None when the attribute is absent.
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// setAttribute analogue.
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }
}

/// Local model of the transient XCAFDoc_Note base attribute.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XCAFDocNote {
    user_name: String,
    time_stamp: String,
}

impl XCAFDocNote {
    pub fn new() -> Self {
        Self::default()
    }

    /// XCAFDoc_Note::Set.
    pub fn set(&mut self, user_name: &str, time_stamp: &str) {
        self.user_name = user_name.to_string();
        self.time_stamp = time_stamp.to_string();
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }
}

/// XmlMDF_ADriver for XCAFDoc_Note attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocNoteDriver {
    type_name: String,
}

impl XmlMXCAFDocNoteDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_Note";

    pub fn new() -> Self {
        Self {
            type_name: Self::TYPE_NAME.to_string(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// OCCT Paste (persistent -> transient): reads "user_name" and
    /// "time_stamp"; returns false when either attribute is missing.
    pub fn paste_from_xml(&self, source: &XmlElement, target: &mut XCAFDocNote) -> bool {
        let user_name = match source.get_attribute(ATTR_USER_NAME) {
            Some(v) => v,
            None => return false,
        };
        let time_stamp = match source.get_attribute(ATTR_TIME_STAMP) {
            Some(v) => v,
            None => return false,
        };
        target.set(user_name, time_stamp);
        true
    }

    /// OCCT Paste (transient -> persistent): writes "user_name" and
    /// "time_stamp" attributes on the target element.
    pub fn paste_to_xml(&self, source: &XCAFDocNote, target: &mut XmlElement) {
        target.set_attribute(ATTR_USER_NAME, source.user_name());
        target.set_attribute(ATTR_TIME_STAMP, source.time_stamp());
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
    fn test_driver_new() {
        let driver = XmlMXCAFDocNoteDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_Note");
    }

    #[test]
    fn test_paste_from_xml_valid() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute(ATTR_USER_NAME, "alice");
        el.set_attribute(ATTR_TIME_STAMP, "2026-07-02T10:00:00");

        let mut note = XCAFDocNote::new();
        assert!(driver.paste_from_xml(&el, &mut note));
        assert_eq!(note.user_name(), "alice");
        assert_eq!(note.time_stamp(), "2026-07-02T10:00:00");
    }

    #[test]
    fn test_paste_from_xml_missing_user_name() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute(ATTR_TIME_STAMP, "ts");

        let mut note = XCAFDocNote::new();
        assert!(!driver.paste_from_xml(&el, &mut note));
        // Target must be untouched on failure.
        assert_eq!(note, XCAFDocNote::new());
    }

    #[test]
    fn test_paste_from_xml_missing_time_stamp() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute(ATTR_USER_NAME, "bob");

        let mut note = XCAFDocNote::new();
        assert!(!driver.paste_from_xml(&el, &mut note));
        assert_eq!(note, XCAFDocNote::new());
    }

    #[test]
    fn test_paste_to_xml() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let mut note = XCAFDocNote::new();
        note.set("carol", "2021-05-05");

        let mut el = XmlElement::new();
        driver.paste_to_xml(&note, &mut el);
        assert_eq!(el.get_attribute(ATTR_USER_NAME), Some("carol"));
        assert_eq!(el.get_attribute(ATTR_TIME_STAMP), Some("2021-05-05"));
    }

    #[test]
    fn test_paste_to_xml_empty_values() {
        // Empty strings are still written as (empty) attributes,
        // so a read back succeeds — unlike missing attributes.
        let driver = XmlMXCAFDocNoteDriver::new();
        let note = XCAFDocNote::new();
        let mut el = XmlElement::new();
        driver.paste_to_xml(&note, &mut el);
        assert_eq!(el.get_attribute(ATTR_USER_NAME), Some(""));

        let mut back = XCAFDocNote::new();
        assert!(driver.paste_from_xml(&el, &mut back));
        assert_eq!(back, note);
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocNoteDriver::new();
        let mut original = XCAFDocNote::new();
        original.set("dave smith", "2026-01-01T00:00:00Z");

        let mut el = XmlElement::new();
        driver.paste_to_xml(&original, &mut el);

        let mut restored = XCAFDocNote::new();
        assert!(driver.paste_from_xml(&el, &mut restored));
        assert_eq!(restored, original);
    }
}
