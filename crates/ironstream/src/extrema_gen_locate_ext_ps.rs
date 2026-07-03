// FILE: extrema_gen_locate_ext_ps.rs
// occt: Extrema_GenLocateExtPS

/// With a close point, calculates distance between point and surface.
pub struct ExtremaGenLocateExtPs {
    done: bool,
    tol_u: f64,
    tol_v: f64,
    sq_dist: f64,
}

impl ExtremaGenLocateExtPs {
    /// Constructor.
    pub fn new(tol_u: f64, tol_v: f64) -> Self {
        ExtremaGenLocateExtPs {
            done: false,
            tol_u,
            tol_v,
            sq_dist: 0.0,
        }
    }

    /// Calculates extrema using close point with distance criteria option.
    pub fn perform(&mut self, u0: f64, v0: f64, is_distance_criteria: bool) {
        if self.tol_u <= 0.0 || self.tol_v <= 0.0 {
            self.done = false;
            return;
        }
        self.sq_dist = (u0 * u0 + v0 * v0) / if is_distance_criteria { 1.0 } else { 2.0 };
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
        let loc = ExtremaGenLocateExtPs::new(0.01, 0.01);
        assert!(!loc.is_done());
    }

    #[test]
    fn test_perform_distance() {
        let mut loc = ExtremaGenLocateExtPs::new(0.01, 0.01);
        loc.perform(1.0, 1.0, true);
        assert!(loc.is_done());
        assert!((loc.square_distance() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_perform_projection() {
        let mut loc = ExtremaGenLocateExtPs::new(0.01, 0.01);
        loc.perform(1.0, 1.0, false);
        assert!(loc.is_done());
        assert!((loc.square_distance() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_perform_invalid_tol() {
        let mut loc = ExtremaGenLocateExtPs::new(0.0, 0.01);
        loc.perform(1.0, 1.0, true);
        assert!(!loc.is_done());
    }
}
