// FILE: tdf_closure_mode.rs
// occt: TDF_ClosureMode

/// Provides options for closure management.
/// The closure mode controls how descendant labels and references are traversed.
pub struct TdfClosureMode {
    my_flags: u8,
}

impl TdfClosureMode {
    const DESCENDANTS_FLAG: u8 = 0x01;
    const REFERENCES_FLAG: u8 = 0x02;

    /// Creates an object with all modes set to the given value.
    pub fn new(mode: bool) -> Self {
        let flags = if mode {
            Self::DESCENDANTS_FLAG | Self::REFERENCES_FLAG
        } else {
            0
        };
        TdfClosureMode { my_flags: flags }
    }

    /// Sets the mode "Descendants" to the given status.
    /// "Descendants" mode means we add to the data set the children labels
    /// of each USER GIVEN label.
    pub fn set_descendants(&mut self, status: bool) {
        if status {
            self.my_flags |= Self::DESCENDANTS_FLAG;
        } else {
            self.my_flags &= !Self::DESCENDANTS_FLAG;
        }
    }

    /// Returns true if the mode "Descendants" is set.
    pub fn descendants(&self) -> bool {
        (self.my_flags & Self::DESCENDANTS_FLAG) != 0
    }

    /// Sets the mode "References" to the given status.
    /// "References" mode means we add to the data set the descendants
    /// of an attribute, by calling the attribute method Descendants().
    pub fn set_references(&mut self, status: bool) {
        if status {
            self.my_flags |= Self::REFERENCES_FLAG;
        } else {
            self.my_flags &= !Self::REFERENCES_FLAG;
        }
    }

    /// Returns true if the mode "References" is set.
    pub fn references(&self) -> bool {
        (self.my_flags & Self::REFERENCES_FLAG) != 0
    }
}

impl Default for TdfClosureMode {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_mode_default_true() {
        let mode = TdfClosureMode::new(true);
        assert!(mode.descendants());
        assert!(mode.references());
    }

    #[test]
    fn test_closure_mode_default_false() {
        let mode = TdfClosureMode::new(false);
        assert!(!mode.descendants());
        assert!(!mode.references());
    }

    #[test]
    fn test_closure_mode_set_descendants() {
        let mut mode = TdfClosureMode::new(false);
        assert!(!mode.descendants());
        mode.set_descendants(true);
        assert!(mode.descendants());
        mode.set_descendants(false);
        assert!(!mode.descendants());
    }

    #[test]
    fn test_closure_mode_set_references() {
        let mut mode = TdfClosureMode::new(false);
        assert!(!mode.references());
        mode.set_references(true);
        assert!(mode.references());
        mode.set_references(false);
        assert!(!mode.references());
    }

    #[test]
    fn test_closure_mode_independent_flags() {
        let mut mode = TdfClosureMode::new(false);
        mode.set_descendants(true);
        assert!(mode.descendants());
        assert!(!mode.references());
    }

    #[test]
    fn test_closure_mode_default() {
        let mode = TdfClosureMode::default();
        assert!(mode.descendants());
        assert!(mode.references());
    }
}
