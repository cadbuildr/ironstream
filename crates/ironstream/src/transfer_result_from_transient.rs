// FILE: transfer_result_from_transient.rs
// occt: Transfer_ResultFromTransient

/// Represents a transfer result derived from a transient entity.
/// Tracks the source transient and the resulting entity.
#[derive(Clone, Debug)]
pub struct TransferResultFromTransient {
    /// Source transient entity ID
    source_id: u32,
    /// Result entity ID
    result_id: u32,
    /// Whether the transfer succeeded
    success: bool,
}

impl TransferResultFromTransient {
    /// Creates a new transfer result from transient.
    pub fn new(source_id: u32, result_id: u32) -> Self {
        Self {
            source_id,
            result_id,
            success: true,
        }
    }

    /// Returns the source transient ID.
    pub fn source_id(&self) -> u32 {
        self.source_id
    }

    /// Returns the result entity ID.
    pub fn result_id(&self) -> u32 {
        self.result_id
    }

    /// Returns whether the transfer succeeded.
    pub fn success(&self) -> bool {
        self.success
    }

    /// Sets the success status.
    pub fn set_success(&mut self, success: bool) {
        self.success = success;
    }
}

impl Default for TransferResultFromTransient {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = TransferResultFromTransient::new(50, 150);
        assert_eq!(result.source_id(), 50);
        assert_eq!(result.result_id(), 150);
        assert!(result.success());
    }

    #[test]
    fn test_set_success() {
        let mut result = TransferResultFromTransient::new(10, 20);
        assert!(result.success());

        result.set_success(false);
        assert!(!result.success());
    }

    #[test]
    fn test_default() {
        let result = TransferResultFromTransient::default();
        assert_eq!(result.source_id(), 0);
        assert_eq!(result.result_id(), 0);
    }
}
