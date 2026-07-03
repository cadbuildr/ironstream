// FILE: geom_hash_polygon3_d_hasher.rs
// occt: GeomHash_Polygon3DHasher

//! Hash function for 3D polygons.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Polygon3DHasher;

impl Polygon3DHasher {
    /// Compute a hash value for a 3D polygon.
    pub fn hash_polygon(points: &[(f64, f64, f64)]) -> u64 {
        let mut hasher = DefaultHasher::new();
        for &(x, y, z) in points {
            x.to_bits().hash(&mut hasher);
            y.to_bits().hash(&mut hasher);
            z.to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Check if two polygons are equal by hash.
    pub fn polygons_equal(points1: &[(f64, f64, f64)], points2: &[(f64, f64, f64)]) -> bool {
        if points1.len() != points2.len() {
            return false;
        }
        Self::hash_polygon(points1) == Self::hash_polygon(points2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty() {
        let hash1 = Polygon3DHasher::hash_polygon(&[]);
        let hash2 = Polygon3DHasher::hash_polygon(&[]);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_single_point() {
        let points = [(1.0, 2.0, 3.0)];
        let hash = Polygon3DHasher::hash_polygon(&points);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_hash_triangle() {
        let points = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
        let hash = Polygon3DHasher::hash_polygon(&points);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_polygons_equal() {
        let p1 = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let p2 = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        assert!(Polygon3DHasher::polygons_equal(&p1, &p2));
    }

    #[test]
    fn test_polygons_different_length() {
        let p1 = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let p2 = [(0.0, 0.0, 0.0)];
        assert!(!Polygon3DHasher::polygons_equal(&p1, &p2));
    }
}
