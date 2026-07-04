// FILE: step_control_actor_write.rs
// occt: STEPControl_ActorWrite

/// Actor for writing STEP files
pub struct STEPControl_ActorWrite {
    enabled: bool,
}

impl STEPControl_ActorWrite {
    pub fn new() -> Self {
        STEPControl_ActorWrite { enabled: true }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for STEPControl_ActorWrite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let actor = STEPControl_ActorWrite::new();
        assert!(actor.is_enabled());
    }

    #[test]
    fn test_set_enabled() {
        let mut actor = STEPControl_ActorWrite::new();
        actor.set_enabled(false);
        assert!(!actor.is_enabled());
    }
}
