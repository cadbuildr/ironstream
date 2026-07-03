// FILE: quantity_period_definition_error.rs
// occt: Quantity_PeriodDefinitionError

use crate::standard_domain_error::StandardDomainError;

/// Exception class for period definition errors
#[derive(Debug, Clone)]
pub struct QuantityPeriodDefinitionError {
    error: StandardDomainError,
}

impl QuantityPeriodDefinitionError {
    /// Creates a new QuantityPeriodDefinitionError exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        QuantityPeriodDefinitionError {
            error: StandardDomainError::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.error.message()
    }
}

impl std::fmt::Display for QuantityPeriodDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quantity_PeriodDefinitionError: {}", self.message())
    }
}

impl std::error::Error for QuantityPeriodDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_period_definition_error() {
        let err = QuantityPeriodDefinitionError::new("invalid period");
        assert_eq!(err.message(), "invalid period");
    }
}
