// FILE: int_curve_surface_the_cs_function_of_h_inter.rs
// occt: IntCurveSurface_TheCSFunctionOfHInter

/// Function evaluator for curve-surface intersection problems.
/// Provides function and derivative evaluations for CS intersection analysis.
pub struct IntCurveSurfaceTheCSFunctionOfHInter {
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl IntCurveSurfaceTheCSFunctionOfHInter {
    /// Create a CS function evaluator
    pub fn new() -> Self {
        IntCurveSurfaceTheCSFunctionOfHInter {
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0,
        }
    }

    /// Set the bounds
    pub fn set_bounds(&mut self, u_min: f64, u_max: f64, v_min: f64, v_max: f64) {
        self.u_min = u_min;
        self.u_max = u_max;
        self.v_min = v_min;
        self.v_max = v_max;
    }

    /// Get U bounds
    pub fn u_bounds(&self) -> (f64, f64) {
        (self.u_min, self.u_max)
    }

    /// Get V bounds
    pub fn v_bounds(&self) -> (f64, f64) {
        (self.v_min, self.v_max)
    }
}

impl Default for IntCurveSurfaceTheCSFunctionOfHInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = IntCurveSurfaceTheCSFunctionOfHInter::new();
        let (umin, umax) = func.u_bounds();
        let (vmin, vmax) = func.v_bounds();
        assert_eq!(umin, 0.0);
        assert_eq!(umax, 1.0);
        assert_eq!(vmin, 0.0);
        assert_eq!(vmax, 1.0);
    }

    #[test]
    fn test_set_bounds() {
        let mut func = IntCurveSurfaceTheCSFunctionOfHInter::new();
        func.set_bounds(0.5, 1.5, 0.25, 0.75);
        let (umin, umax) = func.u_bounds();
        let (vmin, vmax) = func.v_bounds();
        assert_eq!(umin, 0.5);
        assert_eq!(umax, 1.5);
        assert_eq!(vmin, 0.25);
        assert_eq!(vmax, 0.75);
    }

    #[test]
    fn test_default() {
        let func = IntCurveSurfaceTheCSFunctionOfHInter::default();
        let (umin, umax) = func.u_bounds();
        assert_eq!(umin, 0.0);
        assert_eq!(umax, 1.0);
    }
}
