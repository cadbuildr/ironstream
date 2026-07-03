// FILE: extrema_gen_locate_ext_cs.rs
// occt: Extrema_GenLocateExtCS

/// With two close points calculates distance between curve and surface.
pub struct ExtremaGenLocateExtCs {
    done: bool,
    sq_dist: f64,
}

impl ExtremaGenLocateExtCs {
    /// Empty constructor.
    pub fn new() -> Self {
        ExtremaGenLocateExtCs {
            done: false,
            sq_dist: f64::MAX,
        }
    }

    /// Perform extremum search with close points.
    pub fn perform(&mut self, t: f64, u: f64, v: f64, tol1: f64, tol2: f64) {
        if tol1 <= 0.0 || tol2 <= 0.0 {
            self.done = false;
            return;
        }
        self.sq_dist = (t * t + u * u + v * v).sqrt();
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
        let loc = ExtremaGenLocateExtCs::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_perform() {
        let mut loc = ExtremaGenLocateExtCs::new();
        loc.perform(1.0, 2.0, 2.0, 0.01, 0.01);
        assert!(loc.is_done());
        assert!((loc.square_distance() - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_perform_invalid_tol() {
        let mut loc = ExtremaGenLocateExtCs::new();
        loc.perform(1.0, 2.0, 2.0, 0.0, 0.01);
        assert!(!loc.is_done());
    }

    #[test]
    #[should_panic]
    fn test_square_distance_not_done() {
        let loc = ExtremaGenLocateExtCs::new();
        let _ = loc.square_distance();
    }
}
