// FILE: step_control_actor_read.rs
// occt: STEPControl_ActorRead

/// Actor for reading STEP files
pub struct STEPControl_ActorRead {
    enabled: bool,
}

impl STEPControl_ActorRead {
    pub fn new() -> Self {
        STEPControl_ActorRead { enabled: true }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for STEPControl_ActorRead {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let actor = STEPControl_ActorRead::new();
        assert!(actor.is_enabled());
    }

    #[test]
    fn test_set_enabled() {
        let mut actor = STEPControl_ActorRead::new();
        actor.set_enabled(false);
        assert!(!actor.is_enabled());
    }
}
