// FILE: viewer_test_auto_updater.rs
// occt: ViewerTest_AutoUpdater

/// Manages automatic viewport updates during interactive manipulation
#[derive(Clone, Debug)]
pub struct ViewerTestAutoUpdater {
    is_enabled: bool,
}

impl ViewerTestAutoUpdater {
    /// Create a new auto updater
    pub fn new() -> Self {
        ViewerTestAutoUpdater { is_enabled: true }
    }

    /// Enable automatic updates
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    /// Disable automatic updates
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    /// Check if auto update is enabled
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

impl Default for ViewerTestAutoUpdater {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_auto_updater() {
        let updater = ViewerTestAutoUpdater::new();
        assert!(updater.is_enabled());
    }

    #[test]
    fn test_enable_disable() {
        let mut updater = ViewerTestAutoUpdater::new();
        assert!(updater.is_enabled());
        updater.disable();
        assert!(!updater.is_enabled());
        updater.enable();
        assert!(updater.is_enabled());
    }

    #[test]
    fn test_default() {
        let updater = ViewerTestAutoUpdater::default();
        assert!(updater.is_enabled());
    }

    #[test]
    fn test_clone() {
        let mut updater1 = ViewerTestAutoUpdater::new();
        updater1.disable();
        let updater2 = updater1.clone();
        assert!(!updater2.is_enabled());
    }
}
