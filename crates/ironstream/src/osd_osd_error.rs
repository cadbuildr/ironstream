// FILE: osd_osd_error.rs
// occt: OSD_OSDError

/// OSD-specific error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsdError {
    /// No error
    NoError = 0,
    /// File not found
    FileNotFound = 1,
    /// Permission denied
    PermissionDenied = 2,
    /// Invalid argument
    InvalidArgument = 3,
    /// Disk full
    DiskFull = 4,
    /// I/O error
    IoError = 5,
}

impl OsdError {
    /// Get error message.
    pub fn message(&self) -> &str {
        match self {
            OsdError::NoError => "No error",
            OsdError::FileNotFound => "File not found",
            OsdError::PermissionDenied => "Permission denied",
            OsdError::InvalidArgument => "Invalid argument",
            OsdError::DiskFull => "Disk full",
            OsdError::IoError => "I/O error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_error_messages() {
        assert_eq!(OsdError::NoError.message(), "No error");
        assert_eq!(OsdError::FileNotFound.message(), "File not found");
    }

    #[test]
    fn test_equality() {
        assert_eq!(OsdError::NoError, OsdError::NoError);
        assert_ne!(OsdError::NoError, OsdError::FileNotFound);
    }
}
