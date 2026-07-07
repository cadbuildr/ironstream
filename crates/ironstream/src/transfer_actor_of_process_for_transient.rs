// FILE: transfer_actor_of_process_for_transient.rs
// occt: Transfer_ActorOfProcessForTransient

/// An actor that processes transient entities in a transfer process.
/// Handles the actual transfer logic for transient-based transfers.
#[derive(Clone, Debug)]
pub struct TransferActorOfProcessForTransient {
    /// Actor identifier
    actor_id: u32,
    /// Actor name
    name: String,
    /// Whether the actor is enabled
    enabled: bool,
}

impl TransferActorOfProcessForTransient {
    /// Creates a new actor for transient process.
    pub fn new() -> Self {
        Self {
            actor_id: 0,
            name: String::from("Transient"),
            enabled: true,
        }
    }

    /// Creates an actor with a name.
    pub fn with_name(name: &str) -> Self {
        Self {
            actor_id: 0,
            name: String::from(name),
            enabled: true,
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

    /// Returns whether the actor is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Sets the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for TransferActorOfProcessForTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let actor = TransferActorOfProcessForTransient::new();
        assert_eq!(actor.name(), "Transient");
        assert!(actor.is_enabled());
    }

    #[test]
    fn test_with_name() {
        let actor = TransferActorOfProcessForTransient::with_name("MyActor");
        assert_eq!(actor.name(), "MyActor");
        assert!(actor.is_enabled());
    }

    #[test]
    fn test_set_enabled() {
        let mut actor = TransferActorOfProcessForTransient::new();
        assert!(actor.is_enabled());

        actor.set_enabled(false);
        assert!(!actor.is_enabled());

        actor.set_enabled(true);
        assert!(actor.is_enabled());
    }
}
