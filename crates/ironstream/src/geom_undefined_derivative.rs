// FILE: geom_undefined_derivative.rs
// occt: Geom_UndefinedDerivative

//! Exception for when a derivative is undefined at a point.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndefinedDerivative {
    pub message: String,
}

impl UndefinedDerivative {
    pub fn new(message: &str) -> Self {
        UndefinedDerivative {
            message: message.to_string(),
        }
    }

    pub fn new_default() -> Self {
        UndefinedDerivative {
            message: "Undefined derivative".to_string(),
        }
    }
}

impl std::fmt::Display for UndefinedDerivative {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UndefinedDerivative {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let err = UndefinedDerivative::new("Test error");
        assert_eq!(err.message, "Test error");
    }

    #[test]
    fn test_default() {
        let err = UndefinedDerivative::new_default();
        assert_eq!(err.message, "Undefined derivative");
    }

    #[test]
    fn test_display() {
        let err = UndefinedDerivative::new("Custom message");
        assert_eq!(err.to_string(), "Custom message");
    }

    #[test]
    fn test_equality() {
        let err1 = UndefinedDerivative::new("Same");
        let err2 = UndefinedDerivative::new("Same");
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_inequality() {
        let err1 = UndefinedDerivative::new("Different1");
        let err2 = UndefinedDerivative::new("Different2");
        assert_ne!(err1, err2);
    }
}
