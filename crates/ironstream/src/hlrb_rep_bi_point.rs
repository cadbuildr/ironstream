// FILE: hlrb_rep_bi_point.rs
// occt: HLRBRep_BiPoint

#[derive(Clone)]
pub struct HlrbrépBiPoint {
    x1: f64, y1: f64, z1: f64,
    x2: f64, y2: f64, z2: f64,
    xt1: f64, yt1: f64, zt1: f64,
    xt2: f64, yt2: f64, zt2: f64,
    shape_index: i32,
    flags: i32,
}

impl HlrbrépBiPoint {
    pub fn new() -> Self {
        HlrbrépBiPoint {
            x1: 0.0, y1: 0.0, z1: 0.0, x2: 0.0, y2: 0.0, z2: 0.0,
            xt1: 0.0, yt1: 0.0, zt1: 0.0, xt2: 0.0, yt2: 0.0, zt2: 0.0,
            shape_index: 0, flags: 0,
        }
    }

    pub fn pnt1(&self) -> [f64; 3] { [self.x1, self.y1, self.z1] }
    pub fn pnt2(&self) -> [f64; 3] { [self.x2, self.y2, self.z2] }
    pub fn pnt_p1(&self) -> [f64; 3] { [self.xt1, self.yt1, self.zt1] }
    pub fn pnt_p2(&self) -> [f64; 3] { [self.xt2, self.yt2, self.zt2] }
    pub fn shape_index(&self) -> i32 { self.shape_index }
    pub fn set_shape_index(&mut self, i: i32) { self.shape_index = i; }
    pub fn set_flags(&mut self, f: i32) { self.flags = f; }
    pub fn flags(&self) -> i32 { self.flags }
}

impl Default for HlrbrépBiPoint {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let bp = HlrbrépBiPoint::new();
        assert_eq!(bp.shape_index(), 0);
    }
}
