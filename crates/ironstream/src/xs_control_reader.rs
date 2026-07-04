// FILE: xs_control_reader.rs
// occt: XSControl_Reader

/// Reader for exchange format files using the control framework.
/// Orchestrates file reading and entity transfer.
#[derive(Clone, Debug)]
pub struct XSControlReader {
    /// Reader identifier
    reader_id: u32,
    /// Whether a file is loaded
    is_loaded: bool,
    /// File name
    filename: String,
}

impl XSControlReader {
    /// Creates a new reader.
    pub fn new() -> Self {
        Self {
            reader_id: 0,
            is_loaded: false,
            filename: String::new(),
        }
    }

    /// Returns the reader ID.
    pub fn id(&self) -> u32 {
        self.reader_id
    }

    /// Returns whether a file is loaded.
    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    /// Returns the filename.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Sets the filename and marks file as loaded.
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = String::from(filename);
        self.is_loaded = true;
    }

    /// Clears the loaded file.
    pub fn clear(&mut self) {
        self.filename.clear();
        self.is_loaded = false;
    }
}

impl Default for XSControlReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reader = XSControlReader::new();
        assert!(!reader.is_loaded());
        assert_eq!(reader.filename(), "");
    }

    #[test]
    fn test_set_filename() {
        let mut reader = XSControlReader::new();
        reader.set_filename("test.stp");
        assert!(reader.is_loaded());
        assert_eq!(reader.filename(), "test.stp");
    }

    #[test]
    fn test_clear() {
        let mut reader = XSControlReader::new();
        reader.set_filename("test.stp");
        assert!(reader.is_loaded());

        reader.clear();
        assert!(!reader.is_loaded());
        assert_eq!(reader.filename(), "");
    }
}
