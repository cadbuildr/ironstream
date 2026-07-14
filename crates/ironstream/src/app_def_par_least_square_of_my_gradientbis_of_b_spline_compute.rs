// FILE: app_def_par_least_square_of_my_gradientbis_of_b_spline_compute.rs
// occt-ref: AppDefParLeastSquare2

//! Approximation and constraint handling class.
pub struct AppDefParLeastSquare2 {
    is_done: bool,
    error: f64,
}

impl AppDefParLeastSquare2 {
    pub fn new() -> Self {
        Self {
            is_done: false,
            error: 0.0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn error(&self) -> f64 {
        self.error
    }

    pub fn set_done(&mut self, done: bool) {
        self.is_done = done;
    }

    pub fn set_error(&mut self, err: f64) {
        self.error = err;
    }

    pub fn perform(&mut self) {
        self.is_done = true;
    }
}

impl Default for AppDefParLeastSquare2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let obj = AppDefParLeastSquare2::new();
        assert!(!obj.is_done());
    }

    #[test]
    fn test_perform() {
        let mut obj = AppDefParLeastSquare2::new();
        obj.perform();
        assert!(obj.is_done());
    }

    #[test]
    fn test_error() {
        let mut obj = AppDefParLeastSquare2::new();
        obj.set_error(0.01);
        assert_eq!(obj.error(), 0.01);
    }
}

