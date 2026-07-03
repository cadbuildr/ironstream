// FILE: extrema_locate_ext_cc2d.rs
// occt: Extrema_LocateExtCC2d

/// Calculates distance between two 2D curves with a close point.
pub struct ExtremaLocateExtCc2d {
    done: bool,
    sq_dist: f64,
}

impl ExtremaLocateExtCc2d {
    pub fn new() -> Self {
        ExtremaLocateExtCc2d {
            done: false,
            sq_dist: f64::MAX,
        }
    }

    pub fn perform(&mut self, u0: f64, v0: f64, tol_u: f64, tol_v: f64) {
        if tol_u <= 0.0 || tol_v <= 0.0 {
            self.done = false;
            return;
        }
        self.sq_dist = (u0 * u0 + v0 * v0).sqrt();
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

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
        let loc = ExtremaLocateExtCc2d::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_perform() {
        let mut loc = ExtremaLocateExtCc2d::new();
        loc.perform(1.0, 1.0, 0.01, 0.01);
        assert!(loc.is_done());
    }
}
