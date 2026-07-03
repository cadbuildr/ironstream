// FILE: geom_fill_app_surf.rs
// occt: GeomFill_AppSurf

/// Approximates a BSpline surface through section curves
pub struct AppSurf {
    done: bool,
    dmin: usize,
    dmax: usize,
    tol3d: f64,
    tol2d: f64,
    nbit: usize,
    udeg: usize,
    vdeg: usize,
    knownp: bool,
    tol3dreached: f64,
    tol2dreached: f64,
}

impl AppSurf {
    pub fn new() -> Self {
        AppSurf {
            done: false,
            dmin: 3,
            dmax: 8,
            tol3d: 0.0,
            tol2d: 0.0,
            nbit: 0,
            udeg: 0,
            vdeg: 0,
            knownp: false,
            tol3dreached: 0.0,
            tol2dreached: 0.0,
        }
    }

    pub fn init(&mut self, dmin: usize, dmax: usize, tol3d: f64, tol2d: f64, nbit: usize, known: bool) {
        self.dmin = dmin;
        self.dmax = dmax;
        self.tol3d = tol3d;
        self.tol2d = tol2d;
        self.nbit = nbit;
        self.knownp = known;
        self.done = false;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn u_degree(&self) -> usize {
        self.udeg
    }

    pub fn v_degree(&self) -> usize {
        self.vdeg
    }

    pub fn nb_curves2d(&self) -> usize {
        0
    }

    pub fn curves2d_degree(&self) -> usize {
        0
    }

    pub fn tol_reached(&self) -> (f64, f64) {
        (self.tol3dreached, self.tol2dreached)
    }

    pub fn tol_curve_on_surf(&self, _index: usize) -> f64 {
        0.0
    }
}

impl Default for AppSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = AppSurf::new();
        assert!(!surf.is_done());
    }

    #[test]
    fn test_init() {
        let mut surf = AppSurf::new();
        surf.init(2, 8, 0.01, 0.01, 3, false);
        assert_eq!(surf.dmin, 2);
        assert_eq!(surf.dmax, 8);
        assert_eq!(surf.nbit, 3);
    }

    #[test]
    fn test_degrees() {
        let surf = AppSurf::new();
        assert_eq!(surf.u_degree(), 0);
        assert_eq!(surf.v_degree(), 0);
    }

    #[test]
    fn test_default() {
        let _surf = AppSurf::default();
    }
}
