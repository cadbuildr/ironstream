// FILE: ldom_cdata_section.rs
// occt: LDOM_CDATASection

/// Represents a CDATA section in the LDOM DOM tree.
/// CDATA sections are text nodes that contain character data
/// that should not be parsed for markup.
#[derive(Clone, Default)]
pub struct LDOMCDATASection {
    data: String,
}

impl LDOMCDATASection {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMCDATASection {
            data: String::new(),
        }
    }

    /// Constructor from another CDATA section (copy)
    pub fn from_other(other: &LDOMCDATASection) -> Self {
        LDOMCDATASection {
            data: other.data.clone(),
        }
    }

    /// Nullify the CDATA section
    pub fn set_null(&mut self) {
        self.data.clear();
    }

    /// Get the CDATA content
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Set the CDATA content
    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_cdata() {
        let cdata = LDOMCDATASection::new();
        assert_eq!(cdata.get_data(), "");
    }

    #[test]
    fn test_copy_constructor() {
        let cdata1 = LDOMCDATASection {
            data: "test data".to_string(),
        };
        let cdata2 = LDOMCDATASection::from_other(&cdata1);
        assert_eq!(cdata2.get_data(), "test data");
    }

    #[test]
    fn test_set_data() {
        let mut cdata = LDOMCDATASection::new();
        cdata.set_data("some markup <tag>");
        assert_eq!(cdata.get_data(), "some markup <tag>");
    }

    #[test]
    fn test_nullify() {
        let mut cdata = LDOMCDATASection::new();
        cdata.set_data("data");
        cdata.set_null();
        assert_eq!(cdata.get_data(), "");
    }
}
