// FILE: std_object_gp_curves.rs
// occt: StdObject_gp_Curves

/// Persistent representation of geometric curves
pub struct GpCurve {
    curve_type: i32,
    params: Vec<f64>,
}

impl GpCurve {
    /// Create a new curve
    pub fn new(curve_type: i32) -> Self {
        GpCurve {
            curve_type,
            params: Vec::new(),
        }
    }

    /// Get curve type
    pub fn curve_type(&self) -> i32 {
        self.curve_type
    }

    /// Get parameters
    pub fn params(&self) -> &[f64] {
        &self.params
    }

    /// Set parameters
    pub fn set_params(&mut self, params: Vec<f64>) {
        self.params = params;
    }

    /// Add a parameter
    pub fn add_param(&mut self, param: f64) {
        self.params.push(param);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let curve = GpCurve::new(1);
        assert_eq!(curve.curve_type(), 1);
        assert!(curve.params().is_empty());
    }

    #[test]
    fn test_add_param() {
        let mut curve = GpCurve::new(1);
        curve.add_param(1.5);
        curve.add_param(2.5);

        assert_eq!(curve.params().len(), 2);
        assert_eq!(curve.params()[0], 1.5);
    }

    #[test]
    fn test_set_params() {
        let mut curve = GpCurve::new(1);
        let params = vec![1.0, 2.0, 3.0];
        curve.set_params(params.clone());

        assert_eq!(curve.params(), &params[..]);
    }
}
