// FILE: geom_hash_polygon_on_tri_hasher.rs
// occt: GeomHash_PolygonOnTriHasher

//! Hash function for polygons projected on triangles.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct PolygonOnTriHasher;

impl PolygonOnTriHasher {
    /// Compute a hash value for a polygon on a triangle.
    /// Combines polygon points and triangle vertices.
    pub fn hash_polygon_on_tri(
        polygon_points: &[(f64, f64)],
        triangle: &[(f64, f64, f64); 3],
    ) -> u64 {
        let mut hasher = DefaultHasher::new();

        // Hash polygon points
        for &(x, y) in polygon_points {
            x.to_bits().hash(&mut hasher);
            y.to_bits().hash(&mut hasher);
        }

        // Hash triangle vertices
        for (x, y, z) in triangle {
            x.to_bits().hash(&mut hasher);
            y.to_bits().hash(&mut hasher);
            z.to_bits().hash(&mut hasher);
        }

        hasher.finish()
    }

    /// Check equality between two polygon-on-tri configurations.
    pub fn equal(
        poly1_points: &[(f64, f64)],
        tri1: &[(f64, f64, f64); 3],
        poly2_points: &[(f64, f64)],
        tri2: &[(f64, f64, f64); 3],
    ) -> bool {
        if poly1_points.len() != poly2_points.len() {
            return false;
        }
        Self::hash_polygon_on_tri(poly1_points, tri1)
            == Self::hash_polygon_on_tri(poly2_points, tri2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty_polygon() {
        let tri = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
        let hash = PolygonOnTriHasher::hash_polygon_on_tri(&[], &tri);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_hash_single_point() {
        let polygon = [(0.5, 0.5)];
        let tri = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
        let hash = PolygonOnTriHasher::hash_polygon_on_tri(&polygon, &tri);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_equal() {
        let poly = [(0.5, 0.5)];
        let tri = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
        assert!(PolygonOnTriHasher::equal(&poly, &tri, &poly, &tri));
    }

    #[test]
    fn test_not_equal_different_polygon() {
        let poly1 = [(0.5, 0.5)];
        let poly2 = [(0.3, 0.3)];
        let tri = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
        assert!(!PolygonOnTriHasher::equal(&poly1, &tri, &poly2, &tri));
    }
}
