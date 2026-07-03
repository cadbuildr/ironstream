// FILE: gp_vector_with_null_magnitude.rs
// occt: gp_VectorWithNullMagnitude

/// Exception for vector with null/zero magnitude.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VectorWithNullMagnitude;

impl std::fmt::Display for VectorWithNullMagnitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector with null magnitude")
    }
}

impl std::error::Error for VectorWithNullMagnitude {}

/// Check if a vector has null magnitude (is zero vector).
pub fn has_null_magnitude(x: f64, y: f64, z: f64) -> bool {
    let mag_sq = x * x + y * y + z * z;
    mag_sq < 1e-30  // Square of 1e-15
}

/// Check if a 2D vector has null magnitude.
pub fn has_null_magnitude_2d(x: f64, y: f64) -> bool {
    let mag_sq = x * x + y * y;
    mag_sq < 1e-30
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_magnitude_display() {
        let err = VectorWithNullMagnitude;
        let msg = format!("{}", err);
        assert_eq!(msg, "Vector with null magnitude");
    }

    #[test]
    fn test_has_null_magnitude_zero() {
        assert!(has_null_magnitude(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_has_null_magnitude_small() {
        assert!(has_null_magnitude(1e-16, 1e-16, 1e-16));
    }

    #[test]
    fn test_has_null_magnitude_non_zero() {
        assert!(!has_null_magnitude(1.0, 0.0, 0.0));
        assert!(!has_null_magnitude(0.0, 1.0, 0.0));
        assert!(!has_null_magnitude(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_has_null_magnitude_2d() {
        assert!(has_null_magnitude_2d(0.0, 0.0));
        assert!(!has_null_magnitude_2d(1.0, 0.0));
    }
}
