// FILE: geom2d_undefined_value.rs
// occt: Geom2d_UndefinedValue

/// Exception raised when a geometric value is undefined.
#[derive(Clone, Debug)]
pub struct Geom2dUndefinedValue {
    message: String,
}

impl Geom2dUndefinedValue {
    pub fn new(msg: &str) -> Self {
        Geom2dUndefinedValue {
            message: msg.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Geom2dUndefinedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geom2d_UndefinedValue: {}", self.message)
    }
}

impl std::error::Error for Geom2dUndefinedValue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_value_new() {
        let exc = Geom2dUndefinedValue::new("Test message");
        assert_eq!(exc.message(), "Test message");
    }

    #[test]
    fn test_undefined_value_display() {
        let exc = Geom2dUndefinedValue::new("Vector has null magnitude");
        let msg = format!("{}", exc);
        assert!(msg.contains("Geom2d_UndefinedValue"));
        assert!(msg.contains("Vector has null magnitude"));
    }

    #[test]
    fn test_undefined_value_error_trait() {
        let exc = Geom2dUndefinedValue::new("Undefined value");
        let _: &dyn std::error::Error = &exc;
    }
}
