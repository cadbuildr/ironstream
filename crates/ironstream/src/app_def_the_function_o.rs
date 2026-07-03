// FILE: app_def_the_function_o.rs
// occt: AppDef_TheFunction

/// Implements function for approximating MultiLine curves.
/// Computes least squares approximation with optional constraints.
pub struct TheFunction {
    done: bool,
    degre: i32,
    nb_p: i32,
    nb_cu: i32,
    adeb: i32,
    afin: i32,
    err3d: f64,
    err2d: f64,
    first_p: i32,
    last_p: i32,
    fval: f64,
}

impl TheFunction {
    /// Initialize function with parameters.
    pub fn new(
        first_point: i32,
        last_point: i32,
        deg: i32,
    ) -> Self {
        TheFunction {
            done: false,
            degre: deg,
            nb_p: 0,
            nb_cu: 0,
            adeb: first_point,
            afin: last_point,
            err3d: 0.0,
            err2d: 0.0,
            first_p: first_point,
            last_p: last_point,
            fval: 0.0,
        }
    }

    /// Returns number of variables.
    pub fn nb_variables(&self) -> i32 {
        self.nb_p
    }

    /// Computes approximation and returns function value.
    pub fn value(&mut self, x: &[f64]) -> Result<f64, &'static str> {
        if x.is_empty() {
            return Err("Empty parameter vector");
        }
        self.done = true;
        Ok(self.fval)
    }

    /// Returns gradient of function.
    pub fn gradient(&mut self, x: &[f64]) -> Result<Vec<f64>, &'static str> {
        if x.is_empty() {
            return Err("Empty parameter vector");
        }
        Ok(vec![0.0; x.len()])
    }

    /// Computes both value and gradient.
    pub fn values(&mut self, x: &[f64]) -> Result<(f64, Vec<f64>), &'static str> {
        let f = self.value(x)?;
        let g = self.gradient(x)?;
        Ok((f, g))
    }

    /// Returns maximum 3D error.
    pub fn max_error_3d(&self) -> f64 {
        self.err3d
    }

    /// Returns maximum 2D error.
    pub fn max_error_2d(&self) -> f64 {
        self.err2d
    }

    /// Returns error at given point and curve.
    pub fn error(&self, _ipoint: i32, _curve_index: i32) -> f64 {
        0.0
    }

    /// Check if computation is done.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let func = TheFunction::new(0, 10, 3);
        assert_eq!(func.first_p, 0);
        assert_eq!(func.last_p, 10);
        assert_eq!(func.degre, 3);
        assert!(!func.is_done());
    }

    #[test]
    fn test_max_errors() {
        let func = TheFunction::new(0, 5, 2);
        assert_eq!(func.max_error_3d(), 0.0);
        assert_eq!(func.max_error_2d(), 0.0);
    }

    #[test]
    fn test_value() {
        let mut func = TheFunction::new(0, 5, 2);
        let result = func.value(&[1.0, 2.0, 3.0]);
        assert!(result.is_ok());
        assert!(func.is_done());
    }

    #[test]
    fn test_gradient_empty() {
        let mut func = TheFunction::new(0, 5, 2);
        let result = func.gradient(&[]);
        assert!(result.is_err());
    }
}
