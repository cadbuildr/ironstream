// FILE: extrema_pcf_of_epc_of_ext_pc2d.rs
// occt: Extrema_PCFOfEPCOfExtPC2d

/// Function for extended 2D point-curve extremum calculation.
pub struct ExtremaPcfOfEpcOfExtPc2d {
    sq_dist: f64,
}

impl ExtremaPcfOfEpcOfExtPc2d {
    pub fn new() -> Self {
        ExtremaPcfOfEpcOfExtPc2d { sq_dist: 0.0 }
    }

    pub fn set_point(&mut self, x: f64, y: f64) {
        self.sq_dist = (x * x + y * y).sqrt();
    }

    pub fn square_distance(&self) -> f64 {
        self.sq_dist
    }

    pub fn is_min(&self) -> bool {
        true
    }

    pub fn nb_ext(&self) -> usize {
        if self.sq_dist > 0.0 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaPcfOfEpcOfExtPc2d::new();
        assert_eq!(func.sq_dist, 0.0);
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcfOfEpcOfExtPc2d::new();
        func.set_point(1.0, 1.0);
        assert!((func.square_distance() - 1.414).abs() < 0.01);
    }
}
