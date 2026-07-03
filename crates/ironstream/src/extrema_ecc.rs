// FILE: extrema_ecc.rs
// occt: Extrema_ECC

/// Curve-curve extrema solver in 3D.
pub struct ExtremaEcc {
    done: bool,
    nb_extrema: i32,
    extrema: Vec<(f64, f64, f64)>,  // (u1, u2, distance_sq)
}

impl ExtremaEcc {
    /// Creates a new solver.
    pub fn new() -> Self {
        ExtremaEcc {
            done: false,
            nb_extrema: 0,
            extrema: Vec::new(),
        }
    }

    /// Indicates if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns number of extrema.
    pub fn nb_extrema(&self) -> i32 {
        self.nb_extrema
    }

    /// Returns u1 parameter of i-th extremum.
    pub fn u1_extremum(&self, i: usize) -> Option<f64> {
        self.extrema.get(i).map(|e| e.0)
    }

    /// Returns u2 parameter of i-th extremum.
    pub fn u2_extremum(&self, i: usize) -> Option<f64> {
        self.extrema.get(i).map(|e| e.1)
    }

    /// Returns distance of i-th extremum.
    pub fn distance_extremum(&self, i: usize) -> Option<f64> {
        self.extrema.get(i).map(|e| e.2.sqrt())
    }

    /// Adds an extremum.
    pub fn add_extremum(&mut self, u1: f64, u2: f64, dist_sq: f64) {
        self.extrema.push((u1, u2, dist_sq));
        self.nb_extrema = self.extrema.len() as i32;
    }

    /// Sets done flag.
    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    /// Clears extrema.
    pub fn clear(&mut self) {
        self.extrema.clear();
        self.nb_extrema = 0;
    }
}

impl Default for ExtremaEcc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ecc = ExtremaEcc::new();
        assert!(!ecc.is_done());
        assert_eq!(ecc.nb_extrema(), 0);
    }

    #[test]
    fn test_add_extremum() {
        let mut ecc = ExtremaEcc::new();
        ecc.add_extremum(0.3, 0.7, 4.0);
        assert_eq!(ecc.nb_extrema(), 1);
        assert_eq!(ecc.u1_extremum(0), Some(0.3));
        assert_eq!(ecc.u2_extremum(0), Some(0.7));
        assert_eq!(ecc.distance_extremum(0), Some(2.0));
    }

    #[test]
    fn test_multiple_extrema() {
        let mut ecc = ExtremaEcc::new();
        ecc.add_extremum(0.2, 0.6, 1.0);
        ecc.add_extremum(0.5, 0.9, 9.0);
        assert_eq!(ecc.nb_extrema(), 2);
        assert_eq!(ecc.u1_extremum(1), Some(0.5));
    }

    #[test]
    fn test_clear() {
        let mut ecc = ExtremaEcc::new();
        ecc.add_extremum(0.5, 0.5, 1.0);
        ecc.clear();
        assert_eq!(ecc.nb_extrema(), 0);
    }
}
