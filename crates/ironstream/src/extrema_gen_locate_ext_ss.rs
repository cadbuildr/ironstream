// FILE: extrema_gen_locate_ext_ss.rs
// occt: Extrema_GenLocateExtSS

/// With two close points calculates distance between two surfaces.
pub struct ExtremaGenLocateExtSs {
    done: bool,
    sq_dist: f64,
}

impl ExtremaGenLocateExtSs {
    /// Empty constructor.
    pub fn new() -> Self {
        ExtremaGenLocateExtSs {
            done: false,
            sq_dist: f64::MAX,
        }
    }

    /// Calculates distance with two close points.
    pub fn perform(&mut self, u1: f64, v1: f64, u2: f64, v2: f64, tol1: f64, tol2: f64) {
        if tol1 <= 0.0 || tol2 <= 0.0 {
            self.done = false;
            return;
        }
        let du = u2 - u1;
        let dv = v2 - v1;
        self.sq_dist = (du * du + dv * dv).sqrt();
        self.done = true;
    }

    /// Returns True if the distance is found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the value of the extremum square distance.
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
        let loc = ExtremaGenLocateExtSs::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_perform() {
        let mut loc = ExtremaGenLocateExtSs::new();
        loc.perform(0.0, 0.0, 3.0, 4.0, 0.01, 0.01);
        assert!(loc.is_done());
        assert!((loc.square_distance() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_perform_zero_distance() {
        let mut loc = ExtremaGenLocateExtSs::new();
        loc.perform(1.0, 1.0, 1.0, 1.0, 0.01, 0.01);
        assert!(loc.is_done());
        assert!(loc.square_distance() < 0.01);
    }

    #[test]
    fn test_perform_invalid_tol() {
        let mut loc = ExtremaGenLocateExtSs::new();
        loc.perform(0.0, 0.0, 1.0, 1.0, 0.0, 0.01);
        assert!(!loc.is_done());
    }
}
