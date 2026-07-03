// FILE: extrema_pc_loc_f_of_loc_epc_of_locate_ext_pc2d.rs
// occt: Extrema_PCLocFOfLocEPCOfLocateExtPC2d

/// Local function for point-curve extremum on 2D curves.
pub struct ExtremaPcLocFOfLocEpcOfLocateExtPc2d {
    sq_dist: f64,
    is_min: bool,
}

impl ExtremaPcLocFOfLocEpcOfLocateExtPc2d {
    pub fn new() -> Self {
        ExtremaPcLocFOfLocEpcOfLocateExtPc2d {
            sq_dist: 0.0,
            is_min: true,
        }
    }

    pub fn initialize(&mut self) {
        self.sq_dist = 0.0;
        self.is_min = true;
    }

    pub fn set_point(&mut self, x: f64, y: f64) {
        self.sq_dist = (x * x + y * y).sqrt();
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
        let func = ExtremaPcLocFOfLocEpcOfLocateExtPc2d::new();
        assert!(func.is_min());
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcLocFOfLocEpcOfLocateExtPc2d::new();
        func.set_point(1.0, 1.0);
        assert!(func.square_distance() > 0.0);
    }
}
