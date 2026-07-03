// FILE: osd_file_system_selector.rs
// occt: OSD_FileSystemSelector

/// Selector for appropriate file system implementation.
pub struct FileSystemSelector;

impl FileSystemSelector {
    /// Get default file system for platform.
    pub fn default_file_system() -> String {
        #[cfg(target_os = "windows")]
        {
            "Windows".to_string()
        }
        #[cfg(not(target_os = "windows"))]
        {
            "Unix".to_string()
        }
    }

    /// Check if file system is available.
    pub fn is_available(name: &str) -> bool {
        matches!(name, "Unix" | "Windows")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fs() {
        let fs = FileSystemSelector::default_file_system();
        assert!(!fs.is_empty());
    }

    #[test]
    fn test_is_available() {
        assert!(FileSystemSelector::is_available("Unix"));
        assert!(FileSystemSelector::is_available("Windows"));
        assert!(!FileSystemSelector::is_available("Unknown"));
    }
}
