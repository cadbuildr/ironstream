// FILE: ais_animation_axis_rotation.rs
// occt: AIS_AnimationAxisRotation

#[derive(Clone, Debug)]
pub struct AnimationAxisRotation {
    axis: [f64; 3],
    angle_start: f64,
    angle_end: f64,
    duration: f64,
}

impl AnimationAxisRotation {
    pub fn new(axis: [f64; 3]) -> Self {
        Self { axis, angle_start: 0.0, angle_end: 0.0, duration: 1.0 }
    }
    pub fn axis(&self) -> [f64; 3] { self.axis }
    pub fn angle_start(&self) -> f64 { self.angle_start }
    pub fn angle_end(&self) -> f64 { self.angle_end }
    pub fn duration(&self) -> f64 { self.duration }
    pub fn set_angles(&mut self, start: f64, end: f64) { self.angle_start = start; self.angle_end = end; }
    pub fn set_duration(&mut self, d: f64) { self.duration = d.max(0.01); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = AnimationAxisRotation::new([0.0, 0.0, 1.0]);
        assert_eq!(a.axis(), [0.0, 0.0, 1.0]);
    }
}
