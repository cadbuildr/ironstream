// FILE: xs_control_writer.rs
// occt: XSControl_Writer

/// Writer for exchange format files using the control framework.
/// Orchestrates entity transfer and file writing.
#[derive(Clone, Debug)]
pub struct XSControlWriter {
    /// Writer identifier
    writer_id: u32,
    /// Output file name
    filename: String,
    /// Write mode (0=default, 1=append, 2=overwrite)
    mode: u32,
}

impl XSControlWriter {
    /// Creates a new writer.
    pub fn new() -> Self {
        Self {
            writer_id: 0,
            filename: String::new(),
            mode: 2, // default overwrite
        }
    }

    /// Returns the writer ID.
    pub fn id(&self) -> u32 {
        self.writer_id
    }

    /// Returns the output filename.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Sets the output filename.
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = String::from(filename);
    }

    /// Returns the write mode.
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// Sets the write mode.
    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }
}

impl Default for XSControlWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let writer = XSControlWriter::new();
        assert_eq!(writer.id(), 0);
        assert_eq!(writer.filename(), "");
        assert_eq!(writer.mode(), 2);
    }

    #[test]
    fn test_set_filename() {
        let mut writer = XSControlWriter::new();
        writer.set_filename("output.stp");
        assert_eq!(writer.filename(), "output.stp");
    }

    #[test]
    fn test_set_mode() {
        let mut writer = XSControlWriter::new();
        writer.set_mode(0);
        assert_eq!(writer.mode(), 0);

        writer.set_mode(1);
        assert_eq!(writer.mode(), 1);
    }
}
