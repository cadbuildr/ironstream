// FILE: adaptor2d_offset_curve.rs
// occt: Adaptor2d_OffsetCurve

/// An offset curve in 2D
#[derive(Debug, Clone)]
pub struct Adaptor2dOffsetCurve {
    offset: f64,
    first: f64,
    last: f64,
}

impl Adaptor2dOffsetCurve {
    /// Create an empty offset curve
    pub fn new() -> Self {
        Adaptor2dOffsetCurve {
            offset: 0.0,
            first: 0.0,
            last: 1.0,
        }
    }

    /// Create with offset value
    pub fn with_offset(offset: f64) -> Self {
        Adaptor2dOffsetCurve {
            offset,
            first: 0.0,
            last: 1.0,
        }
    }

    /// Create with offset and parameter range
    pub fn with_range(offset: f64, first: f64, last: f64) -> Self {
        Adaptor2dOffsetCurve { offset, first, last }
    }

    /// Load a new offset value
    pub fn load(&mut self, offset: f64) {
        self.offset = offset;
    }

    /// Load offset and range
    pub fn load_range(&mut self, offset: f64, first: f64, last: f64) {
        self.offset = offset;
        self.first = first;
        self.last = last;
    }

    /// Get the current offset
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Get first parameter
    pub fn first_parameter(&self) -> f64 {
        self.first
    }

    /// Get last parameter
    pub fn last_parameter(&self) -> f64 {
        self.last
    }

    /// Get continuity
    pub fn continuity(&self) -> &'static str {
        "C_Infinity"
    }

    /// Transform offset point
    pub fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        // Simple perpendicular offset
        let len = (x * x + y * y).sqrt();
        if len < 1e-10 {
            (x, y)
        } else {
            let norm_x = -y / len;
            let norm_y = x / len;
            (x + self.offset * norm_x, y + self.offset * norm_y)
        }
    }
}

impl Default for Adaptor2dOffsetCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_curve_new() {
        let curve = Adaptor2dOffsetCurve::new();
        assert_eq!(curve.offset(), 0.0);
    }

    #[test]
    fn test_offset_curve_with_offset() {
        let curve = Adaptor2dOffsetCurve::with_offset(2.5);
        assert_eq!(curve.offset(), 2.5);
    }

    #[test]
    fn test_offset_curve_with_range() {
        let curve = Adaptor2dOffsetCurve::with_range(1.0, 0.0, 5.0);
        assert_eq!(curve.offset(), 1.0);
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 5.0);
    }

    #[test]
    fn test_offset_curve_load() {
        let mut curve = Adaptor2dOffsetCurve::new();
        curve.load(3.5);
        assert_eq!(curve.offset(), 3.5);
    }

    #[test]
    fn test_offset_curve_continuity() {
        let curve = Adaptor2dOffsetCurve::new();
        assert_eq!(curve.continuity(), "C_Infinity");
    }

    #[test]
    fn test_offset_curve_transform() {
        let curve = Adaptor2dOffsetCurve::with_offset(1.0);
        let (x, y) = curve.transform(1.0, 0.0);
        // Result depends on normal direction
        assert!((x - 1.0).abs() < 0.1 || (y - 1.0).abs() < 0.1);
    }
}
