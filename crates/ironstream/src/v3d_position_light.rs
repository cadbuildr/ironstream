// FILE: v3d_position_light.rs
// occt: V3d_PositionLight

//! A light source positioned in 3D space.

#[derive(Clone, Debug)]
pub struct PositionLight {
    pub light_id: u32,
    pub position: [f64; 3],
    pub is_enabled: bool,
    pub intensity: f64,
    pub range: f64,
}

impl PositionLight {
    pub fn new(light_id: u32, position: [f64; 3]) -> Self {
        PositionLight {
            light_id,
            position,
            is_enabled: true,
            intensity: 1.0,
            range: 100.0,
        }
    }

    pub fn with_intensity(mut self, intensity: f64) -> Self {
        self.intensity = intensity.clamp(0.0, 2.0);
        self
    }

    pub fn with_range(mut self, range: f64) -> Self {
        self.range = range.max(0.1);
        self
    }

    pub fn set_position(&mut self, pos: [f64; 3]) {
        self.position = pos;
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn distance_to_point(&self, p: [f64; 3]) -> f64 {
        let dx = p[0] - self.position[0];
        let dy = p[1] - self.position[1];
        let dz = p[2] - self.position[2];
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_light_creation() {
        let light = PositionLight::new(1, [1.0, 2.0, 3.0]);
        assert_eq!(light.light_id, 1);
        assert!(light.is_enabled);
        assert_eq!(light.intensity, 1.0);
        assert_eq!(light.range, 100.0);
    }

    #[test]
    fn position_light_builder() {
        let light = PositionLight::new(1, [0.0, 0.0, 10.0])
            .with_intensity(1.5)
            .with_range(50.0);

        assert_eq!(light.intensity, 1.5);
        assert_eq!(light.range, 50.0);
    }

    #[test]
    fn position_light_distance() {
        let light = PositionLight::new(1, [0.0, 0.0, 0.0]);
        let dist = light.distance_to_point([3.0, 4.0, 0.0]);
        assert!((dist - 5.0).abs() < 1e-10);
    }
}
