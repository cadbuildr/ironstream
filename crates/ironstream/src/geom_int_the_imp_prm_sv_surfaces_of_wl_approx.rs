// FILE: geom_int_the_imp_prm_sv_surfaces_of_wl_approx.rs
// occt: GeomInt_TheImpPrmSvSurfacesOfWLApprox

pub struct GeomIntTheImpPrmSvSurfaces {
    u_min: f64,
    u_max: f64,
}

impl GeomIntTheImpPrmSvSurfaces {
    pub fn new() -> Self {
        GeomIntTheImpPrmSvSurfaces {
            u_min: 0.0,
            u_max: 1.0,
        }
    }

    pub fn u_min(&self) -> f64 {
        self.u_min
    }

    pub fn u_max(&self) -> f64 {
        self.u_max
    }
}

impl Default for GeomIntTheImpPrmSvSurfaces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let surf = GeomIntTheImpPrmSvSurfaces::new();
        assert_eq!(surf.u_min(), 0.0);
        assert_eq!(surf.u_max(), 1.0);
    }
}
