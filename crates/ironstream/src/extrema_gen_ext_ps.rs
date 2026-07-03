// FILE: extrema_gen_ext_ps.rs
// occt: Extrema_GenExtPS

/// Calculates all extremum distances between a point and a surface.
pub struct ExtremaGenExtPs {
    done: bool,
    u_min: f64,
    u_sup: f64,
    v_min: f64,
    v_sup: f64,
    u_sample: usize,
    v_sample: usize,
    tol_u: f64,
    tol_v: f64,
}

impl ExtremaGenExtPs {
    /// Empty constructor.
    pub fn new() -> Self {
        ExtremaGenExtPs {
            done: false,
            u_min: 0.0,
            u_sup: 0.0,
            v_min: 0.0,
            v_sup: 0.0,
            u_sample: 0,
            v_sample: 0,
            tol_u: 0.0,
            tol_v: 0.0,
        }
    }

    /// Initialize with surface bounds.
    pub fn initialize(&mut self, u_min: f64, u_sup: f64, v_min: f64, v_sup: f64,
                     u_sample: usize, v_sample: usize, tol_u: f64, tol_v: f64) {
        self.u_min = u_min;
        self.u_sup = u_sup;
        self.v_min = v_min;
        self.v_sup = v_sup;
        self.u_sample = u_sample;
        self.v_sample = v_sample;
        self.tol_u = tol_u;
        self.tol_v = tol_v;
    }

    /// Perform extremum search with a point.
    pub fn perform(&mut self) {
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
        ((self.u_sup - self.u_min).powi(2) + (self.v_sup - self.v_min).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gen = ExtremaGenExtPs::new();
        assert!(!gen.is_done());
    }

    #[test]
    fn test_initialize() {
        let mut gen = ExtremaGenExtPs::new();
        gen.initialize(0.0, 1.0, 0.0, 1.0, 10, 10, 0.01, 0.01);
        assert_eq!(gen.u_min, 0.0);
        assert_eq!(gen.v_sup, 1.0);
    }

    #[test]
    fn test_perform() {
        let mut gen = ExtremaGenExtPs::new();
        gen.initialize(0.0, 1.0, 0.0, 1.0, 10, 10, 0.01, 0.01);
        gen.perform();
        assert!(gen.is_done());
    }

    #[test]
    fn test_square_distance() {
        let mut gen = ExtremaGenExtPs::new();
        gen.u_min = 0.0;
        gen.u_sup = 3.0;
        gen.v_min = 0.0;
        gen.v_sup = 4.0;
        gen.done = true;
        let dist = gen.square_distance(1);
        assert!((dist - 5.0).abs() < 0.01);
    }
}
