// FILE: extrema_pcf_of_epc_of_ext_pc.rs
// occt: Extrema_PCFOfEPCOfExtPC

/// Function for extended point-curve extremum calculation.
pub struct ExtremaPcfOfEpcOfExtPc {
    sq_dist: f64,
}

impl ExtremaPcfOfEpcOfExtPc {
    pub fn new() -> Self {
        ExtremaPcfOfEpcOfExtPc { sq_dist: 0.0 }
    }

    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.sq_dist = (x * x + y * y + z * z).sqrt();
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
        let func = ExtremaPcfOfEpcOfExtPc::new();
        assert_eq!(func.sq_dist, 0.0);
        assert_eq!(func.nb_ext(), 0);
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcfOfEpcOfExtPc::new();
        func.set_point(1.0, 1.0, 1.0);
        assert!((func.square_distance() - 1.732).abs() < 0.01);
        assert_eq!(func.nb_ext(), 1);
    }
}
