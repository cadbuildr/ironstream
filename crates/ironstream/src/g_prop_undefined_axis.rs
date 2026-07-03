// FILE: g_prop_undefined_axis.rs
// occt: GProp_UndefinedAxis

/// Exception raised when an inertia axis of symmetry is undefined.
/// This occurs when trying to access principal axes when the shape
/// has spherical symmetry or insufficient principal inertia values.
#[derive(Debug, Clone)]
pub struct UndefinedAxisError {
    message: String,
}

impl UndefinedAxisError {
    /// Creates a new UndefinedAxisError with the given message.
    pub fn new(message: &str) -> Self {
        UndefinedAxisError {
            message: message.to_string(),
        }
    }

    /// Gets the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for UndefinedAxisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GProp_UndefinedAxis: {}", self.message)
    }
}

impl std::error::Error for UndefinedAxisError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_axis_new() {
        let error = UndefinedAxisError::new("Principal axis undefined");
        assert_eq!(error.message(), "Principal axis undefined");
    }

    #[test]
    fn test_undefined_axis_display() {
        let error = UndefinedAxisError::new("Spherical symmetry");
        let displayed = format!("{}", error);
        assert!(displayed.contains("Spherical symmetry"));
    }

    #[test]
    fn test_undefined_axis_clone() {
        let error1 = UndefinedAxisError::new("Test message");
        let error2 = error1.clone();
        assert_eq!(error1.message(), error2.message());
    }
}
