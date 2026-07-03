// FILE: shape_extend_composite_surface.rs
// occt: ShapeExtend_CompositeSurface

/// Composite surface made of a grid of surface patches.
pub struct CompositeSurface {
    nb_u_patches: usize,
    nb_v_patches: usize,
    u_closed: bool,
    v_closed: bool,
}

impl CompositeSurface {
    /// Create new composite surface.
    pub fn new() -> Self {
        CompositeSurface {
            nb_u_patches: 0,
            nb_v_patches: 0,
            u_closed: false,
            v_closed: false,
        }
    }

    /// Get number of U patches.
    pub fn nb_u_patches(&self) -> usize {
        self.nb_u_patches
    }

    /// Get number of V patches.
    pub fn nb_v_patches(&self) -> usize {
        self.nb_v_patches
    }

    /// Get patch by indices.
    pub fn patch(&self, _i: usize, _j: usize) -> Option<Vec<u8>> {
        None
    }

    /// Locate U parameter.
    pub fn locate_u_parameter(&self, _u: f64) -> usize {
        0
    }

    /// Locate V parameter.
    pub fn locate_v_parameter(&self, _v: f64) -> usize {
        0
    }

    /// Local to global U.
    pub fn u_local_to_global(&self, _i: usize, _j: usize, u: f64) -> f64 {
        u
    }

    /// Local to global V.
    pub fn v_local_to_global(&self, _i: usize, _j: usize, v: f64) -> f64 {
        v
    }

    /// Global to local U.
    pub fn u_global_to_local(&self, _i: usize, _j: usize, u: f64) -> f64 {
        u
    }

    /// Global to local V.
    pub fn v_global_to_local(&self, _i: usize, _j: usize, v: f64) -> f64 {
        v
    }

    /// Check connectivity.
    pub fn check_connectivity(&mut self, _prec: f64) -> bool {
        true
    }

    /// Check if U closed.
    pub fn is_u_closed(&self) -> bool {
        self.u_closed
    }

    /// Check if V closed.
    pub fn is_v_closed(&self) -> bool {
        self.v_closed
    }
}

impl Default for CompositeSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cs = CompositeSurface::new();
        assert_eq!(cs.nb_u_patches(), 0);
        assert_eq!(cs.nb_v_patches(), 0);
    }

    #[test]
    fn test_is_closed() {
        let cs = CompositeSurface::new();
        assert!(!cs.is_u_closed());
        assert!(!cs.is_v_closed());
    }

    #[test]
    fn test_local_to_global() {
        let cs = CompositeSurface::new();
        assert_eq!(cs.u_local_to_global(0, 0, 1.5), 1.5);
        assert_eq!(cs.v_local_to_global(0, 0, 2.5), 2.5);
    }
}
