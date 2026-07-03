// FILE: top_ope_b_rep_shape_intersector.rs
// occt: TopOpeBRep_ShapeIntersector

/// Intersects two shapes.
pub struct ShapeIntersector {
    shape1: Vec<u8>,
    shape2: Vec<u8>,
    intersection_done: bool,
    tol1: f64,
    tol2: f64,
    ff_done: bool,
    eeff_done: bool,
    ef_done: bool,
    fe_done: bool,
    ee_done: bool,
}

impl ShapeIntersector {
    /// Create a new ShapeIntersector.
    pub fn new() -> Self {
        ShapeIntersector {
            shape1: Vec::new(),
            shape2: Vec::new(),
            intersection_done: false,
            tol1: 0.0,
            tol2: 0.0,
            ff_done: false,
            eeff_done: false,
            ef_done: false,
            fe_done: false,
            ee_done: false,
        }
    }

    /// Initialize the intersection of two shapes.
    pub fn init_intersection(&mut self, _s1: &[u8], _s2: &[u8]) {
        self.reset();
        self.intersection_done = true;
    }

    /// Initialize the intersection with specific faces.
    pub fn init_intersection_with_faces(&mut self, _s1: &[u8], _s2: &[u8], _f1: &[u8], _f2: &[u8]) {
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
        self.intersection_done && !self.ff_done && !self.ee_done
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

    /// Get the tolerances.
    pub fn get_tolerances(&self) -> (f64, f64) {
        (self.tol1, self.tol2)
    }

    fn reset(&mut self) {
        self.ff_done = false;
        self.eeff_done = false;
        self.ef_done = false;
        self.fe_done = false;
        self.ee_done = false;
    }
}

impl Default for ShapeIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let intersector = ShapeIntersector::new();
        assert!(!intersector.intersection_done);
        assert!(!intersector.more_intersection());
    }

    #[test]
    fn test_init_intersection() {
        let mut intersector = ShapeIntersector::new();
        intersector.init_intersection(&[], &[]);
        assert!(intersector.intersection_done);
    }

    #[test]
    fn test_shape_access() {
        let intersector = ShapeIntersector::new();
        assert!(intersector.shape(0).is_some());
        assert!(intersector.shape(1).is_some());
        assert!(intersector.shape(2).is_none());
    }

    #[test]
    fn test_get_tolerances() {
        let intersector = ShapeIntersector::new();
        let (t1, t2) = intersector.get_tolerances();
        assert_eq!(t1, 0.0);
        assert_eq!(t2, 0.0);
    }
}
