// FILE: transfer_status_result.rs
// occt: Transfer_StatusResult

/// Enumeration representing the status of a transfer result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferStatusResult {
    /// Transfer succeeded
    Success = 0,
    /// Transfer produced a result with warnings
    SuccessWithWarning = 1,
    /// Transfer failed
    Failure = 2,
    /// Transfer failed with warnings
    FailureWithWarning = 3,
    /// Transfer was skipped
    Skipped = 4,
    /// Transfer status unknown
    Unknown = 5,
}

impl Default for TransferStatusResult {
    fn default() -> Self {
        Self::Unknown
    }
}

impl TransferStatusResult {
    /// Returns the name of the status.
    pub fn name(self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::SuccessWithWarning => "SuccessWithWarning",
            Self::Failure => "Failure",
            Self::FailureWithWarning => "FailureWithWarning",
            Self::Skipped => "Skipped",
            Self::Unknown => "Unknown",
        }
    }

    /// Returns whether the status represents a successful transfer.
    pub fn is_success(self) -> bool {
        matches!(self, Self::Success | Self::SuccessWithWarning)
    }

    /// Returns whether the status represents a failure.
    pub fn is_failure(self) -> bool {
        matches!(self, Self::Failure | Self::FailureWithWarning)
    }

    /// Returns whether the status indicates warnings.
    pub fn has_warning(self) -> bool {
        matches!(
            self,
            Self::SuccessWithWarning | Self::FailureWithWarning
        )
    }

    /// Parses a string to a status result.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Success" => Some(Self::Success),
            "SuccessWithWarning" => Some(Self::SuccessWithWarning),
            "Failure" => Some(Self::Failure),
            "FailureWithWarning" => Some(Self::FailureWithWarning),
            "Skipped" => Some(Self::Skipped),
            "Unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(TransferStatusResult::default(), TransferStatusResult::Unknown);
    }

    #[test]
    fn test_names() {
        assert_eq!(TransferStatusResult::Success.name(), "Success");
        assert_eq!(TransferStatusResult::Failure.name(), "Failure");
        assert_eq!(TransferStatusResult::Skipped.name(), "Skipped");
    }

    #[test]
    fn test_is_success() {
        assert!(TransferStatusResult::Success.is_success());
        assert!(TransferStatusResult::SuccessWithWarning.is_success());
        assert!(!TransferStatusResult::Failure.is_success());
    }

    #[test]
    fn test_is_failure() {
        assert!(TransferStatusResult::Failure.is_failure());
        assert!(TransferStatusResult::FailureWithWarning.is_failure());
        assert!(!TransferStatusResult::Success.is_failure());
    }

    #[test]
    fn test_has_warning() {
        assert!(TransferStatusResult::SuccessWithWarning.has_warning());
        assert!(TransferStatusResult::FailureWithWarning.has_warning());
        assert!(!TransferStatusResult::Success.has_warning());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            TransferStatusResult::from_str("Success"),
            Some(TransferStatusResult::Success)
        );
        assert_eq!(
            TransferStatusResult::from_str("Failure"),
            Some(TransferStatusResult::Failure)
        );
        assert_eq!(TransferStatusResult::from_str("Invalid"), None);
    }
}
