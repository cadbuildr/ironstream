// FILE: interface_file_reader_tool.rs
// occt: Interface_FileReaderTool

/// Tool for reading files in interface format.
#[derive(Clone, Debug)]
pub struct InterfaceFileReaderTool {
    file_name: String,
}

impl InterfaceFileReaderTool {
    /// Creates a FileReaderTool
    pub fn new(file_name: String) -> Self {
        Self { file_name }
    }

    /// Returns the file name
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tool = InterfaceFileReaderTool::new("test.iges".to_string());
        assert_eq!(tool.file_name(), "test.iges");
    }
}
