// FILE: if_select_session_file.rs
// occt: IFSelect_SessionFile

/// Handles file-based persistence of sessions.
/// Works with SessionDumper library to read/write session data.
#[derive(Clone, Debug)]
pub struct IFSelectSessionFile {
    file_path: String,
}

impl IFSelectSessionFile {
    /// Creates a SessionFile for a given file path
    pub fn new(path: String) -> Self {
        Self { file_path: path }
    }

    /// Returns the file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let file = IFSelectSessionFile::new("session.txt".to_string());
        assert_eq!(file.file_path(), "session.txt");
    }

    #[test]
    fn test_different_paths() {
        let file1 = IFSelectSessionFile::new("file1.txt".to_string());
        let file2 = IFSelectSessionFile::new("file2.txt".to_string());
        assert_eq!(file1.file_path(), "file1.txt");
        assert_eq!(file2.file_path(), "file2.txt");
    }
}
