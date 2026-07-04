// FILE: std_storage_header_data.rs
// occt: StdStorage_HeaderData

/// Storage header information
pub struct HeaderData {
    format_version: i32,
    description: String,
    creation_date: String,
}

impl HeaderData {
    /// Create a new header
    pub fn new() -> Self {
        HeaderData {
            format_version: 1,
            description: String::new(),
            creation_date: String::new(),
        }
    }

    /// Get format version
    pub fn format_version(&self) -> i32 {
        self.format_version
    }

    /// Set format version
    pub fn set_format_version(&mut self, version: i32) {
        self.format_version = version;
    }

    /// Get description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set description
    pub fn set_description(&mut self, desc: &str) {
        self.description = desc.to_string();
    }

    /// Get creation date
    pub fn creation_date(&self) -> &str {
        &self.creation_date
    }

    /// Set creation date
    pub fn set_creation_date(&mut self, date: &str) {
        self.creation_date = date.to_string();
    }
}

impl Default for HeaderData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let header = HeaderData::new();
        assert_eq!(header.format_version(), 1);
        assert_eq!(header.description(), "");
    }

    #[test]
    fn test_set_description() {
        let mut header = HeaderData::new();
        header.set_description("My Document");
        assert_eq!(header.description(), "My Document");
    }

    #[test]
    fn test_set_date() {
        let mut header = HeaderData::new();
        header.set_creation_date("2024-01-01");
        assert_eq!(header.creation_date(), "2024-01-01");
    }
}
