// FILE: extrema_pc.rs
// occt: ExtremaPC

/// Point-to-curve extrema solver.
pub struct ExtremaPC {
    done: bool,
    nb_extrema: i32,
    u_extrema: Vec<f64>,
    dist_extrema: Vec<f64>,
}

impl ExtremaPC {
    /// Creates a new point-curve extrema solver.
    pub fn new() -> Self {
        ExtremaPC {
            done: false,
            nb_extrema: 0,
            u_extrema: Vec::new(),
            dist_extrema: Vec::new(),
        }
    }

    /// Indicates if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns number of extrema found.
    pub fn nb_extrema(&self) -> i32 {
        self.nb_extrema
    }

    /// Returns U parameter of i-th extremum (0-indexed).
    pub fn u_extremum(&self, i: usize) -> Option<f64> {
        if i < self.u_extrema.len() {
            Some(self.u_extrema[i])
        } else {
            None
        }
    }

    /// Returns distance at i-th extremum.
    pub fn distance_extremum(&self, i: usize) -> Option<f64> {
        if i < self.dist_extrema.len() {
            Some(self.dist_extrema[i])
        } else {
            None
        }
    }

    /// Returns minimum distance.
    pub fn min_distance(&self) -> f64 {
        self.dist_extrema.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Sets done flag.
    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    /// Adds an extremum.
    pub fn add_extremum(&mut self, u: f64, dist: f64) {
        self.u_extrema.push(u);
        self.dist_extrema.push(dist);
        self.nb_extrema = self.u_extrema.len() as i32;
    }

    /// Clears all extrema.
    pub fn clear(&mut self) {
        self.u_extrema.clear();
        self.dist_extrema.clear();
        self.nb_extrema = 0;
    }
}

impl Default for ExtremaPC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext = ExtremaPC::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_extrema(), 0);
        assert!(ext.min_distance().is_infinite());
    }

    #[test]
    fn test_add_extremum() {
        let mut ext = ExtremaPC::new();
        ext.add_extremum(0.5, 1.5);
        ext.add_extremum(0.7, 2.0);
        assert_eq!(ext.nb_extrema(), 2);
        assert_eq!(ext.u_extremum(0), Some(0.5));
        assert_eq!(ext.distance_extremum(1), Some(2.0));
    }

    #[test]
    fn test_min_distance() {
        let mut ext = ExtremaPC::new();
        ext.add_extremum(0.3, 3.0);
        ext.add_extremum(0.5, 1.0);
        ext.add_extremum(0.8, 2.5);
        assert_eq!(ext.min_distance(), 1.0);
    }

    #[test]
    fn test_clear() {
        let mut ext = ExtremaPC::new();
        ext.add_extremum(0.5, 1.5);
        assert_eq!(ext.nb_extrema(), 1);
        ext.clear();
        assert_eq!(ext.nb_extrema(), 0);
    }
}
