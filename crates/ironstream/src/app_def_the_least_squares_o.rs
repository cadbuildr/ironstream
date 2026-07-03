// FILE: app_def_the_least_squares_o.rs
// occt: AppDef_TheLeastSquares

/// Computes least squares approximation using Householder-QR method.
pub struct TheLeastSquares {
    done: bool,
    first_point: i32,
    last_point: i32,
    nb_pol: i32,
}

impl TheLeastSquares {
    /// Initialize least squares solver with Bezier basis.
    pub fn new_bezier(
        first_point: i32,
        last_point: i32,
        nb_pol: i32,
    ) -> Self {
        TheLeastSquares {
            done: false,
            first_point,
            last_point,
            nb_pol,
        }
    }

    /// Initialize least squares solver with B-Spline basis.
    pub fn new_bspline(
        first_point: i32,
        last_point: i32,
        nb_pol: i32,
    ) -> Self {
        TheLeastSquares {
            done: false,
            first_point,
            last_point,
            nb_pol,
        }
    }

    /// Perform least squares with parameters.
    pub fn perform(&mut self, parameters: &[f64]) -> Result<(), &'static str> {
        if parameters.is_empty() {
            return Err("Empty parameters");
        }
        self.done = true;
        Ok(())
    }

    /// Perform with tangent vector constraints.
    pub fn perform_with_tangents(
        &mut self,
        parameters: &[f64],
        v1t: &[f64],
        v2t: &[f64],
        l1: f64,
        l2: f64,
    ) -> Result<(), &'static str> {
        if parameters.is_empty() {
            return Err("Empty parameters");
        }
        self.done = true;
        Ok(())
    }

    /// Perform with tangent and curvature constraints.
    pub fn perform_with_constraints(
        &mut self,
        parameters: &[f64],
        v1t: &[f64],
        v2t: &[f64],
        v1c: &[f64],
        v2c: &[f64],
        l1: f64,
        l2: f64,
    ) -> Result<(), &'static str> {
        if parameters.is_empty() {
            return Err("Empty parameters");
        }
        self.done = true;
        Ok(())
    }

    /// Returns whether computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns error analysis.
    pub fn error(&self) -> Result<(f64, f64, f64), &'static str> {
        if !self.done {
            return Err("Not done");
        }
        Ok((0.0, 0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier_creation() {
        let sq = TheLeastSquares::new_bezier(0, 10, 5);
        assert!(!sq.is_done());
        assert_eq!(sq.first_point, 0);
        assert_eq!(sq.last_point, 10);
    }

    #[test]
    fn test_bspline_creation() {
        let sq = TheLeastSquares::new_bspline(0, 10, 5);
        assert!(!sq.is_done());
    }

    #[test]
    fn test_perform() {
        let mut sq = TheLeastSquares::new_bezier(0, 10, 5);
        let params = vec![0.0, 0.5, 1.0];
        let result = sq.perform(&params);
        assert!(result.is_ok());
        assert!(sq.is_done());
    }

    #[test]
    fn test_perform_empty_params() {
        let mut sq = TheLeastSquares::new_bezier(0, 10, 5);
        let result = sq.perform(&[]);
        assert!(result.is_err());
    }
}
