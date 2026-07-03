// FILE: geom2d_undefined_derivative.rs
// occt: Geom2d_UndefinedDerivative

/// Exception raised when a derivative is undefined.
#[derive(Clone, Debug)]
pub struct Geom2dUndefinedDerivative {
    message: String,
}

impl Geom2dUndefinedDerivative {
    pub fn new(msg: &str) -> Self {
        Geom2dUndefinedDerivative {
            message: msg.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for Geom2dUndefinedDerivative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geom2d_UndefinedDerivative: {}", self.message)
    }
}

impl std::error::Error for Geom2dUndefinedDerivative {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_derivative_new() {
        let exc = Geom2dUndefinedDerivative::new("Test message");
        assert_eq!(exc.message(), "Test message");
    }

    #[test]
    fn test_undefined_derivative_display() {
        let exc = Geom2dUndefinedDerivative::new("Point has zero curvature");
        let msg = format!("{}", exc);
        assert!(msg.contains("Geom2d_UndefinedDerivative"));
        assert!(msg.contains("Point has zero curvature"));
    }

    #[test]
    fn test_undefined_derivative_error_trait() {
        let exc = Geom2dUndefinedDerivative::new("Error message");
        let _: &dyn std::error::Error = &exc;
    }
}
