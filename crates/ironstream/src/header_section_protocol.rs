// FILE: header_section_protocol.rs
// occt: HeaderSection_Protocol

/// Protocol for handling STEP header section data
pub struct HeaderSection_Protocol;

impl HeaderSection_Protocol {
    /// Creates a new protocol instance
    pub fn new() -> Self {
        HeaderSection_Protocol
    }

    /// Returns the schema name for the protocol
    pub fn schema_name() -> &'static str {
        "HEADER_SECTION"
    }
}

impl Default for HeaderSection_Protocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = HeaderSection_Protocol::new();
    }

    #[test]
    fn test_schema_name() {
        assert_eq!(HeaderSection_Protocol::schema_name(), "HEADER_SECTION");
    }
}
