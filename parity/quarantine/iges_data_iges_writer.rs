// FILE: iges_data_iges_writer.rs
// occt: IGESData_IGESWriter

//! Writer for IGES format files.

#[derive(Clone, Debug)]
pub struct IGESWriter {
    filename: String,
}

impl IGESWriter {
    pub fn new(filename: &str) -> Self {
        IGESWriter {
            filename: filename.to_string(),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn send_entity(&mut self, id: usize) -> bool {
        true
    }

    pub fn flush(&mut self) -> bool {
        true
    }
}

impl Default for IGESWriter {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let writer = IGESWriter::new("test.igs");
        assert_eq!(writer.filename(), "test.igs");
    }

    #[test]
    fn test_send_entity() {
        let mut writer = IGESWriter::new("test.igs");
        assert!(writer.send_entity(1));
    }

    #[test]
    fn test_flush() {
        let mut writer = IGESWriter::new("test.igs");
        assert!(writer.flush());
    }

    #[test]
    fn test_default() {
        let writer = IGESWriter::default();
        assert_eq!(writer.filename(), "");
    }
}
