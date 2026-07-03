// FILE: geom_hash_triangulation_hasher.rs
// occt: GeomHash_TriangulationHasher

//! Hash function for triangulation meshes.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct TriangulationHasher;

impl TriangulationHasher {
    /// Compute a hash value for a triangulation mesh.
    pub fn hash_triangulation(vertices: &[(f64, f64, f64)], triangles: &[(usize, usize, usize)]) -> u64 {
        let mut hasher = DefaultHasher::new();

        // Hash vertices
        for &(x, y, z) in vertices {
            x.to_bits().hash(&mut hasher);
            y.to_bits().hash(&mut hasher);
            z.to_bits().hash(&mut hasher);
        }

        // Hash triangle indices
        for &(i, j, k) in triangles {
            i.hash(&mut hasher);
            j.hash(&mut hasher);
            k.hash(&mut hasher);
        }

        hasher.finish()
    }

    /// Check if two triangulations are equal by hash.
    pub fn triangulations_equal(
        vertices1: &[(f64, f64, f64)],
        triangles1: &[(usize, usize, usize)],
        vertices2: &[(f64, f64, f64)],
        triangles2: &[(usize, usize, usize)],
    ) -> bool {
        if vertices1.len() != vertices2.len() || triangles1.len() != triangles2.len() {
            return false;
        }
        Self::hash_triangulation(vertices1, triangles1)
            == Self::hash_triangulation(vertices2, triangles2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_triangulation() {
        let hash = TriangulationHasher::hash_triangulation(&[], &[]);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_single_triangle() {
        let vertices = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
        let triangles = [(0, 1, 2)];
        let hash = TriangulationHasher::hash_triangulation(&vertices, &triangles);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_multiple_triangles() {
        let vertices = [
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
        ];
        let triangles = [(0, 1, 2), (1, 3, 2)];
        let hash = TriangulationHasher::hash_triangulation(&vertices, &triangles);
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_equal_triangulations() {
        let vertices = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let triangles = [(0, 1, 0)];
        assert!(TriangulationHasher::triangulations_equal(
            &vertices, &triangles, &vertices, &triangles
        ));
    }

    #[test]
    fn test_different_vertex_count() {
        let v1 = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let v2 = [(0.0, 0.0, 0.0)];
        let tri = [(0, 1, 0)];
        assert!(!TriangulationHasher::triangulations_equal(&v1, &tri, &v2, &tri));
    }
}
