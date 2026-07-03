// FILE: extrema_glob_opt_func_cs.rs
// occt: Extrema_GlobOptFuncCS

/// Function calculating square Euclidean distance between point on curve and point on surface (C2).
pub struct ExtremaGlobOptFuncCs {
    nb_vars: usize,
}

impl ExtremaGlobOptFuncCs {
    /// Create function for curve and surface.
    pub fn new() -> Self {
        ExtremaGlobOptFuncCs { nb_vars: 3 }
    }

    /// Number of variables (cu, su, sv).
    pub fn nb_variables(&self) -> usize {
        self.nb_vars
    }

    /// Evaluate function value.
    pub fn value(&self, x: &[f64]) -> f64 {
        if x.len() < 3 {
            panic!("Invalid input size");
        }
        let cu = x[0];
        let su = x[1];
        let sv = x[2];
        (cu * cu + su * su + sv * sv).sqrt()
    }

    /// Evaluate gradient.
    pub fn gradient(&self, x: &[f64], g: &mut [f64]) {
        if x.len() < 3 || g.len() < 3 {
            panic!("Invalid input size");
        }
        let cu = x[0];
        let su = x[1];
        let sv = x[2];
        let r = (cu * cu + su * su + sv * sv).sqrt();
        if r > 1e-10 {
            g[0] = cu / r;
            g[1] = su / r;
            g[2] = sv / r;
        }
    }

    /// Evaluate Hessian.
    pub fn hessian(&self, x: &[f64], h: &mut [[f64; 3]; 3]) {
        if x.len() < 3 {
            panic!("Invalid input size");
        }
        let cu = x[0];
        let su = x[1];
        let sv = x[2];
        let r = (cu * cu + su * su + sv * sv).sqrt();
        let r3 = r * r * r;
        if r > 1e-10 {
            h[0][0] = (su * su + sv * sv) / r3;
            h[1][1] = (cu * cu + sv * sv) / r3;
            h[2][2] = (cu * cu + su * su) / r3;
            h[0][1] = -cu * su / r3;
            h[0][2] = -cu * sv / r3;
            h[1][2] = -su * sv / r3;
            h[1][0] = h[0][1];
            h[2][0] = h[0][2];
            h[2][1] = h[1][2];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaGlobOptFuncCs::new();
        assert_eq!(func.nb_variables(), 3);
    }

    #[test]
    fn test_value() {
        let func = ExtremaGlobOptFuncCs::new();
        let x = [2.0, 2.0, 1.0];
        assert!((func.value(&x) - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_gradient() {
        let func = ExtremaGlobOptFuncCs::new();
        let x = [2.0, 2.0, 1.0];
        let mut g = [0.0; 3];
        func.gradient(&x, &mut g);
        assert!((g[0] - (2.0 / 3.0)).abs() < 0.01);
    }

    #[test]
    fn test_hessian() {
        let func = ExtremaGlobOptFuncCs::new();
        let x = [2.0, 2.0, 1.0];
        let mut h = [[0.0; 3]; 3];
        func.hessian(&x, &mut h);
        assert!(h[0][0] > 0.0);
    }
}
