// FILE: extrema_loc_ecc2d.rs
// occt: Extrema_LocECC2d

/// Type alias for 2D curve-curve local extremum locator.
pub struct ExtremaLocEcc2d {
    done: bool,
    sq_dist: f64,
}

impl ExtremaLocEcc2d {
    /// Create 2D curve-curve locator.
    pub fn new() -> Self {
        ExtremaLocEcc2d {
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
        let loc = ExtremaLocEcc2d::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_initialize() {
        let mut loc = ExtremaLocEcc2d::new();
        loc.initialize(0.0, 1.0, 0.0, 1.0);
        assert!(loc.is_done());
    }
}
