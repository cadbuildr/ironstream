// FILE: osd_local_file_system.rs
// occt: OSD_LocalFileSystem

/// Local file system implementation.
pub struct LocalFileSystem;

impl LocalFileSystem {
    /// Create new local file system instance.
    pub fn new() -> Self {
        LocalFileSystem
    }

    /// Get absolute path.
    pub fn absolute_path(path: &str) -> std::io::Result<String> {
        let path_buf = std::fs::canonicalize(path)?;
        Ok(path_buf.to_string_lossy().to_string())
    }

    /// List directory contents.
    pub fn list_dir(path: &str) -> std::io::Result<Vec<String>> {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name() {
                entries.push(name.to_string_lossy().to_string());
            }
        }
        Ok(entries)
    }

    /// Get file information.
    pub fn file_size(path: &str) -> std::io::Result<u64> {
        Ok(std::fs::metadata(path)?.len())
    }

    /// Check if file.
    pub fn is_file(path: &str) -> bool {
        std::path::Path::new(path).is_file()
    }

    /// Check if directory.
    pub fn is_dir(path: &str) -> bool {
        std::path::Path::new(path).is_dir()
    }
}

impl Default for LocalFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_local_fs() {
        let fs = LocalFileSystem::new();
        let _ = fs;
    }

    #[test]
    fn test_is_file_nonexistent() {
        assert!(!LocalFileSystem::is_file("/nonexistent/xyz"));
    }

    #[test]
    fn test_is_dir() {
        assert!(LocalFileSystem::is_dir("/tmp"));
    }
}
