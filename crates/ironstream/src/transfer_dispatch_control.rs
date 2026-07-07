// FILE: transfer_dispatch_control.rs
// occt: Transfer_DispatchControl

/// Controls dispatch operations in transfer processing.
/// Manages the dispatching of entities through multiple actors.
#[derive(Clone, Debug)]
pub struct TransferDispatchControl {
    /// Whether dispatch is enabled
    enabled: bool,
    /// Dispatch mode (0=sequential, 1=parallel)
    mode: u32,
}

impl TransferDispatchControl {
    /// Creates a new dispatch control.
    pub fn new() -> Self {
        Self {
            enabled: true,
            mode: 0,
        }
    }

    /// Returns whether dispatch is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Sets the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns the dispatch mode.
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// Sets the dispatch mode.
    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }
}

impl Default for TransferDispatchControl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let control = TransferDispatchControl::new();
        assert!(control.is_enabled());
        assert_eq!(control.mode(), 0);
    }

    #[test]
    fn test_set_enabled() {
        let mut control = TransferDispatchControl::new();
        control.set_enabled(false);
        assert!(!control.is_enabled());

        control.set_enabled(true);
        assert!(control.is_enabled());
    }

    #[test]
    fn test_set_mode() {
        let mut control = TransferDispatchControl::new();
        control.set_mode(1);
        assert_eq!(control.mode(), 1);

        control.set_mode(0);
        assert_eq!(control.mode(), 0);
    }
}
