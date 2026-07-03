// FILE: standard_abortive_transaction.rs
// occt: Standard_AbortiveTransaction

/// Exception class for abortive transactions
#[derive(Debug, Clone)]
pub struct StandardAbortiveTransaction {
    message: String,
}

impl StandardAbortiveTransaction {
    /// Creates a new StandardAbortiveTransaction exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        StandardAbortiveTransaction {
            message: message.into(),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for StandardAbortiveTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_AbortiveTransaction: {}", self.message)
    }
}

impl std::error::Error for StandardAbortiveTransaction {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_abortive_transaction() {
        let err = StandardAbortiveTransaction::new("transaction aborted");
        assert_eq!(err.message(), "transaction aborted");
    }
}
