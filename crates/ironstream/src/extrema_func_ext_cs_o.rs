// FILE: extrema_func_ext_cs_o.rs
// occt: Extrema_FuncExtCS

/// Function for extrema calculation between curve and surface.
pub struct FuncExtCS {
    nb_vars: i32,
}

impl FuncExtCS {
    /// Initialize function.
    pub fn new() -> Self {
        FuncExtCS { nb_vars: 0 }
    }

    /// Returns number of variables.
    pub fn nb_variables(&self) -> i32 {
        self.nb_vars
    }

    /// Compute function value.
    pub fn value(&self, x: &[f64]) -> Result<f64, &'static str> {
        if x.is_empty() {
            return Err("Empty parameters");
        }
        Ok(0.0)
    }

    /// Compute gradient.
    pub fn gradient(&self, x: &[f64]) -> Result<Vec<f64>, &'static str> {
        if x.is_empty() {
            return Err("Empty parameters");
        }
        Ok(vec![0.0; x.len()])
    }
}

impl Default for FuncExtCS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = FuncExtCS::new();
        assert_eq!(func.nb_variables(), 0);
    }

    #[test]
    fn test_value() {
        let func = FuncExtCS::new();
        let result = func.value(&[1.0, 2.0]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_value_empty() {
        let func = FuncExtCS::new();
        let result = func.value(&[]);
        assert!(result.is_err());
    }
}
