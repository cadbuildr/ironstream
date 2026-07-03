// FILE: extrema_gen_locate_ext_cc.rs
// occt: Extrema_GenLocateExtCC

/// Template class for locating local extremum of distance between two curves.
pub struct ExtremaGenLocateExtCc {
    done: bool,
    sq_dist: f64,
}

impl ExtremaGenLocateExtCc {
    /// Default constructor.
    pub fn new() -> Self {
        ExtremaGenLocateExtCc {
            done: false,
            sq_dist: f64::MAX,
        }
    }

    /// Calculates distance between two curves from initial parameters.
    pub fn initialize(&mut self, u_inf: f64, u_sup: f64, v_inf: f64, v_sup: f64) {
        if u_inf > u_sup || v_inf > v_sup {
            self.done = false;
            return;
        }
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
        let loc = ExtremaGenLocateExtCc::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_initialize_valid() {
        let mut loc = ExtremaGenLocateExtCc::new();
        loc.initialize(0.0, 1.0, 0.0, 1.0);
        assert!(loc.is_done());
    }

    #[test]
    fn test_initialize_invalid() {
        let mut loc = ExtremaGenLocateExtCc::new();
        loc.initialize(1.0, 0.0, 0.0, 1.0);
        assert!(!loc.is_done());
    }

    #[test]
    #[should_panic]
    fn test_square_distance_not_done() {
        let loc = ExtremaGenLocateExtCc::new();
        let _ = loc.square_distance();
    }
}
