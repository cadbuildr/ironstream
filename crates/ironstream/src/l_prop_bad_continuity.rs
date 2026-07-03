// FILE: l_prop_bad_continuity.rs
// occt: LProp_BadContinuity

/// Exception for bad continuity conditions.
#[derive(Debug, Clone)]
pub struct LPropBadContinuity {
    message: String,
}

impl LPropBadContinuity {
    /// Create a new bad continuity exception.
    pub fn new(message: &str) -> Self {
        LPropBadContinuity {
            message: message.to_string(),
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for LPropBadContinuity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LProp_BadContinuity: {}", self.message)
    }
}

impl std::error::Error for LPropBadContinuity {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_continuity_creation() {
        let exc = LPropBadContinuity::new("Test error");
        assert_eq!(exc.message(), "Test error");
    }

    #[test]
    fn test_bad_continuity_display() {
        let exc = LPropBadContinuity::new("Invalid continuity");
        let s = format!("{}", exc);
        assert!(s.contains("Invalid continuity"));
    }

    #[test]
    fn test_bad_continuity_debug() {
        let exc = LPropBadContinuity::new("Test");
        let debug_str = format!("{:?}", exc);
        assert!(debug_str.contains("Test"));
    }
}
