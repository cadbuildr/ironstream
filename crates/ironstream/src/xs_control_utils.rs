// FILE: xs_control_utils.rs
// occt: XSControl_Utils

/// Utility functions for the control framework.
/// Provides static helper methods for common operations.
pub struct XSControlUtils;

impl XSControlUtils {
    /// Validates a filename.
    pub fn is_valid_filename(filename: &str) -> bool {
        !filename.is_empty() && !filename.contains('\0')
    }

    /// Converts a status code to a message.
    pub fn status_message(status: i32) -> &'static str {
        match status {
            0 => "Success",
            -1 => "File not found",
            -2 => "Invalid format",
            1 => "Warning",
            2 => "Error",
            _ => "Unknown",
        }
    }

    /// Clamps a value between min and max.
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Returns the maximum of two values.
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }

    /// Returns the minimum of two values.
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b { a } else { b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_filename() {
        assert!(XSControlUtils::is_valid_filename("file.stp"));
        assert!(XSControlUtils::is_valid_filename("test"));
        assert!(!XSControlUtils::is_valid_filename(""));
    }

    #[test]
    fn test_status_message() {
        assert_eq!(XSControlUtils::status_message(0), "Success");
        assert_eq!(XSControlUtils::status_message(-1), "File not found");
        assert_eq!(XSControlUtils::status_message(99), "Unknown");
    }

    #[test]
    fn test_clamp() {
        assert_eq!(XSControlUtils::clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(XSControlUtils::clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(XSControlUtils::clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_max() {
        assert_eq!(XSControlUtils::max(5.0, 3.0), 5.0);
        assert_eq!(XSControlUtils::max(3.0, 5.0), 5.0);
        assert_eq!(XSControlUtils::max(5.0, 5.0), 5.0);
    }

    #[test]
    fn test_min() {
        assert_eq!(XSControlUtils::min(5.0, 3.0), 3.0);
        assert_eq!(XSControlUtils::min(3.0, 5.0), 3.0);
        assert_eq!(XSControlUtils::min(5.0, 5.0), 5.0);
    }
}
