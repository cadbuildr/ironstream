// FILE: math_multiple_var_function_with_gradient.rs
// occt: math_MultipleVarFunctionWithGradient

/// Multiple variable function with gradient computation.
pub trait MultipleVarFunctionWithGradient {
    fn nb_variables(&self) -> usize;
    fn value(&self, x: &[f64]) -> Option<f64>;
    fn gradient(&self, x: &[f64]) -> Option<Vec<f64>>;
}

pub struct SimpleMultipleVarFunction {
    nb_vars: usize,
}

impl SimpleMultipleVarFunction {
    pub fn new(nb_vars: usize) -> Self {
        SimpleMultipleVarFunction { nb_vars }
    }
}

impl MultipleVarFunctionWithGradient for SimpleMultipleVarFunction {
    fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    fn value(&self, _x: &[f64]) -> Option<f64> {
        Some(0.0)
    }

    fn gradient(&self, x: &[f64]) -> Option<Vec<f64>> {
        Some(vec![0.0; x.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_var_function() {
        let f = SimpleMultipleVarFunction::new(3);
        assert_eq!(f.nb_variables(), 3);
    }
}
