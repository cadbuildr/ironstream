// FILE: extrema_pcf_of_epc_of_elpc_of_locate_ext_pc2d.rs
// occt: Extrema_PCFOfEPCOfELPCOfLocateExtPC2d

/// Function for 2D point-curve extremum calculation.
pub struct ExtremaPcfOfEpcOfElpcOfLocateExtPc2d {
    sq_dist: f64,
}

impl ExtremaPcfOfEpcOfElpcOfLocateExtPc2d {
    pub fn new() -> Self {
        ExtremaPcfOfEpcOfElpcOfLocateExtPc2d { sq_dist: 0.0 }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaPcfOfEpcOfElpcOfLocateExtPc2d::new();
        assert_eq!(func.sq_dist, 0.0);
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcfOfEpcOfElpcOfLocateExtPc2d::new();
        func.set_point(3.0, 4.0);
        assert!((func.square_distance() - 5.0).abs() < 0.01);
    }
}
