// FILE: standard_overflow.rs
// occt: Standard_Overflow

//! Exception thrown when numeric overflow occurs.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardOverflow {
    message: String,
}

impl StandardOverflow {
    pub fn new(message: impl Into<String>) -> Self {
        StandardOverflow {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_Overflow"
    }
}

impl fmt::Display for StandardOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardOverflow {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_creation() {
        let err = StandardOverflow::new("Integer overflow");
        assert_eq!(err.message(), "Integer overflow");
    }

    #[test]
    fn test_overflow_display() {
        let err = StandardOverflow::new("Result too large");
        assert_eq!(err.to_string(), "Standard_Overflow: Result too large");
    }

    #[test]
    fn test_overflow_clone() {
        let err = StandardOverflow::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
