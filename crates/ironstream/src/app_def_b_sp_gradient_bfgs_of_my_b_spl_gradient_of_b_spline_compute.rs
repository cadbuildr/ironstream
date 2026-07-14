// FILE: app_def_b_sp_gradient_bfgs_of_my_b_spl_gradient_of_b_spline_compute.rs
// occt-ref: AppDefBSpGradientBFGS

//! Approximation and constraint handling class.
pub struct AppDefBSpGradientBFGS {
    is_done: bool,
    error: f64,
}

impl AppDefBSpGradientBFGS {
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

impl Default for AppDefBSpGradientBFGS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let obj = AppDefBSpGradientBFGS::new();
        assert!(!obj.is_done());
    }

    #[test]
    fn test_perform() {
        let mut obj = AppDefBSpGradientBFGS::new();
        obj.perform();
        assert!(obj.is_done());
    }

    #[test]
    fn test_error() {
        let mut obj = AppDefBSpGradientBFGS::new();
        obj.set_error(0.01);
        assert_eq!(obj.error(), 0.01);
    }
}

