// FILE: interface_check_failure.rs
// occt: Interface_CheckFailure

/// An exception type for check failures in data exchange.
#[derive(Clone, Debug)]
pub struct InterfaceCheckFailure {
    message: String,
}

impl InterfaceCheckFailure {
    /// Creates a check failure exception
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Returns the failure message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for InterfaceCheckFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Check Failure: {}", self.message)
    }
}

impl std::error::Error for InterfaceCheckFailure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let failure = InterfaceCheckFailure::new("Test error".to_string());
        assert_eq!(failure.message(), "Test error");
    }

    #[test]
    fn test_display() {
        let failure = InterfaceCheckFailure::new("Error message".to_string());
        assert_eq!(failure.to_string(), "Check Failure: Error message");
    }
}
