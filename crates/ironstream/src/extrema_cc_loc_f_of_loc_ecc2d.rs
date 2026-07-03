// FILE: extrema_cc_loc_f_of_loc_ecc2d.rs
// occt: Extrema_CCLocFOfLocECC2d

/// Function class for curve-curve local extrema in 2D.
pub struct ExtremaCcLocFOfLocEcc2d {
    is_init: bool,
    param1: f64,
    param2: f64,
    dist_sq: f64,
}

impl ExtremaCcLocFOfLocEcc2d {
    /// Creates a new 2D function.
    pub fn new() -> Self {
        ExtremaCcLocFOfLocEcc2d {
            is_init: false,
            param1: 0.0,
            param2: 0.0,
            dist_sq: 0.0,
        }
    }

    /// Initializes the function.
    pub fn init(&mut self) {
        self.is_init = true;
    }

    /// Sets curve parameters.
    pub fn set_params(&mut self, p1: f64, p2: f64) {
        self.param1 = p1;
        self.param2 = p2;
    }

    /// Computes distance squared in 2D.
    pub fn compute_dist_sq(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        self.dist_sq = dx * dx + dy * dy;
    }

    /// Returns distance squared.
    pub fn dist_sq(&self) -> f64 {
        self.dist_sq
    }

    /// Returns parameters.
    pub fn params(&self) -> (f64, f64) {
        (self.param1, self.param2)
    }

    /// Checks if initialized.
    pub fn is_init(&self) -> bool {
        self.is_init
    }
}

impl Default for ExtremaCcLocFOfLocEcc2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaCcLocFOfLocEcc2d::new();
        assert!(!func.is_init());
        assert_eq!(func.dist_sq(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut func = ExtremaCcLocFOfLocEcc2d::new();
        func.init();
        assert!(func.is_init());
    }

    #[test]
    fn test_set_params() {
        let mut func = ExtremaCcLocFOfLocEcc2d::new();
        func.set_params(0.25, 0.75);
        let (p1, p2) = func.params();
        assert_eq!(p1, 0.25);
        assert_eq!(p2, 0.75);
    }

    #[test]
    fn test_compute_dist_sq_2d() {
        let mut func = ExtremaCcLocFOfLocEcc2d::new();
        func.compute_dist_sq(0.0, 0.0, 3.0, 4.0);
        assert_eq!(func.dist_sq(), 25.0);
    }
}
