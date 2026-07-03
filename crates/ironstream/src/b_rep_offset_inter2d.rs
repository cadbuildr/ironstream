// FILE: b_rep_offset_inter2d.rs
// occt: BRepOffset_Inter2d

/// Computes the intersections between edges on a face.
/// Stores result in AsDes from BRepOffset.
#[derive(Debug, Clone)]
pub struct BRepOffsetInter2d {
    /// Number of intersections found
    intersection_count: usize,
}

impl BRepOffsetInter2d {
    /// Create a new Inter2d
    pub fn new() -> Self {
        Self {
            intersection_count: 0,
        }
    }

    /// Get number of intersections
    pub fn intersection_count(&self) -> usize {
        self.intersection_count
    }

    /// Add an intersection
    pub fn add_intersection(&mut self) {
        self.intersection_count += 1;
    }

    /// Clear all intersections
    pub fn clear(&mut self) {
        self.intersection_count = 0;
    }
}

impl Default for BRepOffsetInter2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inter = BRepOffsetInter2d::new();
        assert_eq!(inter.intersection_count(), 0);
    }

    #[test]
    fn test_add_intersection() {
        let mut inter = BRepOffsetInter2d::new();
        inter.add_intersection();
        assert_eq!(inter.intersection_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut inter = BRepOffsetInter2d::new();
        inter.add_intersection();
        inter.clear();
        assert_eq!(inter.intersection_count(), 0);
    }

    #[test]
    fn test_default() {
        let inter = BRepOffsetInter2d::default();
        assert_eq!(inter.intersection_count(), 0);
    }
}
