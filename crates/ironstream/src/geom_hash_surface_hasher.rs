// FILE: geom_hash_surface_hasher.rs
// occt: GeomHash_SurfaceHasher

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct GeomHashSurfaceHasher;

impl GeomHashSurfaceHasher {
    /// Compute hash of a surface.
    pub fn hash_surface(surface_id: u32, precision: f64) -> u64 {
        let mut hasher = DefaultHasher::new();
        surface_id.hash(&mut hasher);
        precision.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    /// Compute hash of surface with tolerances.
    pub fn hash_surface_with_tolerances(
        surface_id: u32,
        precision: f64,
        tol_u: f64,
        tol_v: f64,
    ) -> u64 {
        let mut hasher = DefaultHasher::new();
        surface_id.hash(&mut hasher);
        precision.to_bits().hash(&mut hasher);
        tol_u.to_bits().hash(&mut hasher);
        tol_v.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_surface() {
        let h1 = GeomHashSurfaceHasher::hash_surface(1, 1e-7);
        let h2 = GeomHashSurfaceHasher::hash_surface(1, 1e-7);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_surface_different_ids() {
        let h1 = GeomHashSurfaceHasher::hash_surface(1, 1e-7);
        let h2 = GeomHashSurfaceHasher::hash_surface(2, 1e-7);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_surface_with_tolerances() {
        let h1 = GeomHashSurfaceHasher::hash_surface_with_tolerances(1, 1e-7, 1e-5, 1e-5);
        let h2 = GeomHashSurfaceHasher::hash_surface_with_tolerances(1, 1e-7, 1e-5, 1e-5);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_surface_different_tol_u() {
        let h1 = GeomHashSurfaceHasher::hash_surface_with_tolerances(1, 1e-7, 1e-5, 1e-5);
        let h2 = GeomHashSurfaceHasher::hash_surface_with_tolerances(1, 1e-7, 1e-4, 1e-5);
        assert_ne!(h1, h2);
    }
}
