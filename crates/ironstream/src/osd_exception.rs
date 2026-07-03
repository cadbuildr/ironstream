// FILE: osd_exception.rs
// occt: OSD_Exception

/// Base exception class for OSD.
#[derive(Debug, Clone)]
pub struct Exception {
    message: String,
}

impl Exception {
    /// Create new exception.
    pub fn new(msg: &str) -> Self {
        Exception {
            message: msg.to_string(),
        }
    }

    /// Get exception message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OSD_Exception: {}", self.message)
    }
}

impl std::error::Error for Exception {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exception() {
        let exc = Exception::new("Test exception");
        assert_eq!(exc.message(), "Test exception");
    }

    #[test]
    fn test_display() {
        let exc = Exception::new("Error message");
        let s = format!("{}", exc);
        assert!(s.contains("OSD_Exception"));
        assert!(s.contains("Error message"));
    }
}
