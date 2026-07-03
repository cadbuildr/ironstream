// FILE: gce_error_type.rs
// occt: gce_ErrorType

/// Status codes returned by geometric construction algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GceErrorType {
    /// Construction completed successfully.
    Done = 0,
    /// Two points are coincident.
    ConfusedPoints = 1,
    /// A radius value is negative.
    NegativeRadius = 2,
    /// Three points are collinear.
    ColinearPoints = 3,
    /// Intersection cannot be computed.
    IntersectionError = 4,
    /// Axis is undefined.
    NullAxis = 5,
    /// Angle value is invalid (usually null).
    NullAngle = 6,
    /// Radius is null.
    NullRadius = 7,
    /// Axis value is invalid.
    InvertAxis = 8,
    /// Angle value is invalid.
    BadAngle = 9,
    /// Radius values are inconsistent.
    InvertRadius = 10,
    /// Focal distance is null.
    NullFocusLength = 11,
    /// Vector is null.
    NullVector = 12,
    /// Coefficients of an equation are invalid.
    BadEquation = 13,
}

impl GceErrorType {
    /// Convert from i32 to enum.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(GceErrorType::Done),
            1 => Some(GceErrorType::ConfusedPoints),
            2 => Some(GceErrorType::NegativeRadius),
            3 => Some(GceErrorType::ColinearPoints),
            4 => Some(GceErrorType::IntersectionError),
            5 => Some(GceErrorType::NullAxis),
            6 => Some(GceErrorType::NullAngle),
            7 => Some(GceErrorType::NullRadius),
            8 => Some(GceErrorType::InvertAxis),
            9 => Some(GceErrorType::BadAngle),
            10 => Some(GceErrorType::InvertRadius),
            11 => Some(GceErrorType::NullFocusLength),
            12 => Some(GceErrorType::NullVector),
            13 => Some(GceErrorType::BadEquation),
            _ => None,
        }
    }

    /// Convert enum to i32.
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    /// Check if construction was successful.
    pub fn is_ok(self) -> bool {
        self == GceErrorType::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_type_values() {
        assert_eq!(GceErrorType::Done.to_i32(), 0);
        assert_eq!(GceErrorType::ConfusedPoints.to_i32(), 1);
        assert_eq!(GceErrorType::NegativeRadius.to_i32(), 2);
        assert_eq!(GceErrorType::BadEquation.to_i32(), 13);
    }

    #[test]
    fn test_error_type_from_i32() {
        assert_eq!(GceErrorType::from_i32(0), Some(GceErrorType::Done));
        assert_eq!(GceErrorType::from_i32(13), Some(GceErrorType::BadEquation));
        assert_eq!(GceErrorType::from_i32(14), None);
    }

    #[test]
    fn test_is_ok() {
        assert!(GceErrorType::Done.is_ok());
        assert!(!GceErrorType::NegativeRadius.is_ok());
    }

    #[test]
    fn test_debug_display() {
        let et = GceErrorType::NullAxis;
        let debug_str = format!("{:?}", et);
        assert!(debug_str.contains("NullAxis"));
    }
}
