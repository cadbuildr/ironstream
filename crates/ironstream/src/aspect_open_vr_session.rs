// FILE: aspect_open_vr_session.rs
// occt: Aspect_OpenVRSession

/// OpenVR session handle.
pub struct AspectOpenVRSession {
    handle: *mut std::ffi::c_void,
    is_initialized: bool,
}

impl AspectOpenVRSession {
    /// Create uninitialized session.
    pub fn new() -> Self {
        AspectOpenVRSession {
            handle: std::ptr::null_mut(),
            is_initialized: false,
        }
    }

    /// Check if initialized.
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Initialize session.
    pub fn initialize(&mut self) {
        self.is_initialized = true;
    }

    /// Close session.
    pub fn close(&mut self) {
        self.is_initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let session = AspectOpenVRSession::new();
        assert!(!session.is_initialized());
    }

    #[test]
    fn test_initialize() {
        let mut session = AspectOpenVRSession::new();
        session.initialize();
        assert!(session.is_initialized());
    }
}
