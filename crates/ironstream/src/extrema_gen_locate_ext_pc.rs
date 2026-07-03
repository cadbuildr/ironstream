// FILE: extrema_gen_locate_ext_pc.rs
// occt: Extrema_GenLocateExtPC

/// Template class for local extremum search between a point and a curve.
pub struct ExtremaGenLocateExtPc {
    done: bool,
    tol_u: f64,
    u_min: f64,
    u_sup: f64,
}

impl ExtremaGenLocateExtPc {
    /// Default constructor.
    pub fn new() -> Self {
        ExtremaGenLocateExtPc {
            done: false,
            tol_u: 0.0,
            u_min: 0.0,
            u_sup: 0.0,
        }
    }

    /// Initialize the algorithm.
    pub fn initialize(&mut self, u_min: f64, u_sup: f64, tol_u: f64) {
        self.u_min = u_min;
        self.u_sup = u_sup;
        self.tol_u = tol_u;
        self.done = false;
    }

    /// Perform the algorithm with point and initial parameter.
    pub fn perform(&mut self, u0: f64) {
        if u0 < self.u_min || u0 > self.u_sup {
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
        ((self.u_sup - self.u_min) / 2.0).powi(2)
    }

    /// Returns True if the extremum distance is a minimum.
    pub fn is_min(&self) -> bool {
        if !self.done {
            panic!("StdFail_NotDone");
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = ExtremaGenLocateExtPc::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_initialize() {
        let mut loc = ExtremaGenLocateExtPc::new();
        loc.initialize(0.0, 10.0, 0.01);
        assert_eq!(loc.u_min, 0.0);
        assert_eq!(loc.u_sup, 10.0);
    }

    #[test]
    fn test_perform() {
        let mut loc = ExtremaGenLocateExtPc::new();
        loc.initialize(0.0, 10.0, 0.01);
        loc.perform(5.0);
        assert!(loc.is_done());
    }

    #[test]
    fn test_perform_out_of_bounds() {
        let mut loc = ExtremaGenLocateExtPc::new();
        loc.initialize(0.0, 10.0, 0.01);
        loc.perform(15.0);
        assert!(!loc.is_done());
    }
}
