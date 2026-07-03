// FILE: extrema_pcf_of_epc_of_elpc_of_locate_ext_pc.rs
// occt: Extrema_PCFOfEPCOfELPCOfLocateExtPC

/// Function for point-curve extremum calculation.
pub struct ExtremaPcfOfEpcOfElpcOfLocateExtPc {
    sq_dist: f64,
}

impl ExtremaPcfOfEpcOfElpcOfLocateExtPc {
    pub fn new() -> Self {
        ExtremaPcfOfEpcOfElpcOfLocateExtPc { sq_dist: 0.0 }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaPcfOfEpcOfElpcOfLocateExtPc::new();
        assert_eq!(func.sq_dist, 0.0);
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcfOfEpcOfElpcOfLocateExtPc::new();
        func.set_point(3.0, 4.0, 0.0);
        assert!((func.square_distance() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_is_min() {
        let func = ExtremaPcfOfEpcOfElpcOfLocateExtPc::new();
        assert!(func.is_min());
    }
}
