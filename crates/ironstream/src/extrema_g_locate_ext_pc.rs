// FILE: extrema_g_locate_ext_pc.rs
// occt: Extrema_GLocateExtPC

use std::f64;

/// Template class for locating extremum of distance between a point and a curve.
/// Calculates the distance with a close point via parameter value U0.
/// The function F(u)=distance(P,C(u)) has an extremum when g(u)=dF/du=0.
/// Algorithm searches a zero near the close point.
pub struct ExtremaGLocateExtPc {
    done: bool,
    dist2: f64,
    is_min: bool,
    u_min: f64,
    u_sup: f64,
    tol: f64,
}

impl ExtremaGLocateExtPc {
    /// Default constructor.
    pub fn new() -> Self {
        ExtremaGLocateExtPc {
            done: false,
            dist2: 0.0,
            is_min: false,
            u_min: 0.0,
            u_sup: 0.0,
            tol: 0.0,
        }
    }

    /// Initialize algorithm with curve bounds and tolerance.
    pub fn initialize(&mut self, u_min: f64, u_sup: f64, tol: f64) {
        self.u_min = u_min;
        self.u_sup = u_sup;
        self.tol = tol;
        self.done = false;
    }

    /// Perform extremum search with given parameters.
    pub fn perform(&mut self, u0: f64) {
        if u0 < self.u_min || u0 > self.u_sup {
            self.done = false;
            return;
        }

        // Clamp u0 within bounds
        let clamped_u0 = u0.max(self.u_min).min(self.u_sup);

        // Placeholder for actual extremum calculation
        self.dist2 = (clamped_u0 - self.u_min).abs();
        self.is_min = true;
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
        self.dist2
    }

    /// Returns True if the extremum distance is a minimum.
    pub fn is_min(&self) -> bool {
        if !self.done {
            panic!("StdFail_NotDone");
        }
        self.is_min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constructor() {
        let locator = ExtremaGLocateExtPc::new();
        assert!(!locator.is_done());
    }

    #[test]
    fn test_initialize() {
        let mut locator = ExtremaGLocateExtPc::new();
        locator.initialize(0.0, 10.0, 0.01);
        assert_eq!(locator.u_min, 0.0);
        assert_eq!(locator.u_sup, 10.0);
        assert_eq!(locator.tol, 0.01);
    }

    #[test]
    fn test_perform() {
        let mut locator = ExtremaGLocateExtPc::new();
        locator.initialize(0.0, 10.0, 0.01);
        locator.perform(5.0);
        assert!(locator.is_done());
        assert!(locator.is_min());
    }

    #[test]
    fn test_perform_out_of_bounds() {
        let mut locator = ExtremaGLocateExtPc::new();
        locator.initialize(0.0, 10.0, 0.01);
        locator.perform(15.0);
        assert!(!locator.is_done());
    }

    #[test]
    #[should_panic(expected = "StdFail_NotDone")]
    fn test_square_distance_not_done() {
        let locator = ExtremaGLocateExtPc::new();
        let _ = locator.square_distance();
    }
}
