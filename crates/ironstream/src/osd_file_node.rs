// FILE: osd_file_node.rs
// occt: OSD_FileNode

/// File node representation.
#[derive(Clone, Debug)]
pub struct FileNode {
    path: String,
}

impl FileNode {
    /// Create file node from path.
    pub fn new(path: &str) -> Self {
        FileNode {
            path: path.to_string(),
        }
    }

    /// Get file path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get file name.
    pub fn name(&self) -> &str {
        std::path::Path::new(&self.path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    /// Check if exists.
    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    /// Get file size.
    pub fn size(&self) -> std::io::Result<u64> {
        std::fs::metadata(&self.path).map(|m| m.len())
    }

    /// Check if is file.
    pub fn is_file(&self) -> bool {
        std::path::Path::new(&self.path).is_file()
    }

    /// Check if is directory.
    pub fn is_directory(&self) -> bool {
        std::path::Path::new(&self.path).is_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_node() {
        let node = FileNode::new("/tmp/test.txt");
        assert_eq!(node.path(), "/tmp/test.txt");
        assert_eq!(node.name(), "test.txt");
    }

    #[test]
    fn test_non_existent_file() {
        let node = FileNode::new("/nonexistent/file/path/xyz");
        assert!(!node.exists());
        assert!(!node.is_file());
    }
}
