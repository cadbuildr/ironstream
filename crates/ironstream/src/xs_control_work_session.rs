// FILE: xs_control_work_session.rs
// occt: XSControl_WorkSession

/// A work session for exchange format processing.
/// Manages the overall state and configuration for a complete exchange operation.
#[derive(Clone, Debug)]
pub struct XSControlWorkSession {
    /// Session identifier
    session_id: u32,
    /// Whether the session is active
    is_active: bool,
    /// Model loaded flag
    model_loaded: bool,
}

impl XSControlWorkSession {
    /// Creates a new work session.
    pub fn new() -> Self {
        Self {
            session_id: 1,
            is_active: true,
            model_loaded: false,
        }
    }

    /// Returns the session ID.
    pub fn id(&self) -> u32 {
        self.session_id
    }

    /// Returns whether the session is active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Sets the active state.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Returns whether a model is loaded.
    pub fn model_loaded(&self) -> bool {
        self.model_loaded
    }

    /// Sets the model loaded state.
    pub fn set_model_loaded(&mut self, loaded: bool) {
        self.model_loaded = loaded;
    }

    /// Clears the session.
    pub fn clear(&mut self) {
        self.model_loaded = false;
    }
}

impl Default for XSControlWorkSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let session = XSControlWorkSession::new();
        assert!(session.is_active());
        assert!(!session.model_loaded());
    }

    #[test]
    fn test_set_active() {
        let mut session = XSControlWorkSession::new();
        assert!(session.is_active());

        session.set_active(false);
        assert!(!session.is_active());
    }

    #[test]
    fn test_set_model_loaded() {
        let mut session = XSControlWorkSession::new();
        assert!(!session.model_loaded());

        session.set_model_loaded(true);
        assert!(session.model_loaded());
    }

    #[test]
    fn test_clear() {
        let mut session = XSControlWorkSession::new();
        session.set_model_loaded(true);
        assert!(session.model_loaded());

        session.clear();
        assert!(!session.model_loaded());
    }
}
