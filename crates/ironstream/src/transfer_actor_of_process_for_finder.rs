// FILE: transfer_actor_of_process_for_finder.rs
// occt: Transfer_ActorOfProcessForFinder

/// An actor that processes entities in a finder-based process.
/// Performs actual transfer logic for finder-based transfer processes.
#[derive(Clone, Debug)]
pub struct TransferActorOfProcessForFinder {
    /// Actor identifier
    actor_id: u32,
    /// Name of the actor
    name: String,
}

impl TransferActorOfProcessForFinder {
    /// Creates a new actor for finder process.
    pub fn new() -> Self {
        Self {
            actor_id: 0,
            name: String::from("Finder"),
        }
    }

    /// Creates an actor with a name.
    pub fn with_name(name: &str) -> Self {
        Self {
            actor_id: 0,
            name: String::from(name),
        }
    }

    /// Returns the actor ID.
    pub fn id(&self) -> u32 {
        self.actor_id
    }

    /// Returns the actor's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the actor's name.
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

impl Default for TransferActorOfProcessForFinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let actor = TransferActorOfProcessForFinder::new();
        assert_eq!(actor.name(), "Finder");
    }

    #[test]
    fn test_with_name() {
        let actor = TransferActorOfProcessForFinder::with_name("CustomFinder");
        assert_eq!(actor.name(), "CustomFinder");
    }

    #[test]
    fn test_set_name() {
        let mut actor = TransferActorOfProcessForFinder::new();
        actor.set_name("NewName");
        assert_eq!(actor.name(), "NewName");
    }
}
