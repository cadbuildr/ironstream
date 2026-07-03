// FILE: b_rep_lib_edge_error.rs
// occt: BRepLib_EdgeError

/// Errors that can occur at edge construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeError {
    /// No error
    EdgeDone = 0,
    /// Point projection failed
    PointProjectionFailed = 1,
    /// Parameter out of range
    ParameterOutOfRange = 2,
    /// Different points on closed curve
    DifferentPointsOnClosedCurve = 3,
    /// Point with infinite parameter
    PointWithInfiniteParameter = 4,
    /// Different point and parameter
    DifferentsPointAndParameter = 5,
    /// Line through identical points
    LineThroughIdenticPoints = 6,
}

impl EdgeError {
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(EdgeError::EdgeDone),
            1 => Some(EdgeError::PointProjectionFailed),
            2 => Some(EdgeError::ParameterOutOfRange),
            3 => Some(EdgeError::DifferentPointsOnClosedCurve),
            4 => Some(EdgeError::PointWithInfiniteParameter),
            5 => Some(EdgeError::DifferentsPointAndParameter),
            6 => Some(EdgeError::LineThroughIdenticPoints),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_error_values() {
        assert_eq!(EdgeError::EdgeDone.as_i32(), 0);
        assert_eq!(EdgeError::PointProjectionFailed.as_i32(), 1);
        assert_eq!(EdgeError::ParameterOutOfRange.as_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(EdgeError::from_i32(0), Some(EdgeError::EdgeDone));
        assert_eq!(EdgeError::from_i32(1), Some(EdgeError::PointProjectionFailed));
        assert_eq!(EdgeError::from_i32(7), None);
    }
}
