// FILE: top_ope_b_rep_ds_marker.rs
// occt: TopOpeBRepDS_Marker

/// A marker class for marking array elements as visited
#[derive(Debug, Clone)]
pub struct Marker {
    /// Array of boolean flags
    marks: Vec<bool>,
}

impl Marker {
    /// Create new Marker
    pub fn new() -> Self {
        Marker {
            marks: Vec::new(),
        }
    }

    /// Reset all marks to false
    pub fn reset(&mut self) {
        for mark in &mut self.marks {
            *mark = false;
        }
    }

    /// Set mark at index i
    pub fn set(&mut self, i: usize, b: bool) {
        if i < self.marks.len() {
            self.marks[i] = b;
        }
    }

    /// Set mark at many positions
    pub fn set_many(&mut self, b: bool, n: usize, start_index: usize) {
        for j in 0..n {
            let idx = start_index + j;
            if idx < self.marks.len() {
                self.marks[idx] = b;
            }
        }
    }

    /// Get mark at index i
    pub fn get(&self, i: usize) -> bool {
        if i < self.marks.len() {
            self.marks[i]
        } else {
            false
        }
    }

    /// Allocate space for n marks
    pub fn allocate(&mut self, n: usize) {
        self.marks.clear();
        self.marks.resize(n, false);
    }

    /// Get length of marks array
    pub fn len(&self) -> usize {
        self.marks.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }
}

impl Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_new() {
        let m = Marker::new();
        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_marker_allocate() {
        let mut m = Marker::new();
        m.allocate(10);
        assert_eq!(m.len(), 10);
        for i in 0..10 {
            assert!(!m.get(i));
        }
    }

    #[test]
    fn test_marker_set_get() {
        let mut m = Marker::new();
        m.allocate(5);
        m.set(0, true);
        m.set(2, true);
        m.set(4, true);
        assert!(m.get(0));
        assert!(!m.get(1));
        assert!(m.get(2));
        assert!(!m.get(3));
        assert!(m.get(4));
    }

    #[test]
    fn test_marker_reset() {
        let mut m = Marker::new();
        m.allocate(5);
        m.set(0, true);
        m.set(1, true);
        m.set(2, true);
        m.reset();
        for i in 0..5 {
            assert!(!m.get(i));
        }
    }

    #[test]
    fn test_marker_set_many() {
        let mut m = Marker::new();
        m.allocate(10);
        m.set_many(true, 3, 2);
        assert!(!m.get(0));
        assert!(!m.get(1));
        assert!(m.get(2));
        assert!(m.get(3));
        assert!(m.get(4));
        assert!(!m.get(5));
    }

    #[test]
    fn test_marker_get_out_of_bounds() {
        let m = Marker::new();
        assert!(!m.get(0));
        assert!(!m.get(100));
    }
}
