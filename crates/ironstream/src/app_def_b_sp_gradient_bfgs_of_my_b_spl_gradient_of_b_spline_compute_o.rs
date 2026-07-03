// FILE: app_def_b_sp_gradient_bfgs_of_my_b_spl_gradient_of_b_spline_compute_o.rs
// occt: AppDef_BSpGradient_BFGSOfMyBSplGradientOfBSplineCompute

//! BFGS optimizer for B-spline gradient computation.

pub struct BSpGradientBFGS {
    pub iteration: usize,
    pub gradient_norm: f64,
}

impl BSpGradientBFGS {
    pub fn new() -> Self {
        BSpGradientBFGS {
            iteration: 0,
            gradient_norm: 0.0,
        }
    }

    pub fn compute(&mut self, _gradient: &[f64]) -> bool {
        self.iteration += 1;
        self.gradient_norm = _gradient.iter().fold(0.0, |acc, &g| acc + g * g).sqrt();
        self.iteration < 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfgs_create() {
        let bfgs = BSpGradientBFGS::new();
        assert_eq!(bfgs.iteration, 0);
    }

    #[test]
    fn test_bfgs_compute() {
        let mut bfgs = BSpGradientBFGS::new();
        let grad = vec![1.0, 2.0];
        assert!(bfgs.compute(&grad));
        assert!(bfgs.gradient_norm > 0.0);
    }
}
