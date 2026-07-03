// FILE: standard_out_of_memory.rs
// occt: Standard_OutOfMemory

//! Exception thrown when memory allocation fails.
//! This exception uses a fixed buffer to avoid memory allocation during throw.

use std::fmt;

/// Exception thrown for out-of-memory conditions.
/// Uses a fixed-size buffer to avoid dynamic allocation.
#[derive(Debug, Clone)]
pub struct StandardOutOfMemory {
    message: [u8; 1024],
    len: usize,
}

impl StandardOutOfMemory {
    /// Create a new out-of-memory exception with optional message.
    pub fn new(message: Option<&str>) -> Self {
        let mut buffer = [0u8; 1024];
        let len = if let Some(msg) = message {
            let bytes = msg.as_bytes();
            let copy_len = bytes.len().min(1023);
            buffer[..copy_len].copy_from_slice(&bytes[..copy_len]);
            copy_len
        } else {
            0
        };

        StandardOutOfMemory {
            message: buffer,
            len,
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        std::str::from_utf8(&self.message[..self.len]).unwrap_or("")
    }

    /// Set the error message.
    pub fn set_message(&mut self, message: &str) {
        let bytes = message.as_bytes();
        let copy_len = bytes.len().min(1023);
        self.message[..copy_len].copy_from_slice(&bytes[..copy_len]);
        self.len = copy_len;
    }

    pub fn exception_type(&self) -> &str {
        "Standard_OutOfMemory"
    }
}

impl fmt::Display for StandardOutOfMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message())
    }
}

impl std::error::Error for StandardOutOfMemory {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_memory_creation_no_message() {
        let err = StandardOutOfMemory::new(None);
        assert_eq!(err.message(), "");
        assert_eq!(err.exception_type(), "Standard_OutOfMemory");
    }

    #[test]
    fn test_out_of_memory_creation_with_message() {
        let err = StandardOutOfMemory::new(Some("Failed to allocate"));
        assert_eq!(err.message(), "Failed to allocate");
    }

    #[test]
    fn test_out_of_memory_set_message() {
        let mut err = StandardOutOfMemory::new(Some("Original"));
        err.set_message("Updated");
        assert_eq!(err.message(), "Updated");
    }

    #[test]
    fn test_out_of_memory_display() {
        let err = StandardOutOfMemory::new(Some("Memory allocation failed"));
        assert_eq!(
            err.to_string(),
            "Standard_OutOfMemory: Memory allocation failed"
        );
    }

    #[test]
    fn test_out_of_memory_long_message() {
        let long_msg = "a".repeat(2000); // Longer than buffer
        let err = StandardOutOfMemory::new(Some(&long_msg));
        assert_eq!(err.message().len(), 1023); // Truncated to buffer size
    }
}
