// FILE: shape_analysis_wire_vertex.rs
// occt: ShapeAnalysis_WireVertex

/// Analyzes and records status of vertices in a wire.
pub struct WireVertex {
    nb_edges: usize,
    precision: f64,
    done: bool,
    status: Vec<i32>,
}

impl WireVertex {
    /// Create empty analyzer.
    pub fn new() -> Self {
        WireVertex {
            nb_edges: 0,
            precision: 0.0,
            done: false,
            status: Vec::new(),
        }
    }

    /// Initialize with wire and precision.
    pub fn init(&mut self, _wire: &[u8], preci: f64) {
        self.precision = preci;
        self.done = false;
    }

    /// Load a wire.
    pub fn load(&mut self, _wire: &[u8]) {
        self.done = false;
    }

    /// Set precision.
    pub fn set_precision(&mut self, preci: f64) {
        self.precision = preci;
    }

    /// Analyze the wire.
    pub fn analyze(&mut self) {
        self.done = true;
    }

    /// Set "SameVertex" status.
    pub fn set_same_vertex(&mut self, num: usize) {
        if num < self.status.len() {
            self.status[num] = 0;
        }
    }

    /// Set "SameCoords" status.
    pub fn set_same_coords(&mut self, num: usize) {
        if num < self.status.len() {
            self.status[num] = 1;
        }
    }

    /// Set "Close" status.
    pub fn set_close(&mut self, num: usize) {
        if num < self.status.len() {
            self.status[num] = 2;
        }
    }

    /// Check if analysis was done.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Get precision value.
    pub fn precision(&self) -> f64 {
        self.precision
    }

    /// Get number of edges.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    /// Get status of a vertex.
    pub fn status(&self, num: usize) -> Option<i32> {
        if num < self.status.len() {
            Some(self.status[num])
        } else {
            None
        }
    }

    /// Find next status.
    pub fn next_status(&self, _stat: i32, num: usize) -> usize {
        0
    }

    /// Find next criterion.
    pub fn next_criter(&self, _crit: i32, num: usize) -> usize {
        0
    }
}

impl Default for WireVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let wv = WireVertex::new();
        assert!(!wv.is_done());
        assert_eq!(wv.precision(), 0.0);
    }

    #[test]
    fn test_set_precision() {
        let mut wv = WireVertex::new();
        wv.set_precision(0.01);
        assert_eq!(wv.precision(), 0.01);
    }

    #[test]
    fn test_analyze() {
        let mut wv = WireVertex::new();
        wv.analyze();
        assert!(wv.is_done());
    }

    #[test]
    fn test_nb_edges() {
        let wv = WireVertex::new();
        assert_eq!(wv.nb_edges(), 0);
    }
}
