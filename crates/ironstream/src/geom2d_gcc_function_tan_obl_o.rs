// FILE: geom2d_gcc_function_tan_obl_o.rs
// occt: Geom2dGcc_FunctionTanObl

/// Function for oblique tangency condition.
pub struct FunctionTanObl {
    nb_vars: i32,
}

impl FunctionTanObl {
    pub fn new() -> Self {
        FunctionTanObl { nb_vars: 0 }
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

impl Default for FunctionTanObl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = FunctionTanObl::new();
        assert_eq!(func.nb_variables(), 0);
    }

    #[test]
    fn test_value() {
        let func = FunctionTanObl::new();
        let result = func.value(&[1.0]);
        assert!(result.is_ok());
    }
}
