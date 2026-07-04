// FILE: top_ope_b_rep_ds_surface.rs
// occt: TopOpeBRepDS_Surface

/// A geometric surface with an associated tolerance.
/// Represents a surface from Geom_Surface with tolerance for operations.
pub struct TopOpeBRepDSSurface {
    /// Surface tolerance value
    tolerance: f64,
    /// Flag indicating if surface should be kept in the data structure
    keep: bool,
}

impl TopOpeBRepDSSurface {
    /// Create a new empty surface
    pub fn new() -> Self {
        TopOpeBRepDSSurface {
            tolerance: 0.0,
            keep: true,
        }
    }

    /// Create a surface with tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        TopOpeBRepDSSurface {
            tolerance,
            keep: true,
        }
    }

    /// Create a surface as a copy
    pub fn from_other(other: &TopOpeBRepDSSurface) -> Self {
        TopOpeBRepDSSurface {
            tolerance: other.tolerance,
            keep: other.keep,
        }
    }

    /// Assign from another surface
    pub fn assign(&mut self, other: &TopOpeBRepDSSurface) {
        self.tolerance = other.tolerance;
        self.keep = other.keep;
    }

    /// Get the tolerance value
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Update the tolerance value
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Check if surface should be kept
    pub fn keep(&self) -> bool {
        self.keep
    }

    /// Set whether surface should be kept
    pub fn set_keep(&mut self, keep: bool) {
        self.keep = keep;
    }
}

impl Default for TopOpeBRepDSSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopOpeBRepDSSurface {
    fn clone(&self) -> Self {
        Self::from_other(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        let surface = TopOpeBRepDSSurface::new();
        assert_eq!(surface.tolerance(), 0.0);
        assert!(surface.keep());
    }

    #[test]
    fn test_surface_with_tolerance() {
        let surface = TopOpeBRepDSSurface::with_tolerance(0.001);
        assert_eq!(surface.tolerance(), 0.001);
        assert!(surface.keep());
    }

    #[test]
    fn test_set_tolerance() {
        let mut surface = TopOpeBRepDSSurface::new();
        surface.set_tolerance(0.01);
        assert_eq!(surface.tolerance(), 0.01);
    }

    #[test]
    fn test_keep_flag() {
        let mut surface = TopOpeBRepDSSurface::new();
        assert!(surface.keep());
        surface.set_keep(false);
        assert!(!surface.keep());
    }

    #[test]
    fn test_assign() {
        let mut surface1 = TopOpeBRepDSSurface::with_tolerance(0.01);
        surface1.set_keep(false);

        let mut surface2 = TopOpeBRepDSSurface::new();
        surface2.assign(&surface1);

        assert_eq!(surface2.tolerance(), 0.01);
        assert!(!surface2.keep());
    }

    #[test]
    fn test_clone() {
        let surface1 = TopOpeBRepDSSurface::with_tolerance(0.005);
        let surface2 = surface1.clone();
        assert_eq!(surface2.tolerance(), 0.005);
    }

    #[test]
    fn test_default() {
        let surface = TopOpeBRepDSSurface::default();
        assert_eq!(surface.tolerance(), 0.0);
        assert!(surface.keep());
    }
}
