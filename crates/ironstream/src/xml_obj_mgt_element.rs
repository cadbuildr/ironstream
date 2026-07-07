// FILE: xml_obj_mgt_element.rs
// occt: XmlObjMgt_Element

/// XmlObjMgt_Element represents a DOM element node in the XML tree.
/// In OCCT, it wraps LDOM_Element for XML element operations.
#[derive(Clone, Debug)]
pub struct XmlObjMgt_Element {
    tag: String,
    attributes: std::collections::HashMap<String, String>,
}

impl XmlObjMgt_Element {
    /// Create a new element with the given tag name.
    pub fn new(tag: &str) -> Self {
        XmlObjMgt_Element {
            tag: tag.to_string(),
            attributes: std::collections::HashMap::new(),
        }
    }

    /// Get the tag name.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Set an attribute.
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }

    /// Get an attribute.
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Check if attribute exists.
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let elem = XmlObjMgt_Element::new("root");
        assert_eq!(elem.tag(), "root");
    }

    #[test]
    fn test_set_get_attribute() {
        let mut elem = XmlObjMgt_Element::new("elem");
        elem.set_attribute("id", "123");
        assert_eq!(elem.get_attribute("id"), Some("123"));
    }

    #[test]
    fn test_has_attribute() {
        let mut elem = XmlObjMgt_Element::new("elem");
        elem.set_attribute("attr", "value");
        assert!(elem.has_attribute("attr"));
        assert!(!elem.has_attribute("missing"));
    }
}
