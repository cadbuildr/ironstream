// FILE: app_def_linear_criteria.rs
// occt-ref: AppDefLinearCriteria

//! Approximation and constraint handling class.
pub struct AppDefLinearCriteria {
    is_done: bool,
    error: f64,
}

impl AppDefLinearCriteria {
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

impl Default for AppDefLinearCriteria {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let obj = AppDefLinearCriteria::new();
        assert!(!obj.is_done());
    }

    #[test]
    fn test_perform() {
        let mut obj = AppDefLinearCriteria::new();
        obj.perform();
        assert!(obj.is_done());
    }

    #[test]
    fn test_error() {
        let mut obj = AppDefLinearCriteria::new();
        obj.set_error(0.01);
        assert_eq!(obj.error(), 0.01);
    }
}

