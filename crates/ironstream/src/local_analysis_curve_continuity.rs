// FILE: local_analysis_curve_continuity.rs
// occt: LocalAnalysis_CurveContinuity

use crate::local_analysis_status_error_type::StatusErrorType;

/// Curve continuity analysis
#[derive(Clone, Debug)]
pub struct CurveContinuity {
    continuity_order: i32,
    status: StatusErrorType,
}

impl CurveContinuity {
    /// Create new curve continuity analysis
    pub fn new() -> Self {
        CurveContinuity {
            continuity_order: 0,
            status: StatusErrorType::NotDone,
        }
    }

    /// Get continuity order (0=C0, 1=C1, 2=C2, etc.)
    pub fn continuity_order(&self) -> i32 {
        self.continuity_order
    }

    /// Set continuity order
    pub fn set_continuity_order(&mut self, order: i32) {
        self.continuity_order = order;
    }

    /// Get status
    pub fn status(&self) -> StatusErrorType {
        self.status
    }

    /// Set status
    pub fn set_status(&mut self, status: StatusErrorType) {
        self.status = status;
    }
}

impl Default for CurveContinuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cc = CurveContinuity::new();
        assert_eq!(cc.continuity_order(), 0);
        assert_eq!(cc.status(), StatusErrorType::NotDone);
    }

    #[test]
    fn test_setters() {
        let mut cc = CurveContinuity::new();
        cc.set_continuity_order(2);
        cc.set_status(StatusErrorType::OK);
        assert_eq!(cc.continuity_order(), 2);
        assert_eq!(cc.status(), StatusErrorType::OK);
    }
}
