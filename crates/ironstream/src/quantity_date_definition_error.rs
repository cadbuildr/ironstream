// FILE: quantity_date_definition_error.rs
// occt: Quantity_DateDefinitionError

use crate::standard_domain_error::StandardDomainError;

/// Exception class for date definition errors
#[derive(Debug, Clone)]
pub struct QuantityDateDefinitionError {
    error: StandardDomainError,
}

impl QuantityDateDefinitionError {
    /// Creates a new QuantityDateDefinitionError exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        QuantityDateDefinitionError {
            error: StandardDomainError::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.error.message()
    }
}

impl std::fmt::Display for QuantityDateDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quantity_DateDefinitionError: {}", self.message())
    }
}

impl std::error::Error for QuantityDateDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_date_definition_error() {
        let err = QuantityDateDefinitionError::new("invalid date");
        assert_eq!(err.message(), "invalid date");
    }
}
