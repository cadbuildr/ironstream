// FILE: adv_approx_cutting.rs
// occt: AdvApprox_Cutting

pub struct AdvApproxCutting {
    tolerance: f64,
}

impl AdvApproxCutting {
    pub fn new(tolerance: f64) -> Self {
        AdvApproxCutting { tolerance }
    }

    pub fn get_tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn cut(&self, u_min: f64, u_max: f64) -> Vec<(f64, f64)> {
        vec![(u_min, u_max)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_cutting_new() {
        let cutting = AdvApproxCutting::new(1e-7);
        assert!((cutting.get_tolerance() - 1e-7).abs() < PRECISION);
    }

    #[test]
    fn test_cutting_cut() {
        let cutting = AdvApproxCutting::new(1e-7);
        let result = cutting.cut(0.0, 1.0);
        assert_eq!(result.len(), 1);
        assert!((result[0].0 - 0.0).abs() < PRECISION);
        assert!((result[0].1 - 1.0).abs() < PRECISION);
    }
}
