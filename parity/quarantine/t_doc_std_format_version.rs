// FILE: t_doc_std_format_version.rs
// occt: TDocStd_FormatVersion

/// Represents a file format version for documents.
/// Used to track which version of the format was used to save a document.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TDocStd_FormatVersion {
    major: u32,
    minor: u32,
}

impl TDocStd_FormatVersion {
    /// Create a new format version.
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Get the major version.
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Get the minor version.
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Get the version as a string.
    pub fn to_version_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }

    /// Parse a version string.
    pub fn from_version_string(version_str: &str) -> Option<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() >= 2 {
            if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                return Some(Self::new(major, minor));
            }
        }
        None
    }

    /// Get the current/latest format version.
    pub fn current() -> Self {
        Self::new(1, 0)
    }
}

impl Default for TDocStd_FormatVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl std::fmt::Display for TDocStd_FormatVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_version() {
        let ver = TDocStd_FormatVersion::new(1, 0);
        assert_eq!(ver.major(), 1);
        assert_eq!(ver.minor(), 0);
    }

    #[test]
    fn test_version_string() {
        let ver = TDocStd_FormatVersion::new(2, 5);
        assert_eq!(ver.to_version_string(), "2.5");
    }

    #[test]
    fn test_parse_version_string() {
        let ver = TDocStd_FormatVersion::from_version_string("1.0");
        assert_eq!(ver, Some(TDocStd_FormatVersion::new(1, 0)));
    }

    #[test]
    fn test_current_version() {
        let ver = TDocStd_FormatVersion::current();
        assert_eq!(ver.major(), 1);
    }

    #[test]
    fn test_comparison() {
        let ver1 = TDocStd_FormatVersion::new(1, 0);
        let ver2 = TDocStd_FormatVersion::new(1, 1);
        let ver3 = TDocStd_FormatVersion::new(2, 0);
        assert!(ver1 < ver2);
        assert!(ver2 < ver3);
    }

    #[test]
    fn test_display() {
        let ver = TDocStd_FormatVersion::new(1, 5);
        assert_eq!(ver.to_string(), "1.5");
    }

    #[test]
    fn test_default() {
        let ver = TDocStd_FormatVersion::default();
        assert_eq!(ver.major(), 1);
    }
}
