// FILE: transfer_process_for_finder.rs
// occt: Transfer_ProcessForFinder

/// A process manager for finder-based transfer operations.
/// Orchestrates the transfer of entities discovered via a finder mechanism.
#[derive(Clone, Debug)]
pub struct TransferProcessForFinder {
    /// Process identifier
    process_id: u32,
    /// Number of entities processed
    nb_processed: u32,
    /// Whether the process is active
    is_active: bool,
}

impl TransferProcessForFinder {
    /// Creates a new finder-based transfer process.
    pub fn new() -> Self {
        Self {
            process_id: 0,
            nb_processed: 0,
            is_active: true,
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

    /// Resets the process.
    pub fn reset(&mut self) {
        self.nb_processed = 0;
    }
}

impl Default for TransferProcessForFinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let process = TransferProcessForFinder::new();
        assert!(process.is_active());
        assert_eq!(process.nb_processed(), 0);
    }

    #[test]
    fn test_increment_processed() {
        let mut process = TransferProcessForFinder::new();
        assert_eq!(process.nb_processed(), 0);

        process.increment_processed();
        assert_eq!(process.nb_processed(), 1);

        process.increment_processed();
        assert_eq!(process.nb_processed(), 2);
    }

    #[test]
    fn test_set_active() {
        let mut process = TransferProcessForFinder::new();
        assert!(process.is_active());

        process.set_active(false);
        assert!(!process.is_active());

        process.set_active(true);
        assert!(process.is_active());
    }

    #[test]
    fn test_reset() {
        let mut process = TransferProcessForFinder::new();
        process.increment_processed();
        process.increment_processed();
        assert_eq!(process.nb_processed(), 2);

        process.reset();
        assert_eq!(process.nb_processed(), 0);
    }
}
