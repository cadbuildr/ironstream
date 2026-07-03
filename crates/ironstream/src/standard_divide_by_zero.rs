// FILE: standard_divide_by_zero.rs
// occt: Standard_DivideByZero

use crate::standard_domain_error::StandardDomainError;

/// Exception class for division by zero errors
#[derive(Debug, Clone)]
pub struct StandardDivideByZero {
    error: StandardDomainError,
}

impl StandardDivideByZero {
    /// Creates a new StandardDivideByZero exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        StandardDivideByZero {
            error: StandardDomainError::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.error.message()
    }
}

impl std::fmt::Display for StandardDivideByZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_DivideByZero: {}", self.message())
    }
}

impl std::error::Error for StandardDivideByZero {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_divide_by_zero() {
        let err = StandardDivideByZero::new("division by zero");
        assert_eq!(err.message(), "division by zero");
    }
}
