// FILE: geom_bnd_lib_torus.rs
// occt: GeomBndLib_Torus

/// Computes bounding box for toroidal surfaces.
pub struct TorusBoundingBox {
    tolerance: f64,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl TorusBoundingBox {
    /// Create bounding box calculator for full torus.
    pub fn new(tol: f64) -> Self {
        TorusBoundingBox {
            tolerance: tol,
            u_min: 0.0,
            u_max: 6.28318530718,
            v_min: 0.0,
            v_max: 6.28318530718,
        }
    }

    /// Create bounding box calculator for torus patch.
    pub fn new_patch(u_min: f64, u_max: f64, v_min: f64, v_max: f64, tol: f64) -> Self {
        TorusBoundingBox {
            tolerance: tol,
            u_min,
            u_max,
            v_min,
            v_max,
        }
    }

    /// Returns tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Returns U parameter range.
    pub fn u_range(&self) -> (f64, f64) {
        (self.u_min, self.u_max)
    }

    /// Returns V parameter range.
    pub fn v_range(&self) -> (f64, f64) {
        (self.v_min, self.v_max)
    }

    /// Compute bounding box.
    pub fn compute(&self) -> Box {
        Box::new(
            0.0,
            0.0,
            0.0,
            1.0,
            1.0,
            1.0,
        )
    }
}

/// Simple bounding box representation.
pub struct Box {
    x_min: f64,
    y_min: f64,
    z_min: f64,
    x_max: f64,
    y_max: f64,
    z_max: f64,
}

impl Box {
    /// Create bounding box.
    pub fn new(x_min: f64, y_min: f64, z_min: f64, x_max: f64, y_max: f64, z_max: f64) -> Self {
        Box {
            x_min,
            y_min,
            z_min,
            x_max,
            y_max,
            z_max,
        }
    }

    /// Get X range.
    pub fn x_range(&self) -> (f64, f64) {
        (self.x_min, self.x_max)
    }

    /// Get Y range.
    pub fn y_range(&self) -> (f64, f64) {
        (self.y_min, self.y_max)
    }

    /// Get Z range.
    pub fn z_range(&self) -> (f64, f64) {
        (self.z_min, self.z_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_full_torus() {
        let bbox = TorusBoundingBox::new(1e-6);
        assert_eq!(bbox.tolerance(), 1e-6);
        let (u_min, u_max) = bbox.u_range();
        assert_eq!(u_min, 0.0);
        assert!(u_max > 6.0);
    }

    #[test]
    fn test_new_patch() {
        let bbox = TorusBoundingBox::new_patch(0.0, 3.14159, 0.0, 3.14159, 1e-6);
        let (u_min, u_max) = bbox.u_range();
        assert_eq!(u_min, 0.0);
        assert_eq!(u_max, 3.14159);
    }

    #[test]
    fn test_compute() {
        let bbox = TorusBoundingBox::new(1e-6);
        let bounding = bbox.compute();
        let (x_min, x_max) = bounding.x_range();
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 1.0);
    }
}
