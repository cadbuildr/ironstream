// FILE: topo_ds_to_step_root.rs
// occt: TopoDSToStep_Root

/// Base class for TopoDSToStep builders.
pub struct Root {
    tolerance: f64,
    done: bool,
}

impl Root {
    pub fn new() -> Self {
        Root {
            tolerance: 0.0001,
            done: false,
        }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, d: bool) {
        self.done = d;
    }
}

impl Default for Root {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let root = Root::new();
        assert_eq!(root.tolerance(), 0.0001);
        assert!(!root.is_done());
    }

    #[test]
    fn test_tolerance() {
        let mut root = Root::new();
        root.set_tolerance(0.01);
        assert_eq!(root.tolerance(), 0.01);
    }

    #[test]
    fn test_done() {
        let mut root = Root::new();
        root.set_done(true);
        assert!(root.is_done());
    }
}
