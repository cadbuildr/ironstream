// FILE: t_data_std_directory.rs
// occt: TDataStd_Directory

use std::fmt;

/// A Directory attribute in the CAD data framework.
/// Associates a directory in the data framework with a TagSource attribute.
/// Used to organize data hierarchy with sub-directories and objects.
#[derive(Clone, Debug)]
pub struct TDataStd_Directory {
    id: [u8; 16],
}

impl TDataStd_Directory {
    /// Create a new Directory attribute.
    pub fn new() -> Self {
        Self {
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Directory attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_Directory
        [
            0xC4, 0xED, 0x42, 0x56, 0x1D, 0xB4, 0x4A, 0x58, 0xA3, 0xE0, 0x17, 0xA2, 0x22, 0x22,
            0x22, 0x22,
        ]
    }

    /// Get the ID of this attribute instance.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }
}

impl Default for TDataStd_Directory {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TDataStd_Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TDataStd_Directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_directory() {
        let dir = TDataStd_Directory::new();
        assert!(!dir.id().is_empty());
    }

    #[test]
    fn test_get_id() {
        let id1 = TDataStd_Directory::get_id();
        let id2 = TDataStd_Directory::get_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_default() {
        let dir = TDataStd_Directory::default();
        assert_eq!(dir.id(), &TDataStd_Directory::get_id());
    }

    #[test]
    fn test_display() {
        let dir = TDataStd_Directory::new();
        assert_eq!(dir.to_string(), "TDataStd_Directory");
    }
}
