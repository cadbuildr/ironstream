// FILE: osd_oem_type.rs
// occt: OSD_OEMType

/// OEM type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OemType {
    /// Windows
    Windows,
    /// Unix/Linux
    Unix,
}

impl OemType {
    /// Get string representation.
    pub fn as_str(&self) -> &str {
        match self {
            OemType::Windows => "Windows",
            OemType::Unix => "Unix",
        }
    }

    /// Get current platform OEM type.
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        {
            OemType::Windows
        }
        #[cfg(not(target_os = "windows"))]
        {
            OemType::Unix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oem_type_variants() {
        assert_eq!(OemType::Windows.as_str(), "Windows");
        assert_eq!(OemType::Unix.as_str(), "Unix");
    }

    #[test]
    fn test_current_platform() {
        let current = OemType::current();
        assert!(current == OemType::Windows || current == OemType::Unix);
    }
}
