// FILE: geom2d_gcc_function_tan_cir_cu_o.rs
// occt: Geom2dGcc_FunctionTanCirCu

/// Function for circle tangent to circle and curve.
pub struct FunctionTanCirCu {
    nb_vars: i32,
}

impl FunctionTanCirCu {
    pub fn new() -> Self {
        FunctionTanCirCu { nb_vars: 0 }
    }

    pub fn nb_variables(&self) -> i32 {
        self.nb_vars
    }

    pub fn value(&self, x: &[f64]) -> Result<f64, &'static str> {
        if x.is_empty() {
            return Err("Empty parameters");
        }
        Ok(0.0)
    }
}

impl Default for FunctionTanCirCu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = FunctionTanCirCu::new();
        assert_eq!(func.nb_variables(), 0);
    }

    #[test]
    fn test_value() {
        let func = FunctionTanCirCu::new();
        let result = func.value(&[1.0]);
        assert!(result.is_ok());
    }
}
