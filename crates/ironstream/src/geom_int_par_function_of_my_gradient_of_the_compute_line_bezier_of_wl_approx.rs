// FILE: geom_int_par_function_of_my_gradient_of_the_compute_line_bezier_of_wl_approx.rs
// occt: GeomInt_ParFunctionOfMyGradientOfTheComputeLineBezierOfWLApprox

pub struct GeomIntParFunctionBezier {
    value: f64,
}

impl GeomIntParFunctionBezier {
    pub fn new() -> Self {
        GeomIntParFunctionBezier { value: 0.0 }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }
}

impl Default for GeomIntParFunctionBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = GeomIntParFunctionBezier::new();
        assert_eq!(func.value(), 0.0);
    }

    #[test]
    fn test_set_value() {
        let mut func = GeomIntParFunctionBezier::new();
        func.set_value(0.5);
        assert_eq!(func.value(), 0.5);
    }
}
