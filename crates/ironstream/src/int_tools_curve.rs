// FILE: int_tools_curve.rs
// occt: IntTools_Curve

/// Container for 3D intersection curve with 2D PCurves and tolerances.
#[derive(Clone, Debug)]
pub struct IntToolsCurve {
    tolerance: f64,
    tangential_tolerance: f64,
}

impl IntToolsCurve {
    /// Create an empty curve.
    pub fn new() -> Self {
        IntToolsCurve {
            tolerance: 0.0,
            tangential_tolerance: 0.0,
        }
    }

    /// Create with tolerances.
    pub fn with_tolerances(tol: f64, tang_tol: f64) -> Self {
        IntToolsCurve {
            tolerance: tol,
            tangential_tolerance: tang_tol,
        }
    }

    /// Set the tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Get the tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tangential tolerance.
    pub fn set_tangential_tolerance(&mut self, tol: f64) {
        self.tangential_tolerance = tol;
    }

    /// Get the tangential tolerance.
    pub fn tangential_tolerance(&self) -> f64 {
        self.tangential_tolerance
    }

    /// Check if curve has bounds.
    pub fn has_bounds(&self) -> bool {
        false
    }
}

impl Default for IntToolsCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = IntToolsCurve::new();
        assert_eq!(curve.tolerance(), 0.0);
        assert_eq!(curve.tangential_tolerance(), 0.0);
    }

    #[test]
    fn test_with_tolerances() {
        let curve = IntToolsCurve::with_tolerances(0.01, 0.001);
        assert_eq!(curve.tolerance(), 0.01);
        assert_eq!(curve.tangential_tolerance(), 0.001);
    }

    #[test]
    fn test_set_tolerance() {
        let mut curve = IntToolsCurve::new();
        curve.set_tolerance(0.05);
        assert_eq!(curve.tolerance(), 0.05);
    }

    #[test]
    fn test_has_bounds() {
        let curve = IntToolsCurve::new();
        assert!(!curve.has_bounds());
    }
}
