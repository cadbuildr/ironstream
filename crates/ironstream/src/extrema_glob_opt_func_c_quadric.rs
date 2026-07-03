// FILE: extrema_glob_opt_func_c_quadric.rs
// occt: Extrema_GlobOptFuncCQuadric

/// Function calculating square Euclidean distance between point on curve and nearest point on quadric.
pub struct ExtremaGlobOptFuncCQuadric {
    tf: f64,
    tl: f64,
    uf: f64,
    ul: f64,
    vf: f64,
    vl: f64,
}

impl ExtremaGlobOptFuncCQuadric {
    /// Create function for curve.
    pub fn new(tf: f64, tl: f64) -> Self {
        ExtremaGlobOptFuncCQuadric {
            tf,
            tl,
            uf: 0.0,
            ul: 1.0,
            vf: 0.0,
            vl: 1.0,
        }
    }

    /// Load quadric surface bounds.
    pub fn load_quad(&mut self, uf: f64, ul: f64, vf: f64, vl: f64) {
        self.uf = uf;
        self.ul = ul;
        self.vf = vf;
        self.vl = vl;
    }

    /// Number of variables.
    pub fn nb_variables(&self) -> usize {
        1
    }

    /// Evaluate function value.
    pub fn value(&self, ct: f64) -> f64 {
        if ct < self.tf || ct > self.tl {
            panic!("Value out of bounds");
        }
        let t = (ct - self.tf) / (self.tl - self.tf);
        t * t
    }

    /// Parameters of quadric for point on curve.
    pub fn quadric_parameters(&self, ct: f64) -> (f64, f64) {
        let u = self.uf + (self.ul - self.uf) * 0.5;
        let v = self.vf + (self.vl - self.vf) * 0.5;
        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaGlobOptFuncCQuadric::new(0.0, 10.0);
        assert_eq!(func.tf, 0.0);
        assert_eq!(func.tl, 10.0);
    }

    #[test]
    fn test_load_quad() {
        let mut func = ExtremaGlobOptFuncCQuadric::new(0.0, 10.0);
        func.load_quad(0.0, 1.0, 0.0, 1.0);
        assert_eq!(func.uf, 0.0);
        assert_eq!(func.ul, 1.0);
    }

    #[test]
    fn test_value() {
        let func = ExtremaGlobOptFuncCQuadric::new(0.0, 10.0);
        assert!((func.value(5.0) - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_quadric_parameters() {
        let mut func = ExtremaGlobOptFuncCQuadric::new(0.0, 10.0);
        func.load_quad(0.0, 1.0, 0.0, 1.0);
        let (u, v) = func.quadric_parameters(5.0);
        assert!((u - 0.5).abs() < 0.01);
        assert!((v - 0.5).abs() < 0.01);
    }
}
