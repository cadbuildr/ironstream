// FILE: units_no_such_unit.rs
// occt: Units_NoSuchUnit

use std::fmt;

/// Exception for unknown units
#[derive(Debug, Clone)]
pub struct UnitsNoSuchUnit {
    message: String,
}

impl UnitsNoSuchUnit {
    pub fn new(message: impl Into<String>) -> Self {
        UnitsNoSuchUnit {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for UnitsNoSuchUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Units: no such unit: {}", self.message)
    }
}

impl std::error::Error for UnitsNoSuchUnit {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_no_such_unit() {
        let err = UnitsNoSuchUnit::new("UnknownUnit");
        assert_eq!(err.message(), "UnknownUnit");
    }

    #[test]
    fn test_units_no_such_unit_display() {
        let err = UnitsNoSuchUnit::new("BadUnit");
        let s = format!("{}", err);
        assert!(s.contains("BadUnit"));
    }
}
