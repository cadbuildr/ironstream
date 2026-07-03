// FILE: ais_light_source.rs
// occt: AIS_LightSource

#[derive(Clone, Debug)]
pub struct LightSource {
    light_type: u32,
    position: [f64; 3],
    direction: [f64; 3],
    intensity: f32,
}

impl LightSource {
    pub fn new(light_type: u32) -> Self {
        Self { light_type, position: [0.0; 3], direction: [0.0, 0.0, -1.0], intensity: 1.0 }
    }
    pub fn light_type(&self) -> u32 { self.light_type }
    pub fn position(&self) -> [f64; 3] { self.position }
    pub fn direction(&self) -> [f64; 3] { self.direction }
    pub fn intensity(&self) -> f32 { self.intensity }
    pub fn set_position(&mut self, p: [f64; 3]) { self.position = p; }
    pub fn set_direction(&mut self, d: [f64; 3]) { self.direction = d; }
    pub fn set_intensity(&mut self, i: f32) { self.intensity = i.max(0.0); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let l = LightSource::new(0);
        assert_eq!(l.light_type(), 0);
    }
}
