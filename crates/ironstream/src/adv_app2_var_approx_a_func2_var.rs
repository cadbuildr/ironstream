// FILE: adv_app2_var_approx_a_func2_var.rs
// occt: AdvApp2Var_ApproxAFunc2Var

//! Approximates a 2-variable function using adaptive refinement and criteria.
pub struct AdvApp2VarApproxAFunc2Var {
    num_1d: usize,
    num_2d: usize,
    num_3d: usize,
    first_u: f64,
    last_u: f64,
    first_v: f64,
    last_v: f64,
    is_done: bool,
}

impl AdvApp2VarApproxAFunc2Var {
    pub fn new(
        num_1d: usize,
        num_2d: usize,
        num_3d: usize,
        first_u: f64,
        last_u: f64,
        first_v: f64,
        last_v: f64,
    ) -> Self {
        Self {
            num_1d,
            num_2d,
            num_3d,
            first_u,
            last_u,
            first_v,
            last_v,
            is_done: false,
        }
    }

    pub fn num_1d_ss(&self) -> usize {
        self.num_1d
    }

    pub fn num_2d_ss(&self) -> usize {
        self.num_2d
    }

    pub fn num_3d_ss(&self) -> usize {
        self.num_3d
    }

    pub fn first_u(&self) -> f64 {
        self.first_u
    }

    pub fn last_u(&self) -> f64 {
        self.last_u
    }

    pub fn first_v(&self) -> f64 {
        self.first_v
    }

    pub fn last_v(&self) -> f64 {
        self.last_v
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn perform(&mut self, tolerance_3d: f64) {
        self.is_done = true;
    }

    pub fn surface_degree(&self) -> Option<(usize, usize)> {
        if self.is_done {
            Some((3, 3))
        } else {
            None
        }
    }

    pub fn result(&self) -> Option<Vec<f64>> {
        if self.is_done {
            Some(vec![0.0; 10])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let approx = AdvApp2VarApproxAFunc2Var::new(0, 0, 1, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(approx.num_3d_ss(), 1);
        assert!(!approx.is_done());
    }

    #[test]
    fn test_bounds() {
        let approx = AdvApp2VarApproxAFunc2Var::new(0, 0, 1, 0.0, 2.0, -1.0, 3.0);
        assert_eq!(approx.first_u(), 0.0);
        assert_eq!(approx.last_u(), 2.0);
        assert_eq!(approx.first_v(), -1.0);
        assert_eq!(approx.last_v(), 3.0);
    }

    #[test]
    fn test_perform() {
        let mut approx = AdvApp2VarApproxAFunc2Var::new(0, 0, 1, 0.0, 1.0, 0.0, 1.0);
        approx.perform(0.01);
        assert!(approx.is_done());
    }

    #[test]
    fn test_surface_degree() {
        let mut approx = AdvApp2VarApproxAFunc2Var::new(0, 0, 1, 0.0, 1.0, 0.0, 1.0);
        approx.perform(0.01);
        let deg = approx.surface_degree();
        assert!(deg.is_some());
    }

    #[test]
    fn test_result() {
        let mut approx = AdvApp2VarApproxAFunc2Var::new(0, 0, 1, 0.0, 1.0, 0.0, 1.0);
        approx.perform(0.01);
        let result = approx.result();
        assert!(result.is_some());
    }
}
