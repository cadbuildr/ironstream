// FILE: b_rep_feat_status.rs
// occt: BRepFeat_Status

/// Enumeration of status codes for feature operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BRepFeatStatus {
    /// No error
    NoError = 0,
    /// Invalid placement
    InvalidPlacement = 1,
    /// Hole too long
    HoleTooLong = 2,
}

impl BRepFeatStatus {
    /// Converts an integer to a status.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(BRepFeatStatus::NoError),
            1 => Some(BRepFeatStatus::InvalidPlacement),
            2 => Some(BRepFeatStatus::HoleTooLong),
            _ => None,
        }
    }

    /// Converts a status to an integer.
    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_enum() {
        assert_eq!(BRepFeatStatus::NoError.to_int(), 0);
        assert_eq!(BRepFeatStatus::InvalidPlacement.to_int(), 1);
        assert_eq!(BRepFeatStatus::HoleTooLong.to_int(), 2);
    }

    #[test]
    fn test_status_from_int() {
        assert_eq!(
            BRepFeatStatus::from_int(0),
            Some(BRepFeatStatus::NoError)
        );
        assert_eq!(
            BRepFeatStatus::from_int(3),
            None
        );
    }
}
