// FILE: cs_lib_derivative_status.rs

// occt: CSLib_DerivativeStatus
//
// Status enumeration for surface derivatives computation.
// Describes the result of attempting to compute a surface normal from the first
// derivatives D1U and D1V at a point on a surface.

/// Status of surface derivatives computation for normal calculation.
///
/// Describes the result of attempting to compute a surface normal
/// from the first derivatives D1U and D1V at a point on a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CslibDerivativeStatus {
    /// Normal computed successfully.
    Done = 0,
    /// D1U has null length: ||D1U|| <= Resolution.
    D1uIsNull = 1,
    /// D1V has null length: ||D1V|| <= Resolution.
    D1vIsNull = 2,
    /// Both derivatives are null.
    D1IsNull = 3,
    /// D1U is negligible: ||D1U|| / ||D1V|| <= RealEpsilon.
    D1uD1vRatioIsNull = 4,
    /// D1V is negligible: ||D1V|| / ||D1U|| <= RealEpsilon.
    D1vD1uRatioIsNull = 5,
    /// D1U and D1V are parallel.
    D1uIsParallelD1v = 6,
}

impl CslibDerivativeStatus {
    /// Returns the numeric value of this status.
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    /// Creates a status from a numeric value.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(CslibDerivativeStatus::Done),
            1 => Some(CslibDerivativeStatus::D1uIsNull),
            2 => Some(CslibDerivativeStatus::D1vIsNull),
            3 => Some(CslibDerivativeStatus::D1IsNull),
            4 => Some(CslibDerivativeStatus::D1uD1vRatioIsNull),
            5 => Some(CslibDerivativeStatus::D1vD1uRatioIsNull),
            6 => Some(CslibDerivativeStatus::D1uIsParallelD1v),
            _ => None,
        }
    }

    /// Returns true if the status indicates successful computation.
    pub fn is_done(&self) -> bool {
        *self == CslibDerivativeStatus::Done
    }

    /// Returns true if the status indicates a derivative error (null or degenerate).
    pub fn is_error(&self) -> bool {
        !self.is_done()
    }
}

impl Default for CslibDerivativeStatus {
    fn default() -> Self {
        CslibDerivativeStatus::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivative_status_done() {
        let status = CslibDerivativeStatus::Done;
        assert_eq!(status.as_i32(), 0);
        assert!(status.is_done());
        assert!(!status.is_error());
    }

    #[test]
    fn test_derivative_status_d1u_is_null() {
        let status = CslibDerivativeStatus::D1uIsNull;
        assert_eq!(status.as_i32(), 1);
        assert!(!status.is_done());
        assert!(status.is_error());
    }

    #[test]
    fn test_derivative_status_d1v_is_null() {
        let status = CslibDerivativeStatus::D1vIsNull;
        assert_eq!(status.as_i32(), 2);
        assert!(!status.is_done());
        assert!(status.is_error());
    }

    #[test]
    fn test_derivative_status_d1_is_null() {
        let status = CslibDerivativeStatus::D1IsNull;
        assert_eq!(status.as_i32(), 3);
        assert!(!status.is_done());
        assert!(status.is_error());
    }

    #[test]
    fn test_derivative_status_d1u_d1v_ratio_is_null() {
        let status = CslibDerivativeStatus::D1uD1vRatioIsNull;
        assert_eq!(status.as_i32(), 4);
        assert!(!status.is_done());
        assert!(status.is_error());
    }

    #[test]
    fn test_derivative_status_d1v_d1u_ratio_is_null() {
        let status = CslibDerivativeStatus::D1vD1uRatioIsNull;
        assert_eq!(status.as_i32(), 5);
        assert!(!status.is_done());
        assert!(status.is_error());
    }

    #[test]
    fn test_derivative_status_d1u_is_parallel_d1v() {
        let status = CslibDerivativeStatus::D1uIsParallelD1v;
        assert_eq!(status.as_i32(), 6);
        assert!(!status.is_done());
        assert!(status.is_error());
    }

    #[test]
    fn test_from_i32_valid() {
        assert_eq!(
            CslibDerivativeStatus::from_i32(0),
            Some(CslibDerivativeStatus::Done)
        );
        assert_eq!(
            CslibDerivativeStatus::from_i32(1),
            Some(CslibDerivativeStatus::D1uIsNull)
        );
        assert_eq!(
            CslibDerivativeStatus::from_i32(6),
            Some(CslibDerivativeStatus::D1uIsParallelD1v)
        );
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(CslibDerivativeStatus::from_i32(-1), None);
        assert_eq!(CslibDerivativeStatus::from_i32(7), None);
        assert_eq!(CslibDerivativeStatus::from_i32(100), None);
    }

    #[test]
    fn test_default_status() {
        let status = CslibDerivativeStatus::default();
        assert_eq!(status, CslibDerivativeStatus::Done);
    }

    #[test]
    fn test_status_equality() {
        let status1 = CslibDerivativeStatus::D1uIsNull;
        let status2 = CslibDerivativeStatus::D1uIsNull;
        let status3 = CslibDerivativeStatus::D1vIsNull;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_status_in_match() {
        let status = CslibDerivativeStatus::D1IsNull;
        let result = match status {
            CslibDerivativeStatus::Done => "done",
            CslibDerivativeStatus::D1IsNull => "both_null",
            _ => "other",
        };
        assert_eq!(result, "both_null");
    }
}
