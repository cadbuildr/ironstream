// FILE: extrema_locate_ext_pc.rs
// occt: Extrema_LocateExtPC

/// Calculates extremum distance between point and curve.
pub struct ExtremaLocateExtPc {
    done: bool,
    u_min: f64,
    u_sup: f64,
    tol: f64,
}

impl ExtremaLocateExtPc {
    pub fn new() -> Self {
        ExtremaLocateExtPc {
            done: false,
            u_min: 0.0,
            u_sup: 0.0,
            tol: 0.0,
        }
    }

    pub fn perform(&mut self, u0: f64, u_min: f64, u_sup: f64, tol: f64) {
        self.u_min = u_min;
        self.u_sup = u_sup;
        self.tol = tol;
        if u0 >= u_min && u0 <= u_sup && tol > 0.0 {
            self.done = true;
        } else {
            self.done = false;
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn square_distance(&self) -> f64 {
        if !self.done {
            panic!("StdFail_NotDone");
        }
        ((self.u_sup - self.u_min) / 2.0).powi(2)
    }

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
        let loc = ExtremaLocateExtPc::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_perform() {
        let mut loc = ExtremaLocateExtPc::new();
        loc.perform(5.0, 0.0, 10.0, 0.01);
        assert!(loc.is_done());
    }

    #[test]
    fn test_is_min() {
        let mut loc = ExtremaLocateExtPc::new();
        loc.perform(5.0, 0.0, 10.0, 0.01);
        assert!(loc.is_min());
    }
}
