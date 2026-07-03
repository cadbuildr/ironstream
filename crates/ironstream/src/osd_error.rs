// FILE: osd_error.rs
// occt: OSD_Error

/// Represents an OS error state.
#[derive(Debug, Clone)]
pub struct Error {
    code: i32,
    message: String,
}

impl Error {
    /// Create error from code and message.
    pub fn new(code: i32, message: &str) -> Self {
        Error {
            code,
            message: message.to_string(),
        }
    }

    /// Get error code.
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Get error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Check if error is set.
    pub fn is_error(&self) -> bool {
        self.code != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_error() {
        let err = Error::new(1, "Test error");
        assert_eq!(err.code(), 1);
        assert_eq!(err.message(), "Test error");
    }

    #[test]
    fn test_is_error() {
        let err = Error::new(1, "Error");
        assert!(err.is_error());
        let ok = Error::new(0, "OK");
        assert!(!ok.is_error());
    }
}
