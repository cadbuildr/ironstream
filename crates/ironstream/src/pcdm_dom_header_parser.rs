// FILE: pcdm_dom_header_parser.rs
// occt: PCDM_DOMHeaderParser

/// DOM header parser for extracting file format information
pub struct PCDMDOMHeaderParser {
    start_element_name: Option<String>,
    end_element_name: Option<String>,
    element: Option<String>,
}

impl PCDMDOMHeaderParser {
    /// Constructor
    pub fn new() -> Self {
        PCDMDOMHeaderParser {
            start_element_name: None,
            end_element_name: None,
            element: None,
        }
    }

    /// Set the start element name to detect
    pub fn set_start_element_name(&mut self, name: &str) {
        self.start_element_name = Some(name.to_string());
    }

    /// Set the end element name to detect
    pub fn set_end_element_name(&mut self, name: &str) {
        self.end_element_name = Some(name.to_string());
    }

    /// Called on start element
    pub fn start_element(&mut self) -> bool {
        true
    }

    /// Called on end element
    pub fn end_element(&mut self) -> bool {
        true
    }

    /// Get the parsed element
    pub fn get_element(&self) -> Option<&str> {
        self.element.as_deref()
    }
}

impl Default for PCDMDOMHeaderParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = PCDMDOMHeaderParser::new();
        assert_eq!(parser.start_element_name, None);
    }

    #[test]
    fn test_set_start_element() {
        let mut parser = PCDMDOMHeaderParser::new();
        parser.set_start_element_name("root");
        assert_eq!(parser.start_element_name, Some("root".to_string()));
    }

    #[test]
    fn test_set_end_element() {
        let mut parser = PCDMDOMHeaderParser::new();
        parser.set_end_element_name("root");
        assert_eq!(parser.end_element_name, Some("root".to_string()));
    }

    #[test]
    fn test_start_element() {
        let mut parser = PCDMDOMHeaderParser::new();
        assert!(parser.start_element());
    }

    #[test]
    fn test_end_element() {
        let mut parser = PCDMDOMHeaderParser::new();
        assert!(parser.end_element());
    }
}
