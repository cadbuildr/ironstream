// FILE: hlr_topo_b_rep_v_data.rs
// occt: HLRTopoBRep_VData

/// Data structure for a vertex in HLR topology.
#[derive(Clone, Debug)]
pub struct HLRTopoBRepVData {
    x: f64,
    y: f64,
    z: f64,
    normal_x: f64,
    normal_y: f64,
    normal_z: f64,
}

impl HLRTopoBRepVData {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        HLRTopoBRepVData {
            x,
            y,
            z,
            normal_x: 0.0,
            normal_y: 0.0,
            normal_z: 1.0,
        }
    }

    pub fn position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn normal(&self) -> (f64, f64, f64) {
        (self.normal_x, self.normal_y, self.normal_z)
    }

    pub fn set_normal(&mut self, nx: f64, ny: f64, nz: f64) {
        self.normal_x = nx;
        self.normal_y = ny;
        self.normal_z = nz;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let v = HLRTopoBRepVData::new(1.0, 2.0, 3.0);
        assert_eq!(v.position(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_normal() {
        let mut v = HLRTopoBRepVData::new(0.0, 0.0, 0.0);
        v.set_normal(0.0, 0.0, 1.0);
        assert_eq!(v.normal(), (0.0, 0.0, 1.0));
    }
}
