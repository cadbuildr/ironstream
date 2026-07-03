// FILE: math_multiple_var_function_with_hessian.rs
// occt: math_MultipleVarFunctionWithHessian

/// Multiple variable function with Hessian (2nd derivatives).
pub trait MultipleVarFunctionWithHessian {
    fn nb_variables(&self) -> usize;
    fn value(&self, x: &[f64]) -> Option<f64>;
    fn gradient(&self, x: &[f64]) -> Option<Vec<f64>>;
    fn hessian(&self, x: &[f64]) -> Option<Vec<Vec<f64>>>;
}

pub struct SimpleHessianFunction {
    nb_vars: usize,
}

impl SimpleHessianFunction {
    pub fn new(nb_vars: usize) -> Self {
        SimpleHessianFunction { nb_vars }
    }
}

impl MultipleVarFunctionWithHessian for SimpleHessianFunction {
    fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    fn value(&self, _x: &[f64]) -> Option<f64> {
        Some(0.0)
    }

    fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        Some(vec![0.0; x.len()])
    }

    fn hessian(&self, x: &[f64]) -> Option<Vec<Vec<f64>>> {
        Some(vec![vec![0.0; x.len()]; x.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hessian_function() {
        let f = SimpleHessianFunction::new(2);
        assert_eq!(f.nb_variables(), 2);
    }
}
