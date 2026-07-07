// FILE: transfer_transfer_dead_loop.rs
// occt: Transfer_TransferDeadLoop

/// Exception raised when a dead loop (circular dependency) is detected during transfer.
#[derive(Clone, Debug)]
pub struct TransferTransferDeadLoop {
    /// Description of the dead loop condition
    message: String,
}

impl TransferTransferDeadLoop {
    /// Creates a new dead loop exception.
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Default dead loop error message.
    pub fn default_message() -> &'static str {
        "Circular dependency detected in transfer process"
    }
}

impl Default for TransferTransferDeadLoop {
    fn default() -> Self {
        Self::new(Self::default_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let error = TransferTransferDeadLoop::new("Entity A -> B -> A");
        assert_eq!(error.message(), "Entity A -> B -> A");
    }

    #[test]
    fn test_default() {
        let error = TransferTransferDeadLoop::default();
        assert_eq!(
            error.message(),
            "Circular dependency detected in transfer process"
        );
    }

    #[test]
    fn test_default_message() {
        assert_eq!(
            TransferTransferDeadLoop::default_message(),
            "Circular dependency detected in transfer process"
        );
    }
}
