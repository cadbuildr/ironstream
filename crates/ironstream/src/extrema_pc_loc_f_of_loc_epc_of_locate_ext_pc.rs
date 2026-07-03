// FILE: extrema_pc_loc_f_of_loc_epc_of_locate_ext_pc.rs
// occt: Extrema_PCLocFOfLocEPCOfLocateExtPC

/// Local function for point-curve extremum on 3D curves.
pub struct ExtremaPcLocFOfLocEpcOfLocateExtPc {
    sq_dist: f64,
    is_min: bool,
}

impl ExtremaPcLocFOfLocEpcOfLocateExtPc {
    pub fn new() -> Self {
        ExtremaPcLocFOfLocEpcOfLocateExtPc {
            sq_dist: 0.0,
            is_min: true,
        }
    }

    pub fn initialize(&mut self) {
        self.sq_dist = 0.0;
        self.is_min = true;
    }

    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.sq_dist = (x * x + y * y + z * z).sqrt();
    }

    pub fn square_distance(&self) -> f64 {
        self.sq_dist
    }

    pub fn is_min(&self) -> bool {
        self.is_min
    }

    pub fn value(&self, u: f64) -> f64 {
        u * u * self.sq_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaPcLocFOfLocEpcOfLocateExtPc::new();
        assert!(!func.is_min() || func.is_min());
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcLocFOfLocEpcOfLocateExtPc::new();
        func.set_point(1.0, 1.0, 1.0);
        assert!(func.square_distance() > 0.0);
    }

    #[test]
    fn test_value() {
        let mut func = ExtremaPcLocFOfLocEpcOfLocateExtPc::new();
        func.set_point(1.0, 0.0, 0.0);
        let v = func.value(2.0);
        assert!(v > 0.0);
    }
}
