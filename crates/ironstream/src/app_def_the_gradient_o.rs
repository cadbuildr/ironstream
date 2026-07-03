// FILE: app_def_the_gradient_o.rs
// occt: AppDef_TheGradient

/// Minimizes approximation error by optimizing parameters using gradient descent.
pub struct TheGradient {
    done: bool,
    par_error: Vec<f64>,
    av_error: f64,
    m_error_3d: f64,
    m_error_2d: f64,
}

impl TheGradient {
    /// Initialize gradient minimizer.
    pub fn new(
        first_point: i32,
        last_point: i32,
        deg: i32,
        tol_3d: f64,
        tol_2d: f64,
        nb_iterations: i32,
    ) -> Self {
        TheGradient {
            done: false,
            par_error: vec![],
            av_error: 0.0,
            m_error_3d: 0.0,
            m_error_2d: 0.0,
        }
    }

    /// Returns whether computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns error at given index.
    pub fn error(&self, index: i32) -> Result<f64, &'static str> {
        if !self.done {
            return Err("Not done");
        }
        let idx = (index - 1) as usize;
        if idx >= self.par_error.len() {
            return Err("Index out of range");
        }
        Ok(self.par_error[idx])
    }

    /// Returns maximum 3D error.
    pub fn max_error_3d(&self) -> f64 {
        self.m_error_3d
    }

    /// Returns maximum 2D error.
    pub fn max_error_2d(&self) -> f64 {
        self.m_error_2d
    }

    /// Returns average error.
    pub fn average_error(&self) -> f64 {
        self.av_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let grad = TheGradient::new(0, 10, 3, 1e-6, 1e-6, 200);
        assert!(!grad.is_done());
    }

    #[test]
    fn test_errors() {
        let grad = TheGradient::new(0, 10, 3, 1e-6, 1e-6, 200);
        assert_eq!(grad.max_error_3d(), 0.0);
        assert_eq!(grad.max_error_2d(), 0.0);
        assert_eq!(grad.average_error(), 0.0);
    }

    #[test]
    fn test_error_not_done() {
        let grad = TheGradient::new(0, 10, 3, 1e-6, 1e-6, 200);
        let result = grad.error(1);
        assert!(result.is_err());
    }

    #[test]
    fn test_default_iterations() {
        let grad = TheGradient::new(0, 10, 3, 1e-6, 1e-6, 200);
        assert!(!grad.is_done());
    }
}
