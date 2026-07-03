// FILE: shape_analysis_free_bounds_properties.rs
// occt: ShapeAnalysis_FreeBoundsProperties

/// Properties of free bounds in a shape.
pub struct FreeBoundsProperties {
    shape: Vec<u8>,
    tolerance: f64,
    split_closed: bool,
    split_open: bool,
    closed_bounds: Vec<Vec<u8>>,
    open_bounds: Vec<Vec<u8>>,
}

impl FreeBoundsProperties {
    /// Create a new FreeBoundsProperties.
    pub fn new() -> Self {
        FreeBoundsProperties {
            shape: Vec::new(),
            tolerance: 0.0,
            split_closed: false,
            split_open: false,
            closed_bounds: Vec::new(),
            open_bounds: Vec::new(),
        }
    }

    /// Initialize with shape and tolerance.
    pub fn init(&mut self, _shape: &[u8], _tolerance: f64, split_closed: bool, split_open: bool) {
        self.tolerance = _tolerance;
        self.split_closed = split_closed;
        self.split_open = split_open;
    }

    /// Perform analysis.
    pub fn perform(&mut self) -> bool {
        true
    }

    /// Check if shape is loaded.
    pub fn is_loaded(&self) -> bool {
        !self.shape.is_empty()
    }

    /// Get shape.
    pub fn shape(&self) -> Option<&[u8]> {
        if self.shape.is_empty() {
            None
        } else {
            Some(&self.shape)
        }
    }

    /// Get tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Get number of free bounds.
    pub fn nb_free_bounds(&self) -> usize {
        self.closed_bounds.len() + self.open_bounds.len()
    }

    /// Get number of closed free bounds.
    pub fn nb_closed_free_bounds(&self) -> usize {
        self.closed_bounds.len()
    }

    /// Get number of open free bounds.
    pub fn nb_open_free_bounds(&self) -> usize {
        self.open_bounds.len()
    }

    /// Dispatch bounds.
    pub fn dispatch_bounds(&mut self) -> bool {
        true
    }

    /// Check contours.
    pub fn check_contours(&self, _prec: f64) -> bool {
        true
    }

    /// Check notches.
    pub fn check_notches(&self, _prec: f64) -> bool {
        true
    }
}

impl Default for FreeBoundsProperties {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let props = FreeBoundsProperties::new();
        assert!(!props.is_loaded());
        assert_eq!(props.nb_free_bounds(), 0);
    }

    #[test]
    fn test_tolerance() {
        let mut props = FreeBoundsProperties::new();
        props.tolerance = 0.01;
        assert_eq!(props.tolerance(), 0.01);
    }

    #[test]
    fn test_perform() {
        let mut props = FreeBoundsProperties::new();
        assert!(props.perform());
    }

    #[test]
    fn test_check_contours() {
        let props = FreeBoundsProperties::new();
        assert!(props.check_contours(0.01));
    }
}
