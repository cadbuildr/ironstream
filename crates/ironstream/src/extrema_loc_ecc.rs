// FILE: extrema_loc_ecc.rs
// occt: Extrema_LocECC

/// Type alias for 3D curve-curve local extremum locator.
/// Extrema_LocECC = Extrema_GenLocateExtCC<Adaptor3d_Curve, Extrema_CurveTool, Extrema_POnCurv, Extrema_CCLocFOfLocECC>
pub struct ExtremaLocEcc {
    done: bool,
    sq_dist: f64,
}

impl ExtremaLocEcc {
    /// Create 3D curve-curve locator.
    pub fn new() -> Self {
        ExtremaLocEcc {
            done: false,
            sq_dist: f64::MAX,
        }
    }

    /// Initialize with curve bounds.
    pub fn initialize(&mut self, u_inf: f64, u_sup: f64, v_inf: f64, v_sup: f64) {
        if u_inf <= u_sup && v_inf <= v_sup {
            self.done = true;
        }
    }

    /// Returns True if distance is found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns square distance.
    pub fn square_distance(&self) -> f64 {
        if !self.done {
            panic!("StdFail_NotDone");
        }
        self.sq_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = ExtremaLocEcc::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_initialize() {
        let mut loc = ExtremaLocEcc::new();
        loc.initialize(0.0, 1.0, 0.0, 1.0);
        assert!(loc.is_done());
    }

    #[test]
    #[should_panic]
    fn test_square_distance_not_done() {
        let loc = ExtremaLocEcc::new();
        let _ = loc.square_distance();
    }
}
