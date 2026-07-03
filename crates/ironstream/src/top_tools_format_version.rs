// FILE: top_tools_format_version.rs
// occt: TopTools_FormatVersion

/// Binary format versions for topology persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopToolsFormatVersion {
    /// Version 1: Initial format
    Version1 = 1,
    /// Version 2: Enhanced with additional metadata
    Version2 = 2,
    /// Version 3: Current format with extended features
    Version3 = 3,
}

impl TopToolsFormatVersion {
    /// Get the current format version.
    pub fn current() -> Self {
        TopToolsFormatVersion::Version3
    }

    /// Check if a version is supported.
    pub fn is_supported(version: u32) -> bool {
        matches!(version, 1 | 2 | 3)
    }

    /// Get version as u32.
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    /// Create from u32.
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(TopToolsFormatVersion::Version1),
            2 => Some(TopToolsFormatVersion::Version2),
            3 => Some(TopToolsFormatVersion::Version3),
            _ => None,
        }
    }

    /// Get version description.
    pub fn description(&self) -> &'static str {
        match self {
            TopToolsFormatVersion::Version1 => "Initial format",
            TopToolsFormatVersion::Version2 => "Enhanced with metadata",
            TopToolsFormatVersion::Version3 => "Current extended format",
        }
    }

    /// Check if backward compatible with another version.
    pub fn is_compatible_with(&self, other: TopToolsFormatVersion) -> bool {
        // Assume only current version reads current format
        self == &other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        assert_eq!(TopToolsFormatVersion::current(), TopToolsFormatVersion::Version3);
    }

    #[test]
    fn test_is_supported() {
        assert!(TopToolsFormatVersion::is_supported(1));
        assert!(TopToolsFormatVersion::is_supported(2));
        assert!(TopToolsFormatVersion::is_supported(3));
        assert!(!TopToolsFormatVersion::is_supported(4));
    }

    #[test]
    fn test_as_u32() {
        assert_eq!(TopToolsFormatVersion::Version1.as_u32(), 1);
        assert_eq!(TopToolsFormatVersion::Version3.as_u32(), 3);
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(
            TopToolsFormatVersion::from_u32(1),
            Some(TopToolsFormatVersion::Version1)
        );
        assert_eq!(TopToolsFormatVersion::from_u32(99), None);
    }

    #[test]
    fn test_ordering() {
        assert!(TopToolsFormatVersion::Version1 < TopToolsFormatVersion::Version3);
        assert!(TopToolsFormatVersion::Version3 > TopToolsFormatVersion::Version1);
    }

    #[test]
    fn test_is_compatible() {
        let v3 = TopToolsFormatVersion::Version3;
        assert!(v3.is_compatible_with(TopToolsFormatVersion::Version3));
        assert!(!v3.is_compatible_with(TopToolsFormatVersion::Version1));
    }
}
