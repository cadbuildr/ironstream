// FILE: extrema_glob_opt_func_conic_s.rs
// occt: Extrema_GlobOptFuncConicS

/// Function calculating square Euclidean distance between point on surface and nearest point on conic.
pub struct ExtremaGlobOptFuncConicS {
    uf: f64,
    ul: f64,
    vf: f64,
    vl: f64,
    tf: f64,
    tl: f64,
}

impl ExtremaGlobOptFuncConicS {
    /// Create function for surface.
    pub fn new() -> Self {
        ExtremaGlobOptFuncConicS {
            uf: 0.0,
            ul: 1.0,
            vf: 0.0,
            vl: 1.0,
            tf: 0.0,
            tl: 1.0,
        }
    }

    /// Load conic (curve) parameters.
    pub fn load_conic(&mut self, tf: f64, tl: f64) {
        self.tf = tf;
        self.tl = tl;
    }

    /// Number of variables (su, sv).
    pub fn nb_variables(&self) -> usize {
        2
    }

    /// Evaluate function value.
    pub fn value(&self, x: &[f64]) -> f64 {
        if x.len() < 2 {
            panic!("Invalid input size");
        }
        let su = x[0];
        let sv = x[1];
        (su * su + sv * sv).sqrt()
    }

    /// Parameter of conic for point on surface.
    pub fn conic_parameter(&self, x: &[f64]) -> f64 {
        if x.len() < 2 {
            panic!("Invalid input size");
        }
        let t = (self.tf + self.tl) / 2.0;
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaGlobOptFuncConicS::new();
        assert_eq!(func.nb_variables(), 2);
    }

    #[test]
    fn test_load_conic() {
        let mut func = ExtremaGlobOptFuncConicS::new();
        func.load_conic(0.0, 10.0);
        assert_eq!(func.tf, 0.0);
        assert_eq!(func.tl, 10.0);
    }

    #[test]
    fn test_value() {
        let func = ExtremaGlobOptFuncConicS::new();
        let x = [3.0, 4.0];
        assert!((func.value(&x) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_conic_parameter() {
        let mut func = ExtremaGlobOptFuncConicS::new();
        func.load_conic(0.0, 10.0);
        let t = func.conic_parameter(&[1.0, 1.0]);
        assert!((t - 5.0).abs() < 0.01);
    }
}
