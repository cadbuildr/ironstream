// FILE: geom_int_gradient_bfgs_of_my_gradientbis_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_Gradient_BFGSOfMyGradientbisOfTheComputeLineOfWLApprox

pub struct GeomIntGradientBFGSOfMyGradientbis {
    iterations: i32,
}

impl GeomIntGradientBFGSOfMyGradientbis {
    pub fn new() -> Self {
        GeomIntGradientBFGSOfMyGradientbis { iterations: 0 }
    }

    pub fn iterations(&self) -> i32 {
        self.iterations
    }
}

impl Default for GeomIntGradientBFGSOfMyGradientbis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let grad = GeomIntGradientBFGSOfMyGradientbis::new();
        assert_eq!(grad.iterations(), 0);
    }
}
