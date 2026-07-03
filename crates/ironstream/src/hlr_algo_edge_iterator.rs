// FILE: hlr_algo_edge_iterator.rs
// occt: HLRAlgo_EdgeIterator

/// Iterator over visible and hidden intervals of an edge.
pub struct HlrAlgoEdgeIterator {
    nb_vis: usize,
    nb_hid: usize,
    i_vis: usize,
    i_hid: usize,
    hid_start: f64,
    hid_end: f64,
    hid_tol_start: f32,
    hid_tol_end: f32,
}

impl HlrAlgoEdgeIterator {
    /// Create a new edge iterator.
    pub fn new() -> Self {
        HlrAlgoEdgeIterator {
            nb_vis: 0,
            nb_hid: 0,
            i_vis: 0,
            i_hid: 0,
            hid_start: 0.0,
            hid_end: 0.0,
            hid_tol_start: 0.0,
            hid_tol_end: 0.0,
        }
    }

    /// Initialize to iterate over hidden intervals.
    pub fn init_hidden(&mut self, nb_hidden: usize) {
        self.nb_hid = nb_hidden;
        self.i_hid = 0;
    }

    /// Check if there are more hidden intervals.
    pub fn more_hidden(&self) -> bool {
        self.i_hid < self.nb_hid
    }

    /// Move to next hidden interval.
    pub fn next_hidden(&mut self) {
        if self.i_hid < self.nb_hid {
            self.i_hid += 1;
        }
    }

    /// Get bounds and tolerances of current hidden interval.
    pub fn hidden(&self) -> (f64, f32, f64, f32) {
        (self.hid_start, self.hid_tol_start, self.hid_end, self.hid_tol_end)
    }

    /// Set current hidden interval data.
    pub fn set_hidden(&mut self, start: f64, tol_start: f32, end: f64, tol_end: f32) {
        self.hid_start = start;
        self.hid_tol_start = tol_start;
        self.hid_end = end;
        self.hid_tol_end = tol_end;
    }

    /// Initialize to iterate over visible intervals.
    pub fn init_visible(&mut self, nb_visible: usize) {
        self.nb_vis = nb_visible;
        self.i_vis = 0;
    }

    /// Check if there are more visible intervals.
    pub fn more_visible(&self) -> bool {
        self.i_vis < self.nb_vis
    }

    /// Move to next visible interval.
    pub fn next_visible(&mut self) {
        if self.i_vis < self.nb_vis {
            self.i_vis += 1;
        }
    }

    /// Get bounds and tolerances of current visible interval.
    pub fn visible(&self) -> (f64, f32, f64, f32) {
        (self.hid_start, self.hid_tol_start, self.hid_end, self.hid_tol_end)
    }

    /// Reset iterator.
    pub fn reset(&mut self) {
        self.nb_vis = 0;
        self.nb_hid = 0;
        self.i_vis = 0;
        self.i_hid = 0;
    }
}

impl Default for HlrAlgoEdgeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let iter = HlrAlgoEdgeIterator::new();
        assert!(!iter.more_hidden());
        assert!(!iter.more_visible());
    }

    #[test]
    fn test_hidden_iteration() {
        let mut iter = HlrAlgoEdgeIterator::new();
        iter.init_hidden(3);
        assert!(iter.more_hidden());
        iter.set_hidden(0.0, 0.01, 0.5, 0.02);
        assert_eq!(iter.hidden(), (0.0, 0.01, 0.5, 0.02));
        iter.next_hidden();
        assert!(iter.more_hidden());
        iter.next_hidden();
        iter.next_hidden();
        assert!(!iter.more_hidden());
    }

    #[test]
    fn test_visible_iteration() {
        let mut iter = HlrAlgoEdgeIterator::new();
        iter.init_visible(2);
        assert!(iter.more_visible());
        iter.next_visible();
        assert!(iter.more_visible());
        iter.next_visible();
        assert!(!iter.more_visible());
    }

    #[test]
    fn test_reset() {
        let mut iter = HlrAlgoEdgeIterator::new();
        iter.init_hidden(5);
        iter.init_visible(3);
        iter.reset();
        assert!(!iter.more_hidden());
        assert!(!iter.more_visible());
    }
}
