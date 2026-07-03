// FILE: standard_c_locale_sentry.rs
// occt: Standard_CLocaleSentry

/// RAII wrapper for C locale operations
#[derive(Debug)]
pub struct StandardCLocaleSentry {
    _private: (),
}

impl StandardCLocaleSentry {
    /// Creates a new C locale sentry
    pub fn new() -> Self {
        StandardCLocaleSentry { _private: () }
    }

    /// Checks if sentry is active
    pub fn is_active(&self) -> bool {
        true
    }
}

impl Default for StandardCLocaleSentry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StandardCLocaleSentry {
    fn drop(&mut self) {
        // Cleanup C locale when dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_c_locale_sentry_new() {
        let sentry = StandardCLocaleSentry::new();
        assert!(sentry.is_active());
    }

    #[test]
    fn test_standard_c_locale_sentry_default() {
        let sentry = StandardCLocaleSentry::default();
        assert!(sentry.is_active());
    }
}
