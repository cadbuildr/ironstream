// FILE: geom_lib_interpolation_errors.rs
// occt: GeomLib_InterpolationErrors

/// Errors in interpolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomLibInterpolationErrors {
    /// No error.
    NoError = 0,
    /// Invalid number of points.
    InvalidNumberOfPoints = 1,
    /// Points are collinear.
    PointsAreCollinear = 2,
    /// Interpolation failed.
    InterpolationFailed = 3,
}

impl GeomLibInterpolationErrors {
    /// Convert from i32.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(GeomLibInterpolationErrors::NoError),
            1 => Some(GeomLibInterpolationErrors::InvalidNumberOfPoints),
            2 => Some(GeomLibInterpolationErrors::PointsAreCollinear),
            3 => Some(GeomLibInterpolationErrors::InterpolationFailed),
            _ => None,
        }
    }

    /// Convert to i32.
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_values() {
        assert_eq!(GeomLibInterpolationErrors::NoError.to_i32(), 0);
        assert_eq!(GeomLibInterpolationErrors::PointsAreCollinear.to_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            GeomLibInterpolationErrors::from_i32(0),
            Some(GeomLibInterpolationErrors::NoError)
        );
        assert_eq!(GeomLibInterpolationErrors::from_i32(4), None);
    }
}
