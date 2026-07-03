// FILE: b_rep_class_edge.rs
// occt: BRepClass_Edge

/// Description of an edge for classification
pub struct Edge {
    max_tolerance: f64,
    use_bnd_box: bool,
}

impl Edge {
    pub fn new() -> Self {
        Edge {
            max_tolerance: 1e-7,
            use_bnd_box: false,
        }
    }

    pub fn max_tolerance(&self) -> f64 {
        self.max_tolerance
    }

    pub fn set_max_tolerance(&mut self, val: f64) {
        self.max_tolerance = val;
    }

    pub fn use_bnd_box(&self) -> bool {
        self.use_bnd_box
    }

    pub fn set_use_bnd_box(&mut self, val: bool) {
        self.use_bnd_box = val;
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new();
        assert!(!edge.use_bnd_box());
        assert!(edge.max_tolerance() > 0.0);
    }

    #[test]
    fn test_max_tolerance() {
        let mut edge = Edge::new();
        edge.set_max_tolerance(0.01);
        assert_eq!(edge.max_tolerance(), 0.01);
    }

    #[test]
    fn test_use_bnd_box() {
        let mut edge = Edge::new();
        assert!(!edge.use_bnd_box());
        edge.set_use_bnd_box(true);
        assert!(edge.use_bnd_box());
    }
}
