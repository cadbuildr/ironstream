// FILE: proj_lib_projector.rs
// occt: ProjLib_Projector

/// Base projector class.
pub struct ProjLibProjector {
    is_done: bool,
}

impl ProjLibProjector {
    /// Create a new projector.
    pub fn new() -> Self {
        ProjLibProjector { is_done: false }
    }

    /// Perform projection.
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    /// Check if projection is done.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for ProjLibProjector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_projector() {
        let proj = ProjLibProjector::new();
        assert!(!proj.is_done());
    }

    #[test]
    fn test_perform() {
        let mut proj = ProjLibProjector::new();
        proj.perform();
        assert!(proj.is_done());
    }
}
