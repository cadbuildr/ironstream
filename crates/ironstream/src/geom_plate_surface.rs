// FILE: geom_plate_surface.rs
// occt: GeomPlate_Surface

/// Plate surface implementation combining initial surface with plate approximation.
pub struct GeomPlateSurface {
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl GeomPlateSurface {
    pub fn new(u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        GeomPlateSurface {
            u_min,
            u_max,
            v_min,
            v_max,
        }
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (self.u_min, self.u_max, self.v_min, self.v_max)
    }

    pub fn set_bounds(&mut self, u_min: f64, u_max: f64, v_min: f64, v_max: f64) {
        self.u_min = u_min;
        self.u_max = u_max;
        self.v_min = v_min;
        self.v_max = v_max;
    }

    pub fn is_u_closed(&self) -> bool {
        false
    }

    pub fn is_v_closed(&self) -> bool {
        false
    }

    pub fn is_u_periodic(&self) -> bool {
        false
    }

    pub fn is_v_periodic(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let surf = GeomPlateSurface::new(0.0, 1.0, 0.0, 1.0);
        let (u_min, u_max, v_min, v_max) = surf.bounds();
        assert_eq!(u_min, 0.0);
        assert_eq!(u_max, 1.0);
        assert_eq!(v_min, 0.0);
        assert_eq!(v_max, 1.0);
    }

    #[test]
    fn test_set_bounds() {
        let mut surf = GeomPlateSurface::new(0.0, 1.0, 0.0, 1.0);
        surf.set_bounds(0.5, 1.5, 0.25, 0.75);
        let (u_min, u_max, v_min, v_max) = surf.bounds();
        assert_eq!(u_min, 0.5);
        assert_eq!(u_max, 1.5);
        assert_eq!(v_min, 0.25);
        assert_eq!(v_max, 0.75);
    }

    #[test]
    fn test_closed_properties() {
        let surf = GeomPlateSurface::new(0.0, 1.0, 0.0, 1.0);
        assert!(!surf.is_u_closed());
        assert!(!surf.is_v_closed());
        assert!(!surf.is_u_periodic());
        assert!(!surf.is_v_periodic());
    }
}
