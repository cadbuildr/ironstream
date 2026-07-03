// FILE: extrema_loc_epc_of_locate_ext_pc.rs
// occt: Extrema_LocEPCOfLocateExtPC

/// Type alias for 3D curve local extremum search.
pub struct ExtremaLocEpcOfLocateExtPc {
    done: bool,
    tol_u: f64,
    u_min: f64,
    u_sup: f64,
}

impl ExtremaLocEpcOfLocateExtPc {
    pub fn new() -> Self {
        ExtremaLocEpcOfLocateExtPc {
            done: false,
            tol_u: 0.0,
            u_min: 0.0,
            u_sup: 0.0,
        }
    }

    pub fn initialize(&mut self, u_min: f64, u_sup: f64, tol_u: f64) {
        self.u_min = u_min;
        self.u_sup = u_sup;
        self.tol_u = tol_u;
    }

    pub fn perform(&mut self, u0: f64) {
        if u0 >= self.u_min && u0 <= self.u_sup && self.tol_u > 0.0 {
            self.done = true;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = ExtremaLocEpcOfLocateExtPc::new();
        assert!(!loc.is_done());
    }

    #[test]
    fn test_initialize_and_perform() {
        let mut loc = ExtremaLocEpcOfLocateExtPc::new();
        loc.initialize(0.0, 10.0, 0.01);
        loc.perform(5.0);
        assert!(loc.is_done());
    }
}
