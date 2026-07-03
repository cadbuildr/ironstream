// FILE: gcc_ent_bad_qualifier.rs
// occt: GccEnt_BadQualifier

use std::fmt;

/// Exception raised when a qualifier is inconsistent with its argument.
#[derive(Debug, Clone)]
pub struct GccEntBadQualifier {
    message: String,
}

impl GccEntBadQualifier {
    pub fn new(msg: &str) -> Self {
        GccEntBadQualifier {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for GccEntBadQualifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GccEnt_BadQualifier: {}", self.message)
    }
}

impl std::error::Error for GccEntBadQualifier {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_qualifier_creation() {
        let exc = GccEntBadQualifier::new("Inconsistent qualifier");
        assert_eq!(exc.message, "Inconsistent qualifier");
    }

    #[test]
    fn test_bad_qualifier_display() {
        let exc = GccEntBadQualifier::new("enclosed for a circle");
        let s = format!("{}", exc);
        assert!(s.contains("enclosed for a circle"));
    }
}
