// FILE: fsd_f_stream.rs
// occt: FSD_FStream

/// File stream for file storage operations.
pub struct FsdFStream {
    path: String,
    mode: FileMode,
    is_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Read,
    Write,
    Append,
}

impl FsdFStream {
    /// Constructs an unopened file stream.
    pub fn new() -> Self {
        Self {
            path: String::new(),
            mode: FileMode::Read,
            is_open: false,
        }
    }

    /// Opens a file stream.
    pub fn open(path: &str, mode: FileMode) -> Result<Self, &'static str> {
        Ok(Self {
            path: path.to_string(),
            mode,
            is_open: true,
        })
    }

    /// Returns the file path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the file mode.
    pub fn mode(&self) -> FileMode {
        self.mode
    }

    /// Returns whether the stream is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Closes the stream.
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Writes data to the stream.
    pub fn write(&self, _data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_open || self.mode == FileMode::Read {
            Err("Stream not open for writing")
        } else {
            Ok(0) // Stub
        }
    }

    /// Reads data from the stream.
    pub fn read(&self, _len: usize) -> Result<Vec<u8>, &'static str> {
        if !self.is_open || self.mode == FileMode::Write {
            Err("Stream not open for reading")
        } else {
            Ok(Vec::new()) // Stub
        }
    }
}

impl Default for FsdFStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FsdFStream {
    fn drop(&mut self) {
        if self.is_open {
            self.close();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stream = FsdFStream::new();
        assert!(!stream.is_open());
        assert!(stream.path().is_empty());
    }

    #[test]
    fn test_open_for_write() {
        let stream = FsdFStream::open("test.dat", FileMode::Write).unwrap();
        assert!(stream.is_open());
        assert_eq!(stream.path(), "test.dat");
        assert_eq!(stream.mode(), FileMode::Write);
    }

    #[test]
    fn test_mode_validation() {
        let stream = FsdFStream::open("test.dat", FileMode::Read).unwrap();
        let result = stream.write(b"data");
        assert!(result.is_err());
    }

    #[test]
    fn test_close() {
        let mut stream = FsdFStream::open("test.dat", FileMode::Write).unwrap();
        assert!(stream.is_open());
        stream.close();
        assert!(!stream.is_open());
    }
}
