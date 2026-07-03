// FILE: osd_sigint.rs
// occt: OSD_SIGINT

use crate::osd_signal::OsdSignal;

/// Exception class for interrupt signal (SIGINT)
#[derive(Debug, Clone)]
pub struct OsdSigint {
    signal: OsdSignal,
}

impl OsdSigint {
    /// Creates a new OsdSigint exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigint {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGINT: {}", self.message())
    }
}

impl std::error::Error for OsdSigint {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigint_new() {
        let sig = OsdSigint::new("interrupt signal");
        assert_eq!(sig.message(), "interrupt signal");
    }

    #[test]
    fn test_osd_sigint_display() {
        let sig = OsdSigint::new("user interrupt");
        assert_eq!(sig.to_string(), "OSD_SIGINT: user interrupt");
    }
}
