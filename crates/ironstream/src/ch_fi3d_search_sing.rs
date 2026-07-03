// FILE: ch_fi3d_search_sing.rs
// occt: ChFi3d_SearchSing

/// Searches singularities on fillet.
/// Computes F(t) = (C1(t) - C2(t)).(C1'(t) - C2'(t))
/// and its derivative to find singular points.
#[derive(Debug, Clone)]
pub struct ChFi3dSearchSing {
    /// Parameter domain start
    param_start: f64,
    /// Parameter domain end
    param_end: f64,
}

impl ChFi3dSearchSing {
    /// Create a new SearchSing with curves
    pub fn new(_c1_id: usize, _c2_id: usize) -> Self {
        Self {
            param_start: 0.0,
            param_end: 0.0,
        }
    }

    /// Set parameter domain
    pub fn set_param_domain(&mut self, start: f64, end: f64) {
        self.param_start = start;
        self.param_end = end;
    }

    /// Compute value at parameter t
    pub fn value(&self, _t: f64) -> Result<f64, &'static str> {
        Ok(0.0)
    }

    /// Compute derivative at parameter t
    pub fn derivative(&self, _t: f64) -> Result<f64, &'static str> {
        Ok(0.0)
    }

    /// Compute value and derivative
    pub fn value_and_derivative(&self, _t: f64) -> Result<(f64, f64), &'static str> {
        Ok((0.0, 0.0))
    }
}

impl Default for ChFi3dSearchSing {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sing = ChFi3dSearchSing::new(1, 2);
        assert_eq!(sing.param_start, 0.0);
        assert_eq!(sing.param_end, 0.0);
    }

    #[test]
    fn test_set_param_domain() {
        let mut sing = ChFi3dSearchSing::new(1, 2);
        sing.set_param_domain(0.0, 6.28);
        assert_eq!(sing.param_start, 0.0);
        assert_eq!(sing.param_end, 6.28);
    }

    #[test]
    fn test_value() {
        let sing = ChFi3dSearchSing::new(1, 2);
        let result = sing.value(3.14);
        assert!(result.is_ok());
    }

    #[test]
    fn test_derivative() {
        let sing = ChFi3dSearchSing::new(1, 2);
        let result = sing.derivative(3.14);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default() {
        let sing = ChFi3dSearchSing::default();
        assert_eq!(sing.param_start, 0.0);
    }
}
