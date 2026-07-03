// FILE: b_rep_offset_simple_offset.rs
// occt: BRepOffset_SimpleOffset

/// Simple geometric offset computation.
/// Provides basic offset operations without complex topology handling.
#[derive(Debug, Clone)]
pub struct BRepOffsetSimpleOffset {
    /// Offset distance
    offset_distance: f64,
}

impl BRepOffsetSimpleOffset {
    /// Create a new SimpleOffset with distance
    pub fn new(distance: f64) -> Self {
        Self {
            offset_distance: distance,
        }
    }

    /// Get offset distance
    pub fn offset_distance(&self) -> f64 {
        self.offset_distance
    }

    /// Set offset distance
    pub fn set_offset_distance(&mut self, distance: f64) {
        self.offset_distance = distance;
    }
}

impl Default for BRepOffsetSimpleOffset {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let offset = BRepOffsetSimpleOffset::new(1.5);
        assert_eq!(offset.offset_distance(), 1.5);
    }

    #[test]
    fn test_set_offset_distance() {
        let mut offset = BRepOffsetSimpleOffset::new(1.0);
        offset.set_offset_distance(3.0);
        assert_eq!(offset.offset_distance(), 3.0);
    }

    #[test]
    fn test_default() {
        let offset = BRepOffsetSimpleOffset::default();
        assert_eq!(offset.offset_distance(), 0.0);
    }
}
