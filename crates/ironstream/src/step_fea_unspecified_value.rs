// FILE: step_fea_unspecified_value.rs
// occt: StepFEA_UnspecifiedValue

//! An enum representing an unspecified value in FEA context.

/// Represents an unspecified value in FEA
/// This is used as a placeholder when a value is intentionally not specified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnspecifiedValue {
    /// Indicates an unspecified value
    Unspecified,
}

impl UnspecifiedValue {
    /// Get the unspecified value
    pub fn unspecified() -> Self {
        UnspecifiedValue::Unspecified
    }

    /// Check if this is unspecified
    pub fn is_unspecified(&self) -> bool {
        matches!(self, UnspecifiedValue::Unspecified)
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            UnspecifiedValue::Unspecified => "Unspecified",
        }
    }
}

impl Default for UnspecifiedValue {
    fn default() -> Self {
        UnspecifiedValue::Unspecified
    }
}

impl std::fmt::Display for UnspecifiedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unspecified() {
        let val = UnspecifiedValue::unspecified();
        assert!(val.is_unspecified());
    }

    #[test]
    fn test_as_str() {
        let val = UnspecifiedValue::Unspecified;
        assert_eq!(val.as_str(), "Unspecified");
    }

    #[test]
    fn test_default() {
        let val: UnspecifiedValue = Default::default();
        assert_eq!(val, UnspecifiedValue::Unspecified);
    }

    #[test]
    fn test_display() {
        let val = UnspecifiedValue::Unspecified;
        assert_eq!(format!("{}", val), "Unspecified");
    }

    #[test]
    fn test_equality() {
        let val1 = UnspecifiedValue::Unspecified;
        let val2 = UnspecifiedValue::unspecified();
        assert_eq!(val1, val2);
    }
}
