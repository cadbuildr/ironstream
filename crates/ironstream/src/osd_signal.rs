// FILE: osd_signal.rs
// occt: OSD_Signal

//! Exception class for OSD signal-related errors.

/// Base exception for OSD signal operations
#[derive(Debug, Clone)]
pub struct OsdSignal {
    message: String,
}

impl OsdSignal {
    /// Creates a new OsdSignal exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSignal {
            message: message.into(),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for OsdSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_Signal: {}", self.message)
    }
}

impl std::error::Error for OsdSignal {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_signal_new() {
        let sig = OsdSignal::new("test message");
        assert_eq!(sig.message(), "test message");
    }

    #[test]
    fn test_osd_signal_display() {
        let sig = OsdSignal::new("error occurred");
        assert_eq!(sig.to_string(), "OSD_Signal: error occurred");
    }

    #[test]
    fn test_osd_signal_from_string() {
        let sig = OsdSignal::new("another error".to_string());
        assert_eq!(sig.message(), "another error");
    }
}
