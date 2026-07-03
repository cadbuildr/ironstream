// FILE: math_function_set.rs
// occt: math_FunctionSet

/// Set of N functions of N variables for solving systems.
pub trait FunctionSet {
    /// Number of functions.
    fn nb_equations(&self) -> usize;

    /// Number of variables.
    fn nb_variables(&self) -> usize;

    /// Evaluate system at point.
    fn value(&self, x: &[f64], f: &mut [f64]) -> bool;
}

/// Simple implementation wrapper.
pub struct SimpleFunctionSet {
    nb_eqs: usize,
    nb_vars: usize,
}

impl SimpleFunctionSet {
    pub fn new(nb_eqs: usize, nb_vars: usize) -> Self {
        SimpleFunctionSet {
            nb_eqs,
            nb_vars,
        }
    }
}

impl FunctionSet for SimpleFunctionSet {
    fn nb_equations(&self) -> usize {
        self.nb_eqs
    }

    fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    fn value(&self, _x: &[f64], _f: &mut [f64]) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function_set() {
        let fs = SimpleFunctionSet::new(2, 2);
        assert_eq!(fs.nb_equations(), 2);
        assert_eq!(fs.nb_variables(), 2);
    }

    #[test]
    fn test_function_set_value() {
        let fs = SimpleFunctionSet::new(2, 2);
        let x = vec![1.0, 2.0];
        let mut f = vec![0.0, 0.0];
        assert!(fs.value(&x, &mut f));
    }
}
