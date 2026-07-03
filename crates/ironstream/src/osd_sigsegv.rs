// FILE: osd_sigsegv.rs
// occt: OSD_SIGSEGV

use crate::osd_signal::OsdSignal;

/// Exception class for segmentation violation signal (SIGSEGV)
#[derive(Debug, Clone)]
pub struct OsdSigsegv {
    signal: OsdSignal,
}

impl OsdSigsegv {
    /// Creates a new OsdSigsegv exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        OsdSigsegv {
            signal: OsdSignal::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.signal.message()
    }
}

impl std::fmt::Display for OsdSigsegv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSD_SIGSEGV: {}", self.message())
    }
}

impl std::error::Error for OsdSigsegv {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_sigsegv_new() {
        let sig = OsdSigsegv::new("segmentation violation");
        assert_eq!(sig.message(), "segmentation violation");
    }

    #[test]
    fn test_osd_sigsegv_display() {
        let sig = OsdSigsegv::new("invalid memory access");
        assert_eq!(sig.to_string(), "OSD_SIGSEGV: invalid memory access");
    }
}
