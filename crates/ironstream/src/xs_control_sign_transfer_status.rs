// FILE: xs_control_sign_transfer_status.rs
// occt: XSControl_SignTransferStatus

/// Sign/signature provider for transfer operation status information.
/// Provides status codes and status descriptions for transfers.
#[derive(Clone, Debug)]
pub struct XSControlSignTransferStatus {
    /// Status value
    status: u32,
    /// Status description
    description: String,
}

impl XSControlSignTransferStatus {
    /// Creates a new transfer status sign.
    pub fn new(status: u32, description: &str) -> Self {
        Self {
            status,
            description: String::from(description),
        }
    }

    /// Returns the status value.
    pub fn status(&self) -> u32 {
        self.status
    }

    /// Returns the status description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Gets a sign for a common status.
    pub fn sign_for_status(status: u32) -> Option<&'static str> {
        match status {
            0 => Some("OK"),
            1 => Some("Warning"),
            2 => Some("Error"),
            3 => Some("Skipped"),
            _ => None,
        }
    }
}

impl Default for XSControlSignTransferStatus {
    fn default() -> Self {
        Self::new(0, "OK")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sign = XSControlSignTransferStatus::new(2, "Error occurred");
        assert_eq!(sign.status(), 2);
        assert_eq!(sign.description(), "Error occurred");
    }

    #[test]
    fn test_default() {
        let sign = XSControlSignTransferStatus::default();
        assert_eq!(sign.status(), 0);
        assert_eq!(sign.description(), "OK");
    }

    #[test]
    fn test_sign_for_status() {
        assert_eq!(XSControlSignTransferStatus::sign_for_status(0), Some("OK"));
        assert_eq!(XSControlSignTransferStatus::sign_for_status(1), Some("Warning"));
        assert_eq!(XSControlSignTransferStatus::sign_for_status(2), Some("Error"));
        assert_eq!(XSControlSignTransferStatus::sign_for_status(99), None);
    }
}
