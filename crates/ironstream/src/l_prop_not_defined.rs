// FILE: l_prop_not_defined.rs
// occt: LProp_NotDefined

/// Exception for undefined properties.
#[derive(Debug, Clone)]
pub struct LPropNotDefined {
    message: String,
}

impl LPropNotDefined {
    /// Create a new not defined exception.
    pub fn new(message: &str) -> Self {
        LPropNotDefined {
            message: message.to_string(),
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for LPropNotDefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LProp_NotDefined: {}", self.message)
    }
}

impl std::error::Error for LPropNotDefined {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_defined_creation() {
        let exc = LPropNotDefined::new("Property not computed");
        assert_eq!(exc.message(), "Property not computed");
    }

    #[test]
    fn test_not_defined_display() {
        let exc = LPropNotDefined::new("Curvature undefined");
        let s = format!("{}", exc);
        assert!(s.contains("Curvature undefined"));
    }

    #[test]
    fn test_not_defined_debug() {
        let exc = LPropNotDefined::new("Test");
        let debug_str = format!("{:?}", exc);
        assert!(debug_str.contains("Test"));
    }
}
