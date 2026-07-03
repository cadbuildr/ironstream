// FILE: adv_approx_dicho_cutting.rs
// occt: AdvApprox_DichoCutting

pub struct AdvApproxDichoCutting {
    tolerance: f64,
}

impl AdvApproxDichoCutting {
    pub fn new(tolerance: f64) -> Self {
        AdvApproxDichoCutting { tolerance }
    }

    pub fn get_tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Perform dichotomic cutting of a parameter interval.
    pub fn cut(&self, u_min: f64, u_max: f64) -> Vec<(f64, f64)> {
        let mid = (u_min + u_max) / 2.0;
        vec![(u_min, mid), (mid, u_max)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_dicho_cutting_new() {
        let cutting = AdvApproxDichoCutting::new(1e-7);
        assert!((cutting.get_tolerance() - 1e-7).abs() < PRECISION);
    }

    #[test]
    fn test_dicho_cutting_cut() {
        let cutting = AdvApproxDichoCutting::new(1e-7);
        let result = cutting.cut(0.0, 1.0);
        assert_eq!(result.len(), 2);
        assert!((result[0].0 - 0.0).abs() < PRECISION);
        assert!((result[0].1 - 0.5).abs() < PRECISION);
        assert!((result[1].0 - 0.5).abs() < PRECISION);
        assert!((result[1].1 - 1.0).abs() < PRECISION);
    }
}
