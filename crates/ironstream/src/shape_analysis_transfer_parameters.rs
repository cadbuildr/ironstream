// FILE: shape_analysis_transfer_parameters.rs
// occt: ShapeAnalysis_TransferParameters

/// Transfers parameters between 3D curve and 2D pcurve.
pub struct TransferParameters {
    first: f64,
    last: f64,
    first2d: f64,
    last2d: f64,
    shift: f64,
    scale: f64,
    max_tolerance: f64,
}

impl TransferParameters {
    /// Create empty tool with shift=0, scale=1.
    pub fn new() -> Self {
        TransferParameters {
            first: 0.0,
            last: 0.0,
            first2d: 0.0,
            last2d: 0.0,
            shift: 0.0,
            scale: 1.0,
            max_tolerance: f64::MAX,
        }
    }

    /// Initialize with edge and face.
    pub fn init(&mut self, _edge: &[u8], _face: &[u8]) {
        // Implementation stub
    }

    /// Set maximum tolerance.
    pub fn set_max_tolerance(&mut self, maxtol: f64) {
        self.max_tolerance = maxtol;
    }

    /// Perform parameter transfer.
    pub fn perform(&self, param: f64, to2d: bool) -> f64 {
        if to2d {
            self.shift + self.scale * param
        } else {
            (param - self.shift) / self.scale
        }
    }

    /// Check if curves are same range.
    pub fn is_same_range(&self) -> bool {
        self.scale == 1.0 && self.shift == 0.0
    }
}

impl Default for TransferParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tp = TransferParameters::new();
        assert_eq!(tp.scale, 1.0);
        assert_eq!(tp.shift, 0.0);
    }

    #[test]
    fn test_is_same_range() {
        let tp = TransferParameters::new();
        assert!(tp.is_same_range());
    }

    #[test]
    fn test_perform_to2d() {
        let tp = TransferParameters::new();
        assert_eq!(tp.perform(1.0, true), 1.0);
    }

    #[test]
    fn test_perform_from2d() {
        let tp = TransferParameters::new();
        assert_eq!(tp.perform(1.0, false), 1.0);
    }

    #[test]
    fn test_set_max_tolerance() {
        let mut tp = TransferParameters::new();
        tp.set_max_tolerance(0.01);
        assert_eq!(tp.max_tolerance, 0.01);
    }
}
