// FILE: prs3d_arrow_aspect.rs
// occt: Prs3d_ArrowAspect

#[derive(Clone, Debug)]
pub struct Prs3dArrowAspect {
    pub aspect_id: u32,
    pub angle: f64,
    pub length: f64,
}

impl Prs3dArrowAspect {
    pub fn new(aspect_id: u32) -> Self {
        Self { aspect_id, angle: std::f64::consts::PI / 9.0, length: 1.0 }
    }
    pub fn set_angle(&mut self, angle: f64) { self.angle = angle; }
    pub fn set_length(&mut self, len: f64) { self.length = len.max(0.0); }
}

impl Default for Prs3dArrowAspect {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let a = Prs3dArrowAspect::new(1);
        assert_eq!(a.aspect_id, 1);
    }
}
