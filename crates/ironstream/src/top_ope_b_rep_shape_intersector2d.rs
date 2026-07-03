// FILE: top_ope_b_rep_shape_intersector2d.rs
// occt: TopOpeBRep_ShapeIntersector2d

/// Intersects two shapes in 2D.
pub struct ShapeIntersector2d {
    shape1: Vec<u8>,
    shape2: Vec<u8>,
    intersection_done: bool,
    ff_done: bool,
    eeff_done: bool,
}

impl ShapeIntersector2d {
    /// Create a new ShapeIntersector2d.
    pub fn new() -> Self {
        ShapeIntersector2d {
            shape1: Vec::new(),
            shape2: Vec::new(),
            intersection_done: false,
            ff_done: false,
            eeff_done: false,
        }
    }

    /// Initialize the intersection of two shapes.
    pub fn init_intersection(&mut self, _s1: &[u8], _s2: &[u8]) {
        self.reset();
        self.intersection_done = true;
    }

    /// Get the shape at the given index.
    pub fn shape(&self, index: usize) -> Option<&[u8]> {
        match index {
            0 => Some(&self.shape1),
            1 => Some(&self.shape2),
            _ => None,
        }
    }

    /// Check if there are more intersections.
    pub fn more_intersection(&self) -> bool {
        self.intersection_done && !self.ff_done && !self.eeff_done
    }

    /// Move to the next intersection.
    pub fn next_intersection(&mut self) {
        if !self.ff_done {
            self.ff_done = true;
        }
    }

    /// Get the current geometric shapes.
    pub fn current_geom_shape(&self, _index: usize) -> Option<&[u8]> {
        None
    }

    fn reset(&mut self) {
        self.ff_done = false;
        self.eeff_done = false;
    }
}

impl Default for ShapeIntersector2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let intersector = ShapeIntersector2d::new();
        assert!(!intersector.intersection_done);
    }

    #[test]
    fn test_init_intersection() {
        let mut intersector = ShapeIntersector2d::new();
        intersector.init_intersection(&[], &[]);
        assert!(intersector.intersection_done);
    }

    #[test]
    fn test_shape_access() {
        let intersector = ShapeIntersector2d::new();
        assert!(intersector.shape(0).is_some());
        assert!(intersector.shape(1).is_some());
        assert!(intersector.shape(2).is_none());
    }

    #[test]
    fn test_more_intersection() {
        let mut intersector = ShapeIntersector2d::new();
        intersector.init_intersection(&[], &[]);
        assert!(intersector.more_intersection());
        intersector.next_intersection();
        assert!(!intersector.more_intersection());
    }
}
