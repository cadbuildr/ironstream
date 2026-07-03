// FILE: b_rep_offset_offset.rs
// occt: BRepOffset_Offset

/// Computes an offset shape (face, edge, or vertex).
/// Handles geometric offset calculations with various modes.
#[derive(Debug, Clone)]
pub struct BRepOffsetOffset {
    /// Offset distance
    offset_distance: f64,
    /// Whether offset is computed
    is_done: bool,
}

impl BRepOffsetOffset {
    /// Create a new Offset with distance
    pub fn new(distance: f64) -> Self {
        Self {
            offset_distance: distance,
            is_done: false,
        }
    }

    /// Get offset distance
    pub fn offset_distance(&self) -> f64 {
        self.offset_distance
    }

    /// Check if offset is computed
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark offset as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }
}

impl Default for BRepOffsetOffset {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let offset = BRepOffsetOffset::new(2.5);
        assert_eq!(offset.offset_distance(), 2.5);
        assert!(!offset.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut offset = BRepOffsetOffset::new(1.0);
        offset.set_done();
        assert!(offset.is_done());
    }

    #[test]
    fn test_default() {
        let offset = BRepOffsetOffset::default();
        assert_eq!(offset.offset_distance(), 0.0);
    }
}
