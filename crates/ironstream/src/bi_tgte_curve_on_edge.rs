// FILE: bi_tgte_curve_on_edge.rs
// occt: BiTgte_CurveOnEdge

/// Private class used to create a filler rolling on an edge.
/// This is a curve adaptor implementation for BiTgte operations.
pub struct BiTgteCurveOnEdge {
    first_param: f64,
    last_param: f64,
    is_closed: bool,
    is_periodic: bool,
}

impl BiTgteCurveOnEdge {
    /// Creates a new empty curve on edge adaptor.
    pub fn new() -> Self {
        BiTgteCurveOnEdge {
            first_param: 0.0,
            last_param: 1.0,
            is_closed: false,
            is_periodic: false,
        }
    }

    /// Returns the first parameter of the curve.
    pub fn first_parameter(&self) -> f64 {
        self.first_param
    }

    /// Returns the last parameter of the curve.
    pub fn last_parameter(&self) -> f64 {
        self.last_param
    }

    /// Returns whether the curve is closed.
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Returns whether the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// Returns the period of the curve if periodic.
    pub fn period(&self) -> f64 {
        if self.is_periodic {
            self.last_param - self.first_param
        } else {
            0.0
        }
    }

    /// Returns the number of intervals for the given continuity.
    pub fn nb_intervals(&self, _continuity: i32) -> i32 {
        1
    }

    /// Returns the parametric resolution corresponding to the real space resolution.
    pub fn resolution(&self, r3d: f64) -> f64 {
        r3d
    }
}

impl Default for BiTgteCurveOnEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_on_edge_creation() {
        let curve = BiTgteCurveOnEdge::new();
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 1.0);
        assert!(!curve.is_closed());
        assert!(!curve.is_periodic());
    }

    #[test]
    fn test_curve_on_edge_resolution() {
        let curve = BiTgteCurveOnEdge::new();
        assert_eq!(curve.resolution(0.001), 0.001);
    }

    #[test]
    fn test_curve_on_edge_intervals() {
        let curve = BiTgteCurveOnEdge::new();
        assert_eq!(curve.nb_intervals(0), 1);
    }

    #[test]
    fn test_curve_on_edge_period_non_periodic() {
        let curve = BiTgteCurveOnEdge::new();
        assert_eq!(curve.period(), 0.0);
    }
}
