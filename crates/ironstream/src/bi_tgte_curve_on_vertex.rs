// FILE: bi_tgte_curve_on_vertex.rs
// occt: BiTgte_CurveOnVertex

/// Private class used to create a filler rolling on a vertex.
/// This is a curve adaptor implementation for BiTgte operations.
pub struct BiTgteCurveOnVertex {
    first_param: f64,
    last_param: f64,
}

impl BiTgteCurveOnVertex {
    /// Creates a new empty curve on vertex adaptor.
    pub fn new() -> Self {
        BiTgteCurveOnVertex {
            first_param: 0.0,
            last_param: 1.0,
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
        false
    }

    /// Returns whether the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// Returns the period of the curve if periodic.
    pub fn period(&self) -> f64 {
        0.0
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

impl Default for BiTgteCurveOnVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_on_vertex_creation() {
        let curve = BiTgteCurveOnVertex::new();
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 1.0);
    }

    #[test]
    fn test_curve_on_vertex_properties() {
        let curve = BiTgteCurveOnVertex::new();
        assert!(!curve.is_closed());
        assert!(!curve.is_periodic());
        assert_eq!(curve.period(), 0.0);
    }

    #[test]
    fn test_curve_on_vertex_intervals() {
        let curve = BiTgteCurveOnVertex::new();
        assert_eq!(curve.nb_intervals(0), 1);
    }

    #[test]
    fn test_curve_on_vertex_resolution() {
        let curve = BiTgteCurveOnVertex::new();
        assert_eq!(curve.resolution(0.001), 0.001);
    }
}
