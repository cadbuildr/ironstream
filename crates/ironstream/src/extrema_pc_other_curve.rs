// FILE: extrema_pc_other_curve.rs
// occt: ExtremaPC_OtherCurve

/// Adapter for general curves in point-curve extrema computation.
pub struct ExtremaPcOtherCurve {
    first_param: f64,
    last_param: f64,
    is_periodic: bool,
}

impl ExtremaPcOtherCurve {
    /// Creates a new curve adapter.
    pub fn new() -> Self {
        ExtremaPcOtherCurve {
            first_param: 0.0,
            last_param: 1.0,
            is_periodic: false,
        }
    }

    /// Creates with parameter range.
    pub fn with_range(first: f64, last: f64) -> Self {
        ExtremaPcOtherCurve {
            first_param: first,
            last_param: last,
            is_periodic: false,
        }
    }

    /// Sets first parameter.
    pub fn set_first_param(&mut self, first: f64) {
        self.first_param = first;
    }

    /// Sets last parameter.
    pub fn set_last_param(&mut self, last: f64) {
        self.last_param = last;
    }

    /// Sets periodicity.
    pub fn set_periodic(&mut self, periodic: bool) {
        self.is_periodic = periodic;
    }

    /// Returns first parameter.
    pub fn first_param(&self) -> f64 {
        self.first_param
    }

    /// Returns last parameter.
    pub fn last_param(&self) -> f64 {
        self.last_param
    }

    /// Returns parameter range.
    pub fn param_range(&self) -> f64 {
        self.last_param - self.first_param
    }

    /// Checks if periodic.
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// Normalizes parameter to range.
    pub fn normalize_param(&self, param: f64) -> f64 {
        if self.is_periodic {
            let range = self.param_range();
            let mut p = param - self.first_param;
            while p < 0.0 {
                p += range;
            }
            while p > range {
                p -= range;
            }
            self.first_param + p
        } else {
            param.max(self.first_param).min(self.last_param)
        }
    }
}

impl Default for ExtremaPcOtherCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = ExtremaPcOtherCurve::new();
        assert_eq!(curve.first_param(), 0.0);
        assert_eq!(curve.last_param(), 1.0);
        assert!(!curve.is_periodic());
    }

    #[test]
    fn test_with_range() {
        let curve = ExtremaPcOtherCurve::with_range(-1.0, 2.0);
        assert_eq!(curve.first_param(), -1.0);
        assert_eq!(curve.last_param(), 2.0);
        assert_eq!(curve.param_range(), 3.0);
    }

    #[test]
    fn test_normalize_param_non_periodic() {
        let curve = ExtremaPcOtherCurve::with_range(0.0, 10.0);
        assert_eq!(curve.normalize_param(-5.0), 0.0);
        assert_eq!(curve.normalize_param(5.0), 5.0);
        assert_eq!(curve.normalize_param(15.0), 10.0);
    }

    #[test]
    fn test_normalize_param_periodic() {
        let mut curve = ExtremaPcOtherCurve::with_range(0.0, 10.0);
        curve.set_periodic(true);
        assert_eq!(curve.normalize_param(11.0), 1.0);
        assert_eq!(curve.normalize_param(-1.0), 9.0);
    }
}
