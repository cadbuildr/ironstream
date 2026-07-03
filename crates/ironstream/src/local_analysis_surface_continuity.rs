// FILE: local_analysis_surface_continuity.rs
// occt: LocalAnalysis_SurfaceContinuity

use crate::local_analysis_status_error_type::StatusErrorType;

/// Surface continuity analysis
#[derive(Clone, Debug)]
pub struct SurfaceContinuity {
    continuity_order: i32,
    status: StatusErrorType,
}

impl SurfaceContinuity {
    /// Create new surface continuity analysis
    pub fn new() -> Self {
        SurfaceContinuity {
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

impl Default for SurfaceContinuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sc = SurfaceContinuity::new();
        assert_eq!(sc.continuity_order(), 0);
        assert_eq!(sc.status(), StatusErrorType::NotDone);
    }

    #[test]
    fn test_setters() {
        let mut sc = SurfaceContinuity::new();
        sc.set_continuity_order(1);
        sc.set_status(StatusErrorType::OK);
        assert_eq!(sc.continuity_order(), 1);
        assert_eq!(sc.status(), StatusErrorType::OK);
    }
}
