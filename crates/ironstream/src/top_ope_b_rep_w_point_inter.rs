// FILE: top_ope_b_rep_w_point_inter.rs
// occt: TopOpeBRep_WPointInter

/// Walking point of intersection.
pub struct WPointInter {
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl WPointInter {
    /// Create a new WPointInter.
    pub fn new() -> Self {
        WPointInter {
            u1: 0.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Set the point from IntSurf_PntOn2S.
    pub fn set(&mut self, _p: &[u8]) {
        // Implementation stub
    }

    /// Get parameters on surface 1.
    pub fn parameters_on_s1(&self) -> (f64, f64) {
        (self.u1, self.v1)
    }

    /// Get parameters on surface 2.
    pub fn parameters_on_s2(&self) -> (f64, f64) {
        (self.u2, self.v2)
    }

    /// Get all parameters.
    pub fn parameters(&self) -> (f64, f64, f64, f64) {
        (self.u1, self.v1, self.u2, self.v2)
    }

    /// Get value as 2D point on surface 1.
    pub fn value_on_s1(&self) -> (f64, f64) {
        (self.u1, self.v1)
    }

    /// Get value as 2D point on surface 2.
    pub fn value_on_s2(&self) -> (f64, f64) {
        (self.u2, self.v2)
    }

    /// Get value as 3D point.
    pub fn value(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl Default for WPointInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = WPointInter::new();
        assert_eq!(pt.parameters(), (0.0, 0.0, 0.0, 0.0));
        assert_eq!(pt.value(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_parameters_on_s1() {
        let mut pt = WPointInter::new();
        pt.u1 = 1.5;
        pt.v1 = 2.5;
        assert_eq!(pt.parameters_on_s1(), (1.5, 2.5));
    }

    #[test]
    fn test_parameters_on_s2() {
        let mut pt = WPointInter::new();
        pt.u2 = 3.5;
        pt.v2 = 4.5;
        assert_eq!(pt.parameters_on_s2(), (3.5, 4.5));
    }

    #[test]
    fn test_value() {
        let mut pt = WPointInter::new();
        pt.x = 1.0;
        pt.y = 2.0;
        pt.z = 3.0;
        assert_eq!(pt.value(), (1.0, 2.0, 3.0));
    }
}
