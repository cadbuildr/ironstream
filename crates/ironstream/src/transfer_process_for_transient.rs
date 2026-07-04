// FILE: transfer_process_for_transient.rs
// occt: Transfer_ProcessForTransient

/// A process manager for transient-based transfer operations.
/// Orchestrates the transfer of transient (non-persistent) entities.
#[derive(Clone, Debug)]
pub struct TransferProcessForTransient {
    /// Process identifier
    process_id: u32,
    /// Number of entities processed
    nb_processed: u32,
    /// Whether the process is active
    is_active: bool,
    /// Whether the process completed successfully
    success: bool,
}

impl TransferProcessForTransient {
    /// Creates a new transient-based transfer process.
    pub fn new() -> Self {
        Self {
            process_id: 0,
            nb_processed: 0,
            is_active: true,
            success: false,
        }
    }

    /// Returns the process ID.
    pub fn id(&self) -> u32 {
        self.process_id
    }

    /// Returns the number of processed entities.
    pub fn nb_processed(&self) -> u32 {
        self.nb_processed
    }

    /// Increments the processed count.
    pub fn increment_processed(&mut self) {
        self.nb_processed = self.nb_processed.saturating_add(1);
    }

    /// Returns whether the process is active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Sets the active state.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Returns whether the process succeeded.
    pub fn success(&self) -> bool {
        self.success
    }

    /// Sets the success status.
    pub fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    /// Resets the process.
    pub fn reset(&mut self) {
        self.nb_processed = 0;
        self.success = false;
    }
}

impl Default for TransferProcessForTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let process = TransferProcessForTransient::new();
        assert!(process.is_active());
        assert_eq!(process.nb_processed(), 0);
        assert!(!process.success());
    }

    #[test]
    fn test_increment_processed() {
        let mut process = TransferProcessForTransient::new();
        process.increment_processed();
        process.increment_processed();
        assert_eq!(process.nb_processed(), 2);
    }

    #[test]
    fn test_set_success() {
        let mut process = TransferProcessForTransient::new();
        assert!(!process.success());

        process.set_success(true);
        assert!(process.success());

        process.set_success(false);
        assert!(!process.success());
    }

    #[test]
    fn test_reset() {
        let mut process = TransferProcessForTransient::new();
        process.increment_processed();
        process.increment_processed();
        process.set_success(true);

        process.reset();
        assert_eq!(process.nb_processed(), 0);
        assert!(!process.success());
    }
}
