// FILE: b_rep_class_intersector.rs
// occt: BRepClass_Intersector

/// Intersector for 2D edge-line intersection
pub struct Intersector;

impl Intersector {
    pub fn new() -> Self {
        Intersector
    }

    pub fn perform(&mut self) {}

    pub fn local_geometry(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

impl Default for Intersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersector_creation() {
        let _inter = Intersector::new();
    }

    #[test]
    fn test_local_geometry() {
        let inter = Intersector::new();
        let (t, n, c) = inter.local_geometry();
        assert_eq!(t, 0.0);
        assert_eq!(n, 0.0);
        assert_eq!(c, 0.0);
    }
}
