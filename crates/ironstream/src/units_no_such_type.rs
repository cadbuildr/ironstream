// FILE: units_no_such_type.rs
// occt: Units_NoSuchType

use std::fmt;

/// Exception for unknown unit types
#[derive(Debug, Clone)]
pub struct UnitsNoSuchType {
    message: String,
}

impl UnitsNoSuchType {
    pub fn new(message: impl Into<String>) -> Self {
        UnitsNoSuchType {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for UnitsNoSuchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Units: no such type: {}", self.message)
    }
}

impl std::error::Error for UnitsNoSuchType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_no_such_type() {
        let err = UnitsNoSuchType::new("UnknownType");
        assert_eq!(err.message(), "UnknownType");
    }

    #[test]
    fn test_units_no_such_type_display() {
        let err = UnitsNoSuchType::new("BadType");
        let s = format!("{}", err);
        assert!(s.contains("BadType"));
    }
}
