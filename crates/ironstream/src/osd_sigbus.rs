// FILE: osd_sigbus.rs
// occt: OSD_SIGBUS

use crate::osd_signal::OsdSignal;

/// Exception class for bus error signal (SIGBUS)
#[derive(Debug, Clone)]
pub struct OsdSigbus {
    signal: OsdSignal,
}

impl OsdSigbus {
    /// Creates a new OsdSigbus exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigbus {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigbus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGBUS: {}", self.message())
    }
}

impl std::error::Error for OsdSigbus {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigbus_new() {
        let sig = OsdSigbus::new("bus error");
        assert_eq!(sig.message(), "bus error");
    }

    #[test]
    fn test_osd_sigbus_display() {
        let sig = OsdSigbus::new("misaligned access");
        assert_eq!(sig.to_string(), "OSD_SIGBUS: misaligned access");
    }
}
