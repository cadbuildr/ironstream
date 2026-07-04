// FILE: step_to_topo_ds_root.rs
// occt: StepToTopoDS_Root

/// Common services for all classes of StepToTopoDS which report error and sets/returns precision.
pub struct StepToTopoDS_Root {
    done: bool,
    precision: f64,
    max_tol: f64,
}

impl StepToTopoDS_Root {
    pub fn new() -> Self {
        StepToTopoDS_Root {
            done: false,
            precision: 0.0,
            max_tol: 0.0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn precision(&self) -> f64 {
        self.precision
    }

    pub fn set_precision(&mut self, preci: f64) {
        self.precision = preci;
    }

    pub fn max_tol(&self) -> f64 {
        self.max_tol
    }

    pub fn set_max_tol(&mut self, maxpreci: f64) {
        self.max_tol = maxpreci;
    }
}

impl Default for StepToTopoDS_Root {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let root = StepToTopoDS_Root::new();
        assert!(!root.is_done());
        assert_eq!(root.precision(), 0.0);
        assert_eq!(root.max_tol(), 0.0);
    }

    #[test]
    fn test_set_done() {
        let mut root = StepToTopoDS_Root::new();
        root.set_done(true);
        assert!(root.is_done());
        root.set_done(false);
        assert!(!root.is_done());
    }

    #[test]
    fn test_set_precision() {
        let mut root = StepToTopoDS_Root::new();
        root.set_precision(0.001);
        assert_eq!(root.precision(), 0.001);
    }

    #[test]
    fn test_set_max_tol() {
        let mut root = StepToTopoDS_Root::new();
        root.set_max_tol(0.01);
        assert_eq!(root.max_tol(), 0.01);
    }

    #[test]
    fn test_default() {
        let root = StepToTopoDS_Root::default();
        assert!(!root.is_done());
    }
}
