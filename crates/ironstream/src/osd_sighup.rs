// FILE: osd_sighup.rs
// occt: OSD_SIGHUP

use crate::osd_signal::OsdSignal;

/// Exception class for hangup signal (SIGHUP)
#[derive(Debug, Clone)]
pub struct OsdSighup {
    signal: OsdSignal,
}

impl OsdSighup {
    /// Creates a new OsdSighup exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSighup {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSighup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGHUP: {}", self.message())
    }
}

impl std::error::Error for OsdSighup {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sighup_new() {
        let sig = OsdSighup::new("terminal hangup");
        assert_eq!(sig.message(), "terminal hangup");
    }

    #[test]
    fn test_osd_sighup_display() {
        let sig = OsdSighup::new("connection lost");
        assert_eq!(sig.to_string(), "OSD_SIGHUP: connection lost");
    }
}
