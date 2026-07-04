// FILE: transfer_actor_of_finder_process.rs
// occt: Transfer_ActorOfFinderProcess

/// An actor for processing finder-based transfer operations.
/// Used in transfers where entities are discovered via a finder mechanism.
#[derive(Clone, Debug)]
pub struct TransferActorOfFinderProcess {
    /// Unique identifier for this actor
    actor_id: u32,
}

impl TransferActorOfFinderProcess {
    /// Creates a new actor for finder-based processes.
    pub fn new() -> Self {
        Self { actor_id: 0 }
    }

    /// Creates an actor with a specific ID.
    pub fn with_id(actor_id: u32) -> Self {
        Self { actor_id }
    }

    /// Returns the actor's ID.
    pub fn id(&self) -> u32 {
        self.actor_id
    }

    /// Sets the actor's ID.
    pub fn set_id(&mut self, actor_id: u32) {
        self.actor_id = actor_id;
    }
}

impl Default for TransferActorOfFinderProcess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let actor = TransferActorOfFinderProcess::new();
        assert_eq!(actor.id(), 0);
    }

    #[test]
    fn test_with_id() {
        let actor = TransferActorOfFinderProcess::with_id(42);
        assert_eq!(actor.id(), 42);
    }

    #[test]
    fn test_set_id() {
        let mut actor = TransferActorOfFinderProcess::new();
        actor.set_id(123);
        assert_eq!(actor.id(), 123);
    }

    #[test]
    fn test_default() {
        let actor = TransferActorOfFinderProcess::default();
        assert_eq!(actor.id(), 0);
    }
}
