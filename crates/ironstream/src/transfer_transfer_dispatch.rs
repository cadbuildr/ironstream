// FILE: transfer_transfer_dispatch.rs
// occt: Transfer_TransferDispatch

/// Manages dispatch of transfer operations across multiple actors.
/// Coordinates the routing of entities to appropriate transfer handlers.
#[derive(Clone, Debug)]
pub struct TransferTransferDispatch {
    /// Dispatch identifier
    dispatch_id: u32,
    /// Number of entities dispatched
    dispatched_count: u32,
    /// Whether the dispatcher is active
    is_active: bool,
}

impl TransferTransferDispatch {
    /// Creates a new transfer dispatcher.
    pub fn new() -> Self {
        Self {
            dispatch_id: 0,
            dispatched_count: 0,
            is_active: true,
        }
    }

    /// Returns the dispatcher ID.
    pub fn id(&self) -> u32 {
        self.dispatch_id
    }

    /// Returns the count of dispatched entities.
    pub fn dispatched_count(&self) -> u32 {
        self.dispatched_count
    }

    /// Increments the dispatched count.
    pub fn increment_count(&mut self) {
        self.dispatched_count = self.dispatched_count.saturating_add(1);
    }

    /// Returns whether the dispatcher is active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Sets the active state.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Resets the dispatcher.
    pub fn reset(&mut self) {
        self.dispatched_count = 0;
    }
}

impl Default for TransferTransferDispatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dispatcher = TransferTransferDispatch::new();
        assert!(dispatcher.is_active());
        assert_eq!(dispatcher.dispatched_count(), 0);
    }

    #[test]
    fn test_increment_count() {
        let mut dispatcher = TransferTransferDispatch::new();
        dispatcher.increment_count();
        assert_eq!(dispatcher.dispatched_count(), 1);

        dispatcher.increment_count();
        dispatcher.increment_count();
        assert_eq!(dispatcher.dispatched_count(), 3);
    }

    #[test]
    fn test_set_active() {
        let mut dispatcher = TransferTransferDispatch::new();
        assert!(dispatcher.is_active());

        dispatcher.set_active(false);
        assert!(!dispatcher.is_active());
    }

    #[test]
    fn test_reset() {
        let mut dispatcher = TransferTransferDispatch::new();
        dispatcher.increment_count();
        dispatcher.increment_count();
        assert_eq!(dispatcher.dispatched_count(), 2);

        dispatcher.reset();
        assert_eq!(dispatcher.dispatched_count(), 0);
    }
}
