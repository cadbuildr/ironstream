// FILE: adv_app2_var_criterion.rs
// occt: AdvApp2Var_Criterion

//! Criterion for surface approximation convergence testing.
pub struct AdvApp2VarCriterion {
    max_error: f64,
    power: usize,
    tol_3d: f64,
}

impl AdvApp2VarCriterion {
    pub fn new(max_error: f64, power: usize, tol_3d: f64) -> Self {
        Self {
            max_error,
            power,
            tol_3d,
        }
    }

    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    pub fn power(&self) -> usize {
        self.power
    }

    pub fn tol_3d(&self) -> f64 {
        self.tol_3d
    }

    pub fn is_converged(&self, error: f64) -> bool {
        error <= self.max_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let crit = AdvApp2VarCriterion::new(0.01, 2, 0.001);
        assert_eq!(crit.max_error(), 0.01);
        assert_eq!(crit.power(), 2);
    }

    #[test]
    fn test_convergence() {
        let crit = AdvApp2VarCriterion::new(0.01, 2, 0.001);
        assert!(crit.is_converged(0.005));
        assert!(!crit.is_converged(0.02));
    }
}
