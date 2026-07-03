// FILE: gp_o.rs
// occt: gp

//! Module namespace for geometric primitives (gp).
//! Contains basic geometric types: points, vectors, transforms, etc.

pub const VERSION: &str = "1.0.0";

/// Constants for geometric operations.
pub const EPSILON: f64 = 1e-15;
pub const ZERO_TOL: f64 = 1e-15;

/// Result type for geometric operations.
pub type GpResult<T> = Result<T, String>;

/// Generic error for geometric computation.
pub fn make_error(msg: &str) -> String {
    msg.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_epsilon() {
        assert!(EPSILON > 0.0);
        assert!(EPSILON < 0.001);
    }

    #[test]
    fn test_make_error() {
        let err = make_error("test error");
        assert_eq!(err, "test error");
    }
}
