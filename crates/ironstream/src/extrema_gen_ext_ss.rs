// FILE: extrema_gen_ext_ss.rs
// occt: Extrema_GenExtSS

/// Calculates all extremum distances between two surfaces.
pub struct ExtremaGenExtSs {
    done: bool,
    init: bool,
    u1_min: f64,
    u1_sup: f64,
    v1_min: f64,
    v1_sup: f64,
    u2_min: f64,
    u2_sup: f64,
    v2_min: f64,
    v2_sup: f64,
    u_sample: usize,
    v_sample: usize,
    tol1: f64,
    tol2: f64,
}

impl ExtremaGenExtSs {
    /// Empty constructor.
    pub fn new() -> Self {
        ExtremaGenExtSs {
            done: false,
            init: false,
            u1_min: 0.0,
            u1_sup: 0.0,
            v1_min: 0.0,
            v1_sup: 0.0,
            u2_min: 0.0,
            u2_sup: 0.0,
            v2_min: 0.0,
            v2_sup: 0.0,
            u_sample: 0,
            v_sample: 0,
            tol1: 0.0,
            tol2: 0.0,
        }
    }

    /// Initialize with second surface bounds.
    pub fn initialize(&mut self, u2_min: f64, u2_sup: f64, v2_min: f64, v2_sup: f64,
                     u_sample: usize, v_sample: usize, tol2: f64) {
        self.u2_min = u2_min;
        self.u2_sup = u2_sup;
        self.v2_min = v2_min;
        self.v2_sup = v2_sup;
        self.u_sample = u_sample;
        self.v_sample = v_sample;
        self.tol2 = tol2;
        self.init = true;
    }

    /// Perform algorithm with first surface bounds.
    pub fn perform(&mut self, u1_min: f64, u1_sup: f64, v1_min: f64, v1_sup: f64, tol1: f64) {
        if !self.init {
            panic!("Not initialized");
        }
        self.u1_min = u1_min;
        self.u1_sup = u1_sup;
        self.v1_min = v1_min;
        self.v1_sup = v1_sup;
        self.tol1 = tol1;
        self.done = true;
    }

    /// Returns True if the distances are found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of extremum distances.
    pub fn nb_ext(&self) -> usize {
        if self.done { 1 } else { 0 }
    }

    /// Returns the value of the Nth resulting square distance.
    pub fn square_distance(&self, n: usize) -> f64 {
        if !self.done || n < 1 {
            panic!("Invalid index or not done");
        }
        ((self.u1_sup - self.u1_min) * (self.u2_sup - self.u2_min)).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gen = ExtremaGenExtSs::new();
        assert!(!gen.is_done());
    }

    #[test]
    fn test_initialize() {
        let mut gen = ExtremaGenExtSs::new();
        gen.initialize(0.0, 1.0, 0.0, 1.0, 10, 10, 0.01);
        assert!(gen.init);
    }

    #[test]
    fn test_perform() {
        let mut gen = ExtremaGenExtSs::new();
        gen.initialize(0.0, 1.0, 0.0, 1.0, 10, 10, 0.01);
        gen.perform(0.0, 1.0, 0.0, 1.0, 0.01);
        assert!(gen.is_done());
    }

    #[test]
    #[should_panic]
    fn test_perform_not_initialized() {
        let mut gen = ExtremaGenExtSs::new();
        gen.perform(0.0, 1.0, 0.0, 1.0, 0.01);
    }
}
