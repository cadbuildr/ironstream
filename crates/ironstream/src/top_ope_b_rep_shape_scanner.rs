// FILE: top_ope_b_rep_shape_scanner.rs
// occt: TopOpeBRep_ShapeScanner

/// Scans shapes for intersecting subshapes.
pub struct ShapeScanner {
    current_index: usize,
    more: bool,
}

impl ShapeScanner {
    /// Create a new ShapeScanner.
    pub fn new() -> Self {
        ShapeScanner {
            current_index: 0,
            more: false,
        }
    }

    /// Clear the scanner.
    pub fn clear(&mut self) {
        self.current_index = 0;
        self.more = false;
    }

    /// Add boxes from a shape.
    pub fn add_boxes_make_cob(&mut self, _s: &[u8], _ts: u32, _ta: u32) {
        // Implementation stub
    }

    /// Initialize the scanner with a shape.
    pub fn init(&mut self, _e: &[u8]) {
        self.current_index = 0;
        self.more = false;
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.more
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.more {
            self.current_index += 1;
            self.more = false;
        }
    }

    /// Get the current item.
    pub fn current(&self) -> Option<&[u8]> {
        if self.more {
            Some(&[])
        } else {
            None
        }
    }

    /// Get the current index.
    pub fn index(&self) -> usize {
        self.current_index
    }
}

impl Default for ShapeScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scanner = ShapeScanner::new();
        assert!(!scanner.more());
    }

    #[test]
    fn test_clear() {
        let mut scanner = ShapeScanner::new();
        scanner.more = true;
        scanner.current_index = 5;
        scanner.clear();
        assert!(!scanner.more());
        assert_eq!(scanner.index(), 0);
    }

    #[test]
    fn test_init() {
        let mut scanner = ShapeScanner::new();
        scanner.init(&[]);
        assert!(!scanner.more());
        assert_eq!(scanner.index(), 0);
    }

    #[test]
    fn test_index() {
        let mut scanner = ShapeScanner::new();
        assert_eq!(scanner.index(), 0);
        scanner.current_index = 5;
        assert_eq!(scanner.index(), 5);
    }
}
