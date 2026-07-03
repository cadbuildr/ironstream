// FILE: geom2d_hash_curve_hasher.rs
// occt: Geom2dHash_CurveHasher

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hasher for 2D curves
#[derive(Debug, Clone)]
pub struct Geom2dHashCurveHasher {
    hash_value: u64,
}

impl Geom2dHashCurveHasher {
    pub fn new() -> Self {
        Geom2dHashCurveHasher { hash_value: 0 }
    }

    /// Hash a point
    pub fn hash_point(&mut self, x: f64, y: f64) {
        let mut hasher = DefaultHasher::new();
        x.to_bits().hash(&mut hasher);
        y.to_bits().hash(&mut hasher);
        self.hash_value ^= hasher.finish();
    }

    /// Hash a curve segment
    pub fn hash_segment(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.hash_point(x1, y1);
        self.hash_point(x2, y2);
    }

    /// Get the current hash value
    pub fn value(&self) -> u64 {
        self.hash_value
    }

    /// Reset the hash
    pub fn reset(&mut self) {
        self.hash_value = 0;
    }
}

impl Default for Geom2dHashCurveHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_hasher_new() {
        let hasher = Geom2dHashCurveHasher::new();
        assert_eq!(hasher.value(), 0);
    }

    #[test]
    fn test_curve_hasher_hash_point() {
        let mut hasher = Geom2dHashCurveHasher::new();
        hasher.hash_point(1.0, 2.0);
        assert_ne!(hasher.value(), 0);
    }

    #[test]
    fn test_curve_hasher_hash_segment() {
        let mut hasher = Geom2dHashCurveHasher::new();
        hasher.hash_segment(0.0, 0.0, 1.0, 1.0);
        assert_ne!(hasher.value(), 0);
    }

    #[test]
    fn test_curve_hasher_reset() {
        let mut hasher = Geom2dHashCurveHasher::new();
        hasher.hash_point(5.0, 5.0);
        assert_ne!(hasher.value(), 0);
        hasher.reset();
        assert_eq!(hasher.value(), 0);
    }

    #[test]
    fn test_curve_hasher_consistency() {
        let mut h1 = Geom2dHashCurveHasher::new();
        let mut h2 = Geom2dHashCurveHasher::new();
        h1.hash_point(1.0, 2.0);
        h2.hash_point(1.0, 2.0);
        assert_eq!(h1.value(), h2.value());
    }
}
