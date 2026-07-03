// FILE: extrema_ecc2d.rs
// occt: Extrema_ECC2d

/// Curve-curve extrema solver in 2D.
pub struct ExtremaEcc2d {
    done: bool,
    nb_extrema: i32,
    extrema: Vec<(f64, f64, f64)>,  // (u1, u2, distance_sq)
}

impl ExtremaEcc2d {
    /// Creates a new 2D solver.
    pub fn new() -> Self {
        ExtremaEcc2d {
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

impl Default for ExtremaEcc2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ecc2d = ExtremaEcc2d::new();
        assert!(!ecc2d.is_done());
        assert_eq!(ecc2d.nb_extrema(), 0);
    }

    #[test]
    fn test_add_extremum() {
        let mut ecc2d = ExtremaEcc2d::new();
        ecc2d.add_extremum(0.4, 0.6, 16.0);
        assert_eq!(ecc2d.nb_extrema(), 1);
        assert_eq!(ecc2d.u1_extremum(0), Some(0.4));
        assert_eq!(ecc2d.u2_extremum(0), Some(0.6));
        assert_eq!(ecc2d.distance_extremum(0), Some(4.0));
    }

    #[test]
    fn test_done_flag() {
        let mut ecc2d = ExtremaEcc2d::new();
        ecc2d.set_done(true);
        assert!(ecc2d.is_done());
    }
}
