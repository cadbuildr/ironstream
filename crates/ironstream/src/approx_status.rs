// FILE: approx_status.rs
// occt: Approx_Status

/// Status indicator for approximation computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApproxStatus {
    /// Points were added during approximation.
    PointsAdded = 0,
    /// No points were added.
    NoPointsAdded = 1,
    /// Approximation failed.
    NoApproximation = 2,
}

impl ApproxStatus {
    /// Converts an integer to status.
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(ApproxStatus::PointsAdded),
            1 => Some(ApproxStatus::NoPointsAdded),
            2 => Some(ApproxStatus::NoApproximation),
            _ => None,
        }
    }

    /// Converts status to integer.
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_values() {
        assert_eq!(ApproxStatus::PointsAdded.to_i32(), 0);
        assert_eq!(ApproxStatus::NoPointsAdded.to_i32(), 1);
        assert_eq!(ApproxStatus::NoApproximation.to_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            ApproxStatus::from_i32(0),
            Some(ApproxStatus::PointsAdded)
        );
        assert_eq!(
            ApproxStatus::from_i32(1),
            Some(ApproxStatus::NoPointsAdded)
        );
        assert_eq!(
            ApproxStatus::from_i32(2),
            Some(ApproxStatus::NoApproximation)
        );
        assert_eq!(ApproxStatus::from_i32(99), None);
    }

    #[test]
    fn test_round_trip() {
        for i in 0..3 {
            let s = ApproxStatus::from_i32(i).unwrap();
            assert_eq!(s.to_i32(), i);
        }
    }
}
