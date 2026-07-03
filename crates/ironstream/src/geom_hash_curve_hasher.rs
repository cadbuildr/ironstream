// FILE: geom_hash_curve_hasher.rs
// occt: GeomHash_CurveHasher

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct GeomHashCurveHasher;

impl GeomHashCurveHasher {
    /// Compute hash of a curve.
    pub fn hash_curve(curve_id: u32, precision: f64) -> u64 {
        let mut hasher = DefaultHasher::new();
        curve_id.hash(&mut hasher);
        precision.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    /// Compute hash of curve with tolerance.
    pub fn hash_curve_with_tolerance(
        curve_id: u32,
        precision: f64,
        tolerance: f64,
    ) -> u64 {
        let mut hasher = DefaultHasher::new();
        curve_id.hash(&mut hasher);
        precision.to_bits().hash(&mut hasher);
        tolerance.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_curve() {
        let h1 = GeomHashCurveHasher::hash_curve(1, 1e-7);
        let h2 = GeomHashCurveHasher::hash_curve(1, 1e-7);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_curve_different_ids() {
        let h1 = GeomHashCurveHasher::hash_curve(1, 1e-7);
        let h2 = GeomHashCurveHasher::hash_curve(2, 1e-7);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_curve_with_tolerance() {
        let h1 = GeomHashCurveHasher::hash_curve_with_tolerance(1, 1e-7, 1e-5);
        let h2 = GeomHashCurveHasher::hash_curve_with_tolerance(1, 1e-7, 1e-5);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_curve_different_tolerance() {
        let h1 = GeomHashCurveHasher::hash_curve_with_tolerance(1, 1e-7, 1e-5);
        let h2 = GeomHashCurveHasher::hash_curve_with_tolerance(1, 1e-7, 1e-4);
        assert_ne!(h1, h2);
    }
}
