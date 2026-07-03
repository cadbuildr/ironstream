// FILE: osd_sigill.rs
// occt: OSD_SIGILL

use crate::osd_signal::OsdSignal;

/// Exception class for illegal instruction signal (SIGILL)
#[derive(Debug, Clone)]
pub struct OsdSigill {
    signal: OsdSignal,
}

impl OsdSigill {
    /// Creates a new OsdSigill exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigill {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGILL: {}", self.message())
    }
}

impl std::error::Error for OsdSigill {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigill_new() {
        let sig = OsdSigill::new("illegal instruction");
        assert_eq!(sig.message(), "illegal instruction");
    }

    #[test]
    fn test_osd_sigill_display() {
        let sig = OsdSigill::new("invalid opcode");
        assert_eq!(sig.to_string(), "OSD_SIGILL: invalid opcode");
    }
}
