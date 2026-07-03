// FILE: geom_int_gradient_bfgs_of_my_gradient_of_the_compute_line_bezier_of_wl_approx.rs
// occt: GeomInt_Gradient_BFGSOfMyGradientOfTheComputeLineBezierOfWLApprox

pub struct GeomIntGradientBFGSOfMyGradient {
    iterations: i32,
}

impl GeomIntGradientBFGSOfMyGradient {
    pub fn new() -> Self {
        GeomIntGradientBFGSOfMyGradient { iterations: 0 }
    }

    pub fn iterations(&self) -> i32 {
        self.iterations
    }
}

impl Default for GeomIntGradientBFGSOfMyGradient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let grad = GeomIntGradientBFGSOfMyGradient::new();
        assert_eq!(grad.iterations(), 0);
    }
}
