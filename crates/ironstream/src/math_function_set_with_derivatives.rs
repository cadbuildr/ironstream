// FILE: math_function_set_with_derivatives.rs
// occt: math_FunctionSetWithDerivatives

/// Function set with derivatives (Jacobian).
pub trait FunctionSetWithDerivatives {
    fn nb_equations(&self) -> usize;
    fn nb_variables(&self) -> usize;
    fn value(&self, x: &[f64], f: &mut [f64]) -> bool;
    fn derivatives(&self, x: &[f64], j: &mut [Vec<f64>]) -> bool;
}

pub struct SimpleFunctionSetWithDerivatives {
    nb_eqs: usize,
    nb_vars: usize,
}

impl SimpleFunctionSetWithDerivatives {
    pub fn new(nb_eqs: usize, nb_vars: usize) -> Self {
        SimpleFunctionSetWithDerivatives {
            nb_eqs,
            nb_vars,
        }
    }
}

impl FunctionSetWithDerivatives for SimpleFunctionSetWithDerivatives {
    fn nb_equations(&self) -> usize {
        self.nb_eqs
    }

    fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    fn value(&self, _x: &[f64], _f: &mut [f64]) -> bool {
        true
    }

    fn derivatives(&self, _x: &[f64], _j: &mut [Vec<f64>]) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_set_with_derivatives() {
        let fs = SimpleFunctionSetWithDerivatives::new(2, 2);
        assert_eq!(fs.nb_equations(), 2);
        assert_eq!(fs.nb_variables(), 2);
    }
}
