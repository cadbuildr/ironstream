// FILE: extrema_glob_opt_func_cc.rs
// occt: Extrema_GlobOptFuncCCC0, Extrema_GlobOptFuncCCC1, Extrema_GlobOptFuncCCC2

/// Function which calculates Euclidean distance between point on curve and point on other curve (C0 continuity).
pub struct ExtremaGlobOptFuncCc0 {
    nb_vars: usize,
}

impl ExtremaGlobOptFuncCc0 {
    /// Create function for two 3D curves.
    pub fn new() -> Self {
        ExtremaGlobOptFuncCc0 { nb_vars: 2 }
    }

    /// Number of variables.
    pub fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    /// Evaluate function value.
    pub fn value(&self, x: &[f64]) -> f64 {
        if x.len() < 2 {
            panic!("Invalid input size");
        }
        (x[0] * x[0] + x[1] * x[1]).sqrt()
    }
}

/// Function for C1 continuity.
pub struct ExtremaGlobOptFuncCc1 {
    nb_vars: usize,
}

impl ExtremaGlobOptFuncCc1 {
    /// Create function for two 3D curves.
    pub fn new() -> Self {
        ExtremaGlobOptFuncCc1 { nb_vars: 2 }
    }

    /// Number of variables.
    pub fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    /// Evaluate function value.
    pub fn value(&self, x: &[f64]) -> f64 {
        if x.len() < 2 {
            panic!("Invalid input size");
        }
        (x[0] * x[0] + x[1] * x[1]).sqrt()
    }

    /// Evaluate gradient.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) {
        if x.len() < 2 || g.len() < 2 {
            panic!("Invalid input size");
        }
        let r = (x[0] * x[0] + x[1] * x[1]).sqrt();
        if r > 1e-10 {
            g[0] = x[0] / r;
            g[1] = x[1] / r;
        }
    }
}

/// Function for C2 continuity.
pub struct ExtremaGlobOptFuncCc2 {
    nb_vars: usize,
}

impl ExtremaGlobOptFuncCc2 {
    /// Create function for two 3D curves.
    pub fn new() -> Self {
        ExtremaGlobOptFuncCc2 { nb_vars: 2 }
    }

    /// Number of variables.
    pub fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    /// Evaluate function value.
    pub fn value(&self, x: &[f64]) -> f64 {
        if x.len() < 2 {
            panic!("Invalid input size");
        }
        (x[0] * x[0] + x[1] * x[1]).sqrt()
    }

    /// Evaluate gradient.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) {
        if x.len() < 2 || g.len() < 2 {
            panic!("Invalid input size");
        }
        let r = (x[0] * x[0] + x[1] * x[1]).sqrt();
        if r > 1e-10 {
            g[0] = x[0] / r;
            g[1] = x[1] / r;
        }
    }

    /// Evaluate Hessian.
    pub fn hessian(&self, x: &[f64], h: &mut [[f64; 2]; 2]) {
        if x.len() < 2 {
            panic!("Invalid input size");
        }
        let r = (x[0] * x[0] + x[1] * x[1]).sqrt();
        let r3 = r * r * r;
        if r > 1e-10 {
            h[0][0] = x[1] * x[1] / r3;
            h[1][1] = x[0] * x[0] / r3;
            h[0][1] = -x[0] * x[1] / r3;
            h[1][0] = h[0][1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_c0_value() {
        let func = ExtremaGlobOptFuncCc0::new();
        let x = [3.0, 4.0];
        assert!((func.value(&x) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_func_c1_gradient() {
        let func = ExtremaGlobOptFuncCc1::new();
        let x = [3.0, 4.0];
        let mut g = [0.0; 2];
        func.gradient(&x, &mut g);
        assert!((g[0] - 0.6).abs() < 0.01);
        assert!((g[1] - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_func_c2_hessian() {
        let func = ExtremaGlobOptFuncCc2::new();
        let x = [3.0, 4.0];
        let mut h = [[0.0; 2]; 2];
        func.hessian(&x, &mut h);
        assert!(h[0][0] > 0.0);
    }
}
