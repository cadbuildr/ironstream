// FILE: standard_underflow.rs
// occt: Standard_Underflow

//! Exception thrown when numeric underflow occurs.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardUnderflow {
    message: String,
}

impl StandardUnderflow {
    pub fn new(message: impl Into<String>) -> Self {
        StandardUnderflow {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_Underflow"
    }
}

impl fmt::Display for StandardUnderflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardUnderflow {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underflow_creation() {
        let err = StandardUnderflow::new("Numeric underflow");
        assert_eq!(err.message(), "Numeric underflow");
    }

    #[test]
    fn test_underflow_display() {
        let err = StandardUnderflow::new("Value too small");
        assert_eq!(err.to_string(), "Standard_Underflow: Value too small");
    }

    #[test]
    fn test_underflow_clone() {
        let err = StandardUnderflow::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
