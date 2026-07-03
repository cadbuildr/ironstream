// FILE: geom_int_my_b_spl_gradient_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_MyBSplGradientOfTheComputeLineOfWLApprox

pub struct GeomIntMyBSplGradient {
    convergence: bool,
}

impl GeomIntMyBSplGradient {
    pub fn new() -> Self {
        GeomIntMyBSplGradient { convergence: false }
    }

    pub fn is_converged(&self) -> bool {
        self.convergence
    }

    pub fn set_convergence(&mut self, conv: bool) {
        self.convergence = conv;
    }
}

impl Default for GeomIntMyBSplGradient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let grad = GeomIntMyBSplGradient::new();
        assert!(!grad.is_converged());
    }

    #[test]
    fn test_set_convergence() {
        let mut grad = GeomIntMyBSplGradient::new();
        grad.set_convergence(true);
        assert!(grad.is_converged());
    }
}
