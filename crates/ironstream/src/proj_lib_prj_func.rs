// FILE: proj_lib_prj_func.rs
// occt: ProjLib_PrjFunc

/// Projection function.
pub struct ProjLibPrjFunc {
    params: Vec<f64>,
}

impl ProjLibPrjFunc {
    /// Create a projection function.
    pub fn new() -> Self {
        ProjLibPrjFunc {
            params: Vec::new(),
        }
    }

    /// Add a parameter.
    pub fn add_param(&mut self, p: f64) {
        self.params.push(p);
    }

    /// Get the number of parameters.
    pub fn nb_params(&self) -> usize {
        self.params.len()
    }

    /// Get a parameter.
    pub fn param(&self, index: usize) -> Option<f64> {
        if index < self.params.len() {
            Some(self.params[index])
        } else {
            None
        }
    }
}

impl Default for ProjLibPrjFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_func() {
        let func = ProjLibPrjFunc::new();
        assert_eq!(func.nb_params(), 0);
    }

    #[test]
    fn test_add_param() {
        let mut func = ProjLibPrjFunc::new();
        func.add_param(1.5);
        func.add_param(2.5);
        assert_eq!(func.nb_params(), 2);
        assert_eq!(func.param(0), Some(1.5));
        assert_eq!(func.param(1), Some(2.5));
    }
}
