// FILE: osd_sigkill.rs
// occt: OSD_SIGKILL

use crate::osd_signal::OsdSignal;

/// Exception class for kill signal (SIGKILL)
#[derive(Debug, Clone)]
pub struct OsdSigkill {
    signal: OsdSignal,
}

impl OsdSigkill {
    /// Creates a new OsdSigkill exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigkill {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigkill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGKILL: {}", self.message())
    }
}

impl std::error::Error for OsdSigkill {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigkill_new() {
        let sig = OsdSigkill::new("kill signal");
        assert_eq!(sig.message(), "kill signal");
    }

    #[test]
    fn test_osd_sigkill_display() {
        let sig = OsdSigkill::new("process termination");
        assert_eq!(sig.to_string(), "OSD_SIGKILL: process termination");
    }
}
