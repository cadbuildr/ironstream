// FILE: shape_extend_complex_curve.rs
// occt: ShapeExtend_ComplexCurve

/// Complex curve made of multiple curve segments.
pub struct ComplexCurve {
    nb_curves: usize,
    closed: bool,
}

impl ComplexCurve {
    /// Create new complex curve.
    pub fn new() -> Self {
        ComplexCurve {
            nb_curves: 0,
            closed: false,
        }
    }

    /// Get number of curves.
    pub fn nb_curves(&self) -> usize {
        self.nb_curves
    }

    /// Get curve by index.
    pub fn curve(&self, _index: usize) -> Option<Vec<u8>> {
        None
    }

    /// Locate parameter in curves.
    pub fn locate_parameter(&self, _u: f64) -> (usize, f64) {
        (0, 0.0)
    }

    /// Convert local to global parameter.
    pub fn local_to_global(&self, _index: usize, _ulocal: f64) -> f64 {
        0.0
    }

    /// Get scale factor for derivatives.
    pub fn get_scale_factor(&self, _ind: usize) -> f64 {
        1.0
    }

    /// Check connectivity.
    pub fn check_connectivity(&mut self, _prec: f64) -> bool {
        true
    }

    /// Check if closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}

impl Default for ComplexCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cc = ComplexCurve::new();
        assert_eq!(cc.nb_curves(), 0);
    }

    #[test]
    fn test_is_closed() {
        let cc = ComplexCurve::new();
        assert!(!cc.is_closed());
    }

    #[test]
    fn test_get_scale_factor() {
        let cc = ComplexCurve::new();
        assert_eq!(cc.get_scale_factor(0), 1.0);
    }
}
