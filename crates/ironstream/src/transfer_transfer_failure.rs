// FILE: transfer_transfer_failure.rs
// occt: Transfer_TransferFailure

/// Exception raised when a transfer operation fails.
#[derive(Clone, Debug)]
pub struct TransferTransferFailure {
    /// Failure description
    message: String,
    /// Failure code
    code: u32,
}

impl TransferTransferFailure {
    /// Creates a new transfer failure exception.
    pub fn new(message: &str, code: u32) -> Self {
        Self {
            message: String::from(message),
            code,
        }
    }

    /// Returns the failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the failure code.
    pub fn code(&self) -> u32 {
        self.code
    }

    /// Default failure message.
    pub fn default_message() -> &'static str {
        "Transfer operation failed"
    }
}

impl Default for TransferTransferFailure {
    fn default() -> Self {
        Self::new(Self::default_message(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let failure = TransferTransferFailure::new("Invalid entity", 42);
        assert_eq!(failure.message(), "Invalid entity");
        assert_eq!(failure.code(), 42);
    }

    #[test]
    fn test_default() {
        let failure = TransferTransferFailure::default();
        assert_eq!(failure.message(), "Transfer operation failed");
        assert_eq!(failure.code(), 0);
    }

    #[test]
    fn test_default_message() {
        assert_eq!(
            TransferTransferFailure::default_message(),
            "Transfer operation failed"
        );
    }
}
