// FILE: osd_sigsys.rs
// occt: OSD_SIGSYS

use crate::osd_signal::OsdSignal;

/// Exception class for bad system call signal (SIGSYS)
#[derive(Debug, Clone)]
pub struct OsdSigsys {
    signal: OsdSignal,
}

impl OsdSigsys {
    /// Creates a new OsdSigsys exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigsys {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigsys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGSYS: {}", self.message())
    }
}

impl std::error::Error for OsdSigsys {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigsys_new() {
        let sig = OsdSigsys::new("bad system call");
        assert_eq!(sig.message(), "bad system call");
    }

    #[test]
    fn test_osd_sigsys_display() {
        let sig = OsdSigsys::new("invalid syscall");
        assert_eq!(sig.to_string(), "OSD_SIGSYS: invalid syscall");
    }
}
