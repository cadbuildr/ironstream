// FILE: step_to_topo_ds_point_pair.rs
// occt: StepToTopoDS_PointPair

use std::hash::{Hash, Hasher};

/// Stores a pair of Points from STEP
#[derive(Debug, Clone)]
pub struct StepToTopoDS_PointPair {
    p1: String,
    p2: String,
}

impl StepToTopoDS_PointPair {
    pub fn new(p1: String, p2: String) -> Self {
        StepToTopoDS_PointPair { p1, p2 }
    }

    pub fn get_point1(&self) -> &str {
        &self.p1
    }

    pub fn get_point2(&self) -> &str {
        &self.p2
    }
}

impl PartialEq for StepToTopoDS_PointPair {
    fn eq(&self, other: &Self) -> bool {
        ((self.p1 == other.p1) && (self.p2 == other.p2))
            || ((self.p1 == other.p2) && (self.p2 == other.p1))
    }
}

impl Eq for StepToTopoDS_PointPair {}

impl Hash for StepToTopoDS_PointPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut h1 = 0u64;
        let mut h2 = 0u64;

        // Simple hash of the strings
        for c in self.p1.chars() {
            h1 = h1.wrapping_mul(31).wrapping_add(c as u64);
        }
        for c in self.p2.chars() {
            h2 = h2.wrapping_mul(31).wrapping_add(c as u64);
        }

        // Normalize order to ensure symmetry: h(p1,p2) == h(p2,p1)
        let (a, b) = if h1 > h2 { (h2, h1) } else { (h1, h2) };

        a.hash(state);
        b.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pair = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        assert_eq!(pair.get_point1(), "p1");
        assert_eq!(pair.get_point2(), "p2");
    }

    #[test]
    fn test_equality_same_order() {
        let pair1 = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        let pair2 = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        assert_eq!(pair1, pair2);
    }

    #[test]
    fn test_equality_reversed_order() {
        let pair1 = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        let pair2 = StepToTopoDS_PointPair::new("p2".to_string(), "p1".to_string());
        assert_eq!(pair1, pair2);
    }

    #[test]
    fn test_inequality() {
        let pair1 = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        let pair2 = StepToTopoDS_PointPair::new("p1".to_string(), "p3".to_string());
        assert_ne!(pair1, pair2);
    }

    #[test]
    fn test_hash_symmetry() {
        use std::collections::hash_map::DefaultHasher;

        let pair1 = StepToTopoDS_PointPair::new("p1".to_string(), "p2".to_string());
        let pair2 = StepToTopoDS_PointPair::new("p2".to_string(), "p1".to_string());

        let mut hasher1 = DefaultHasher::new();
        pair1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        pair2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2, "Hashes should be the same for reversed pairs");
    }
}
