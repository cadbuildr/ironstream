// FILE: fsd_file.rs
// occt: FSD_File

/// File storage driver for persistent storage operations.
pub struct FsdFile {
    name: String,
    is_open: bool,
}

impl FsdFile {
    /// Constructs a file object.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            is_open: false,
        }
    }

    /// Opens a file.
    pub fn open(name: &str) -> Result<Self, &'static str> {
        Ok(Self {
            name: name.to_string(),
            is_open: true,
        })
    }

    /// Closes the file.
    pub fn close(&mut self) -> Result<(), &'static str> {
        self.is_open = false;
        Ok(())
    }

    /// Returns the file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the file is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Default for FsdFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let file = FsdFile::new();
        assert!(!file.is_open());
        assert!(file.name().is_empty());
    }

    #[test]
    fn test_open() {
        let file = FsdFile::open("test.dat").unwrap();
        assert!(file.is_open());
        assert_eq!(file.name(), "test.dat");
    }

    #[test]
    fn test_close() {
        let mut file = FsdFile::open("test.dat").unwrap();
        assert!(file.is_open());
        file.close().unwrap();
        assert!(!file.is_open());
    }
}
