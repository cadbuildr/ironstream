// FILE: t_doc_std_application_delta.rs
// occt: TDocStd_ApplicationDelta

/// Represents a delta change at the application level.
/// Used for managing undo/redo of application-level modifications.
#[derive(Clone, Debug)]
pub struct TDocStd_ApplicationDelta {
    is_undoing: bool,
    is_redoing: bool,
}

impl TDocStd_ApplicationDelta {
    /// Create a new application delta.
    pub fn new() -> Self {
        Self {
            is_undoing: false,
            is_redoing: false,
        }
    }

    /// Set whether this is an undo operation.
    pub fn set_undoing(&mut self, undoing: bool) {
        self.is_undoing = undoing;
    }

    /// Check if this is an undo operation.
    pub fn is_undoing(&self) -> bool {
        self.is_undoing
    }

    /// Set whether this is a redo operation.
    pub fn set_redoing(&mut self, redoing: bool) {
        self.is_redoing = redoing;
    }

    /// Check if this is a redo operation.
    pub fn is_redoing(&self) -> bool {
        self.is_redoing
    }
}

impl Default for TDocStd_ApplicationDelta {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_delta() {
        let delta = TDocStd_ApplicationDelta::new();
        assert!(!delta.is_undoing());
        assert!(!delta.is_redoing());
    }

    #[test]
    fn test_set_undoing() {
        let mut delta = TDocStd_ApplicationDelta::new();
        delta.set_undoing(true);
        assert!(delta.is_undoing());
    }

    #[test]
    fn test_set_redoing() {
        let mut delta = TDocStd_ApplicationDelta::new();
        delta.set_redoing(true);
        assert!(delta.is_redoing());
    }

    #[test]
    fn test_default() {
        let delta = TDocStd_ApplicationDelta::default();
        assert!(!delta.is_undoing());
    }
}
