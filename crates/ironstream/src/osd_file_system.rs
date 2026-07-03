// FILE: osd_file_system.rs
// occt: OSD_FileSystem

/// Abstract file system interface.
pub trait FileSystem {
    /// Get file system name.
    fn name(&self) -> &str;

    /// Check if path exists.
    fn exists(&self, path: &str) -> bool;

    /// Create directory.
    fn create_dir(&self, path: &str) -> std::io::Result<()>;

    /// Remove file.
    fn remove_file(&self, path: &str) -> std::io::Result<()>;

    /// Remove directory.
    fn remove_dir(&self, path: &str) -> std::io::Result<()>;
}

/// Default local file system implementation.
pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn name(&self) -> &str {
        "LocalFileSystem"
    }

    fn exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn create_dir(&self, path: &str) -> std::io::Result<()> {
        std::fs::create_dir(path)
    }

    fn remove_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }

    fn remove_dir(&self, path: &str) -> std::io::Result<()> {
        std::fs::remove_dir(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_file_system() {
        let fs = LocalFileSystem;
        assert_eq!(fs.name(), "LocalFileSystem");
    }

    #[test]
    fn test_exists() {
        let fs = LocalFileSystem;
        assert!(!fs.exists("/nonexistent/path/xyz"));
    }
}
