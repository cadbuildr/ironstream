// FILE: gce_root.rs
// occt: gce_Root

//! Error type enumeration for geometric construction operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GceErrorType {
    Done,
    ConfusedPoints,
    NegativeRadius,
    ColinearPoints,
    IntersectionError,
    NullAxis,
    NullAngle,
    NullRadius,
    InvertAxis,
    BadAngle,
    InvertRadius,
    NullFocusLength,
    NullVector,
    BadEquation,
}

/// Base class for geometric construction algorithms.
/// Provides common status services for all gce construction classes.
pub struct GceRoot {
    pub(crate) error: GceErrorType,
}

impl GceRoot {
    /// Creates a new GceRoot with Done status.
    pub fn new() -> Self {
        GceRoot {
            error: GceErrorType::Done,
        }
    }

    /// Creates a new GceRoot with the given error status.
    pub fn with_error(error: GceErrorType) -> Self {
        GceRoot { error }
    }

    /// Returns true if the construction is successful.
    pub fn is_done(&self) -> bool {
        self.error == GceErrorType::Done
    }

    /// Returns true if the construction has failed.
    pub fn is_error(&self) -> bool {
        self.error != GceErrorType::Done
    }

    /// Returns the status of the construction.
    pub fn status(&self) -> GceErrorType {
        self.error
    }
}

impl Default for GceRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_done_status() {
        let root = GceRoot::new();
        assert!(root.is_done());
        assert!(!root.is_error());
        assert_eq!(root.status(), GceErrorType::Done);
    }

    #[test]
    fn test_error_status() {
        let root = GceRoot::with_error(GceErrorType::NullFocusLength);
        assert!(!root.is_done());
        assert!(root.is_error());
        assert_eq!(root.status(), GceErrorType::NullFocusLength);
    }

    #[test]
    fn test_default() {
        let root = GceRoot::default();
        assert!(root.is_done());
    }
}
