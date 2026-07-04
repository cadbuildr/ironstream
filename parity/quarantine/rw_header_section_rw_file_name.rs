// FILE: rw_header_section_rw_file_name.rs
// occt: RWHeaderSection_RWFileName

/// Reader/Writer for FileName entities
pub struct RWHeaderSection_RWFileName;

impl RWHeaderSection_RWFileName {
    /// Creates a new FileName R/W instance
    pub fn new() -> Self {
        RWHeaderSection_RWFileName
    }

    /// Returns the type name handled by this reader/writer
    pub fn type_name() -> &'static str {
        "FileName"
    }
}

impl Default for RWHeaderSection_RWFileName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = RWHeaderSection_RWFileName::new();
    }

    #[test]
    fn test_type_name() {
        assert_eq!(RWHeaderSection_RWFileName::type_name(), "FileName");
    }
}
