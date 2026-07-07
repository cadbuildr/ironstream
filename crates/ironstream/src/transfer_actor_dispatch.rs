// FILE: transfer_actor_dispatch.rs
// occt: Transfer_ActorDispatch

/// An actor that dispatches transfer operations across multiple sub-actors.
/// It acts as a default actor while allowing specialized actors to be added for specific transfer types.
#[derive(Clone, Debug)]
pub struct TransferActorDispatch {
    /// Identifier for the dispatch actor
    actor_id: u32,
    /// Sub-actors registered with this dispatcher
    sub_actors: Vec<u32>,
    /// Whether this is active
    is_active: bool,
}

impl TransferActorDispatch {
    /// Creates a new transfer actor dispatcher.
    pub fn new() -> Self {
        Self {
            actor_id: 1,
            sub_actors: Vec::new(),
            is_active: true,
        }
    }

    /// Adds a sub-actor to the dispatcher.
    pub fn add_actor(&mut self, actor_id: u32) {
        if !self.sub_actors.contains(&actor_id) {
            self.sub_actors.push(actor_id);
        }
    }

    /// Removes a sub-actor from the dispatcher.
    pub fn remove_actor(&mut self, actor_id: u32) {
        self.sub_actors.retain(|&id| id != actor_id);
    }

    /// Returns the number of registered sub-actors.
    pub fn nb_actors(&self) -> usize {
        self.sub_actors.len()
    }

    /// Returns whether the dispatcher is active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Sets the active state of the dispatcher.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Returns the actor ID.
    pub fn actor_id(&self) -> u32 {
        self.actor_id
    }
}

impl Default for TransferActorDispatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dispatcher = TransferActorDispatch::new();
        assert!(dispatcher.is_active());
        assert_eq!(dispatcher.nb_actors(), 0);
        assert_eq!(dispatcher.actor_id(), 1);
    }

    #[test]
    fn test_add_actor() {
        let mut dispatcher = TransferActorDispatch::new();
        dispatcher.add_actor(10);
        assert_eq!(dispatcher.nb_actors(), 1);

        dispatcher.add_actor(20);
        assert_eq!(dispatcher.nb_actors(), 2);

        // Adding duplicate should not increase count
        dispatcher.add_actor(10);
        assert_eq!(dispatcher.nb_actors(), 2);
    }

    #[test]
    fn test_remove_actor() {
        let mut dispatcher = TransferActorDispatch::new();
        dispatcher.add_actor(10);
        dispatcher.add_actor(20);
        assert_eq!(dispatcher.nb_actors(), 2);

        dispatcher.remove_actor(10);
        assert_eq!(dispatcher.nb_actors(), 1);

        dispatcher.remove_actor(20);
        assert_eq!(dispatcher.nb_actors(), 0);
    }

    #[test]
    fn test_set_active() {
        let mut dispatcher = TransferActorDispatch::new();
        assert!(dispatcher.is_active());

        dispatcher.set_active(false);
        assert!(!dispatcher.is_active());

        dispatcher.set_active(true);
        assert!(dispatcher.is_active());
    }
}
