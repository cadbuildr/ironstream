// FILE: standard_s_stream.rs
// occt: Standard_SStream

//! String stream for in-memory string I/O.
//! Rust equivalent of C++ std::stringstream.

use std::fmt;

/// A string stream for in-memory reading and writing.
/// Rust equivalent of Standard_SStream (std::stringstream).
#[derive(Debug, Clone)]
pub struct StandardSStream {
    buffer: String,
}

impl StandardSStream {
    /// Create a new string stream.
    pub fn new() -> Self {
        StandardSStream {
            buffer: String::new(),
        }
    }

    /// Create a string stream with initial content.
    pub fn with_content(content: impl Into<String>) -> Self {
        StandardSStream {
            buffer: content.into(),
        }
    }

    /// Write to the stream.
    pub fn write(&mut self, data: &str) {
        self.buffer.push_str(data);
    }

    /// Get the current content as a string.
    pub fn get_string(&self) -> &str {
        &self.buffer
    }

    /// Consume the stream and return its content.
    pub fn into_string(self) -> String {
        self.buffer
    }

    /// Clear the stream.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get the length of the stream content.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the stream is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl Default for StandardSStream {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StandardSStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sstream_creation() {
        let ss = StandardSStream::new();
        assert!(ss.is_empty());
        assert_eq!(ss.len(), 0);
    }

    #[test]
    fn test_sstream_write() {
        let mut ss = StandardSStream::new();
        ss.write("Hello");
        ss.write(" World");
        assert_eq!(ss.get_string(), "Hello World");
    }

    #[test]
    fn test_sstream_with_content() {
        let ss = StandardSStream::with_content("Initial");
        assert_eq!(ss.get_string(), "Initial");
        assert_eq!(ss.len(), 7);
    }

    #[test]
    fn test_sstream_clear() {
        let mut ss = StandardSStream::with_content("Content");
        ss.clear();
        assert!(ss.is_empty());
    }

    #[test]
    fn test_sstream_into_string() {
        let ss = StandardSStream::with_content("Data");
        let s = ss.into_string();
        assert_eq!(s, "Data");
    }

    #[test]
    fn test_sstream_display() {
        let ss = StandardSStream::with_content("Test");
        assert_eq!(ss.to_string(), "Test");
    }

    #[test]
    fn test_sstream_clone() {
        let ss1 = StandardSStream::with_content("Clone");
        let ss2 = ss1.clone();
        assert_eq!(ss1.get_string(), ss2.get_string());
    }
}
