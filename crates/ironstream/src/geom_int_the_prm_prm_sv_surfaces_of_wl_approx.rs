// FILE: geom_int_the_prm_prm_sv_surfaces_of_wl_approx.rs
// occt: GeomInt_ThePrmPrmSvSurfacesOfWLApprox

pub struct GeomIntThePrmPrmSvSurfaces {
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl GeomIntThePrmPrmSvSurfaces {
    pub fn new() -> Self {
        GeomIntThePrmPrmSvSurfaces {
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0,
        }
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (self.u_min, self.u_max, self.v_min, self.v_max)
    }
}

impl Default for GeomIntThePrmPrmSvSurfaces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let surf = GeomIntThePrmPrmSvSurfaces::new();
        let (u_min, u_max, v_min, v_max) = surf.bounds();
        assert_eq!(u_min, 0.0);
        assert_eq!(u_max, 1.0);
        assert_eq!(v_min, 0.0);
        assert_eq!(v_max, 1.0);
    }
}
