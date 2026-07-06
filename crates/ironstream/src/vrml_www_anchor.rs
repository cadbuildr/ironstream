// FILE: vrml_www_anchor.rs
// occt: Vrml_WWWAnchor
//
// Faithful port of OCCT Vrml_WWWAnchor (DataExchange/TKDEVRML/Vrml/
// Vrml_WWWAnchor.hxx/.cxx): the VRML 1.0 `WWWAnchor` node.
// Represents a clickable hyperlink region with description.

/// Port of Vrml_WWWAnchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlWwwAnchor {
    name: String,
    description: String,
}

impl VrmlWwwAnchor {
    /// Vrml_WWWAnchor with defaults (empty name and description).
    pub fn new() -> Self {
        VrmlWwwAnchor {
            name: String::new(),
            description: String::new(),
        }
    }

    /// Vrml_WWWAnchor(aName).
    pub fn with_name(a_name: &str) -> Self {
        VrmlWwwAnchor {
            name: a_name.to_string(),
            description: String::new(),
        }
    }

    /// Vrml_WWWAnchor(aName, aDescription).
    pub fn with_name_and_description(a_name: &str, a_description: &str) -> Self {
        VrmlWwwAnchor {
            name: a_name.to_string(),
            description: a_description.to_string(),
        }
    }

    pub fn set_name(&mut self, a_name: &str) {
        self.name = a_name.to_string();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_description(&mut self, a_description: &str) {
        self.description = a_description.to_string();
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Default for VrmlWwwAnchor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_anchor() {
        let anchor = VrmlWwwAnchor::new();
        assert_eq!(anchor.name(), "");
        assert_eq!(anchor.description(), "");
    }

    #[test]
    fn with_name() {
        let anchor = VrmlWwwAnchor::with_name("http://example.com");
        assert_eq!(anchor.name(), "http://example.com");
        assert_eq!(anchor.description(), "");
    }

    #[test]
    fn with_name_and_description() {
        let anchor = VrmlWwwAnchor::with_name_and_description(
            "http://example.com",
            "Click to visit example.com",
        );
        assert_eq!(anchor.name(), "http://example.com");
        assert_eq!(anchor.description(), "Click to visit example.com");
    }

    #[test]
    fn set_name() {
        let mut anchor = VrmlWwwAnchor::new();
        anchor.set_name("http://test.com");
        assert_eq!(anchor.name(), "http://test.com");
    }

    #[test]
    fn set_description() {
        let mut anchor = VrmlWwwAnchor::new();
        anchor.set_description("Test description");
        assert_eq!(anchor.description(), "Test description");
    }

    #[test]
    fn setters_together() {
        let mut anchor = VrmlWwwAnchor::new();
        anchor.set_name("http://cadbuildr.com");
        anchor.set_description("CAD builder tool");
        assert_eq!(anchor.name(), "http://cadbuildr.com");
        assert_eq!(anchor.description(), "CAD builder tool");
    }

    #[test]
    fn equality() {
        let a1 = VrmlWwwAnchor::with_name_and_description("url1", "desc1");
        let a2 = VrmlWwwAnchor::with_name_and_description("url1", "desc1");
        let a3 = VrmlWwwAnchor::with_name_and_description("url2", "desc2");
        assert_eq!(a1, a2);
        assert_ne!(a1, a3);
    }

    #[test]
    fn empty_strings() {
        let mut anchor = VrmlWwwAnchor::with_name("http://test.com");
        anchor.set_name("");
        anchor.set_description("");
        assert_eq!(anchor.name(), "");
        assert_eq!(anchor.description(), "");
    }
}
