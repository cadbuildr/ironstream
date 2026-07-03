// FILE: int_curve_i_conic_tool.rs
// occt: IntCurve_IConicTool

/// Tool for handling implicit conic curves in intersection problems.
/// Provides function evaluation, distance, and parameter finding for conics.
pub struct IntCurveIConicTool {
    param1: f64,
    param2: f64,
    param3: f64,
    conic_type: u32,
}

impl IntCurveIConicTool {
    /// Create a default conic tool
    pub fn new() -> Self {
        IntCurveIConicTool {
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            conic_type: 0,
        }
    }

    /// Set conic type (0=ellipse, 1=line, 2=circle, 3=parabola, 4=hyperbola)
    pub fn set_type(&mut self, typ: u32) {
        self.conic_type = typ;
    }

    /// Get conic type
    pub fn conic_type(&self) -> u32 {
        self.conic_type
    }

    /// Evaluate point on conic at parameter u
    pub fn value(&self, u: f64) -> [f64; 2] {
        [u.cos() * self.param1, u.sin() * self.param1]
    }

    /// Evaluate derivative at parameter u
    pub fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]) {
        let p = [u.cos() * self.param1, u.sin() * self.param1];
        let t = [-u.sin() * self.param1, u.cos() * self.param1];
        (p, t)
    }

    /// Compute distance from point to conic
    pub fn distance(&self, px: f64, py: f64) -> f64 {
        ((px * px + py * py) - self.param1).abs()
    }

    /// Find parameter for closest point on conic
    pub fn find_parameter(&self, px: f64, py: f64) -> f64 {
        py.atan2(px)
    }
}

impl Default for IntCurveIConicTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IntCurveIConicTool::new();
        assert_eq!(tool.conic_type(), 0);
    }

    #[test]
    fn test_set_type() {
        let mut tool = IntCurveIConicTool::new();
        tool.set_type(2);
        assert_eq!(tool.conic_type(), 2);
    }

    #[test]
    fn test_value() {
        let mut tool = IntCurveIConicTool::new();
        tool.param1 = 1.0;
        let pt = tool.value(0.0);
        assert!((pt[0] - 1.0).abs() < 1e-10);
        assert!(pt[1].abs() < 1e-10);
    }

    #[test]
    fn test_distance() {
        let mut tool = IntCurveIConicTool::new();
        tool.param1 = 1.0;
        let dist = tool.distance(0.0, 0.0);
        assert!((dist - 1.0).abs() < 1e-10);
    }
}
