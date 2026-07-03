// FILE: adaptor3d_inter_func.rs
// occt: Adaptor3d_InterFunc

pub struct Adaptor3dInterFunc {
    tolerance: f64,
}

impl Adaptor3dInterFunc {
    pub fn new(tolerance: f64) -> Self {
        Adaptor3dInterFunc { tolerance }
    }

    pub fn get_tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn evaluate(&self, _t: f64) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_inter_func_new() {
        let func = Adaptor3dInterFunc::new(1e-7);
        assert!((func.get_tolerance() - 1e-7).abs() < PRECISION);
    }

    #[test]
    fn test_inter_func_evaluate() {
        let func = Adaptor3dInterFunc::new(1e-7);
        assert_eq!(func.evaluate(0.5), 0.0);
    }
}
