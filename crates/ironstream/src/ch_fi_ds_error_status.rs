// FILE: ch_fi_ds_error_status.rs
// occt: ChFiDS_ErrorStatus

/// Status codes concerning the cause of errors in fillet operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFiDsErrorStatus {
    /// No error, operation succeeded
    Ok,
    /// Generic error
    Error,
    /// Walking algorithm failed
    WalkingFailure,
    /// Starting solution failed
    StartsolFailure,
    /// Twisted surface detected
    TwistedSurface,
}

impl ChFiDsErrorStatus {
    /// Returns true if this represents a successful status
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns true if this represents an error
    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}

impl Default for ChFiDsErrorStatus {
    fn default() -> Self {
        Self::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ok() {
        assert!(ChFiDsErrorStatus::Ok.is_ok());
        assert!(!ChFiDsErrorStatus::Error.is_ok());
        assert!(!ChFiDsErrorStatus::WalkingFailure.is_ok());
    }

    #[test]
    fn test_is_error() {
        assert!(ChFiDsErrorStatus::Error.is_error());
        assert!(ChFiDsErrorStatus::TwistedSurface.is_error());
        assert!(!ChFiDsErrorStatus::Ok.is_error());
    }

    #[test]
    fn test_default() {
        assert_eq!(ChFiDsErrorStatus::default(), ChFiDsErrorStatus::Ok);
    }
}
