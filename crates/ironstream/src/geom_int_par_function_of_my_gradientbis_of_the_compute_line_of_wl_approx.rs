// FILE: geom_int_par_function_of_my_gradientbis_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_ParFunctionOfMyGradientbisOfTheComputeLineOfWLApprox

pub struct GeomIntParFunctionGradientbis {
    value: f64,
}

impl GeomIntParFunctionGradientbis {
    pub fn new() -> Self {
        GeomIntParFunctionGradientbis { value: 0.0 }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }
}

impl Default for GeomIntParFunctionGradientbis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = GeomIntParFunctionGradientbis::new();
        assert_eq!(func.value(), 0.0);
    }

    #[test]
    fn test_set_value() {
        let mut func = GeomIntParFunctionGradientbis::new();
        func.set_value(1.5);
        assert_eq!(func.value(), 1.5);
    }
}
