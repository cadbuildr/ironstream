// FILE: int_tools_shrunk_range.rs
// occt: IntTools_ShrunkRange

/// Computed shrunk range for an edge.
#[derive(Clone, Debug)]
pub struct IntToolsShrunkRange {
    t1: f64,
    t2: f64,
    ts1: f64,
    ts2: f64,
    is_done: bool,
    is_splittable: bool,
    length: f64,
}

impl IntToolsShrunkRange {
    /// Create an empty shrunk range.
    pub fn new() -> Self {
        IntToolsShrunkRange {
            t1: 0.0,
            t2: 0.0,
            ts1: 0.0,
            ts2: 0.0,
            is_done: false,
            is_splittable: false,
            length: 0.0,
        }
    }

    /// Set the original range.
    pub fn set_range(&mut self, t1: f64, t2: f64) {
        self.t1 = t1;
        self.t2 = t2;
    }

    /// Set the shrunk range.
    pub fn set_shrunk_range(&mut self, ts1: f64, ts2: f64) {
        self.ts1 = ts1;
        self.ts2 = ts2;
        self.is_done = true;
    }

    /// Get the shrunk range.
    pub fn shrunk_range(&self) -> (f64, f64) {
        (self.ts1, self.ts2)
    }

    /// Set the length.
    pub fn set_length(&mut self, len: f64) {
        self.length = len;
    }

    /// Get the length.
    pub fn length(&self) -> f64 {
        self.length
    }

    /// Check if range is computed.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Check if edge is splittable.
    pub fn is_splittable(&self) -> bool {
        self.is_splittable
    }

    /// Set splittable flag.
    pub fn set_splittable(&mut self, flag: bool) {
        self.is_splittable = flag;
    }

    /// Perform computation (placeholder).
    pub fn perform(&mut self) {
        if self.is_done {
            return;
        }
        // Simplified: assume shrunk range is same as original
        self.ts1 = self.t1;
        self.ts2 = self.t2;
        self.is_done = true;
        self.is_splittable = true;
    }
}

impl Default for IntToolsShrunkRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sr = IntToolsShrunkRange::new();
        assert!(!sr.is_done());
        assert!(!sr.is_splittable());
    }

    #[test]
    fn test_set_range() {
        let mut sr = IntToolsShrunkRange::new();
        sr.set_range(1.0, 5.0);
        assert_eq!(sr.t1, 1.0);
        assert_eq!(sr.t2, 5.0);
    }

    #[test]
    fn test_set_shrunk_range() {
        let mut sr = IntToolsShrunkRange::new();
        sr.set_shrunk_range(1.5, 4.5);
        assert!(sr.is_done());
        let (ts1, ts2) = sr.shrunk_range();
        assert_eq!(ts1, 1.5);
        assert_eq!(ts2, 4.5);
    }

    #[test]
    fn test_perform() {
        let mut sr = IntToolsShrunkRange::new();
        sr.set_range(0.0, 10.0);
        sr.perform();
        assert!(sr.is_done());
    }
}
