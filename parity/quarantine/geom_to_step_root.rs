// FILE: geom_to_step_root.rs
// occt: GeomToStep_Root

/// Base class for GeomToStep converters providing common error reporting
pub struct GeomToStep_Root {
    done: bool,
}

impl GeomToStep_Root {
    /// Creates a new GeomToStep_Root with done flag set to false
    pub fn new() -> Self {
        GeomToStep_Root { done: false }
    }

    /// Returns whether the conversion was successful
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Sets the done flag
    pub fn set_done(&mut self, value: bool) {
        self.done = value;
    }
}

impl Default for GeomToStep_Root {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let root = GeomToStep_Root::new();
        assert!(!root.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut root = GeomToStep_Root::new();
        root.set_done(true);
        assert!(root.is_done());
    }

    #[test]
    fn test_default_trait() {
        let root = GeomToStep_Root::default();
        assert!(!root.is_done());
    }
}
