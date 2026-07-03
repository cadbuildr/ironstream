// FILE: osd_sigquit.rs
// occt: OSD_SIGQUIT

use crate::osd_signal::OsdSignal;

/// Exception class for quit signal (SIGQUIT)
#[derive(Debug, Clone)]
pub struct OsdSigquit {
    signal: OsdSignal,
}

impl OsdSigquit {
    /// Creates a new OsdSigquit exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigquit {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigquit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGQUIT: {}", self.message())
    }
}

impl std::error::Error for OsdSigquit {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigquit_new() {
        let sig = OsdSigquit::new("quit signal");
        assert_eq!(sig.message(), "quit signal");
    }

    #[test]
    fn test_osd_sigquit_display() {
        let sig = OsdSigquit::new("user quit");
        assert_eq!(sig.to_string(), "OSD_SIGQUIT: user quit");
    }
}
