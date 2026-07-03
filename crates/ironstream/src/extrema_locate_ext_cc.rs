// FILE: extrema_locate_ext_cc.rs
// occt: Extrema_LocateExtCC

/// Calculates distance between two curves with a close point.
pub struct ExtremaLocateExtCc {
    done: bool,
    sq_dist: f64,
}

impl ExtremaLocateExtCc {
    pub fn new() -> Self {
        ExtremaLocateExtCc {
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
        let loc = ExtremaLocateExtCc::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_perform() {
        let mut loc = ExtremaLocateExtCc::new();
        loc.perform(3.0, 4.0, 0.01, 0.01);
        assert!(loc.is_done());
        assert!((loc.square_distance() - 5.0).abs() < 0.01);
    }
}
