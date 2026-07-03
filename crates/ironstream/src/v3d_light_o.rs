// FILE: v3d_light_o.rs
// occt: V3d_Light

//! Base class for lights in 3D views.

#[derive(Clone, Debug)]
pub struct Light {
    pub light_id: u32,
    pub is_enabled: bool,
    pub intensity: f64,
}

impl Light {
    pub fn new(light_id: u32) -> Self {
        Light {
            light_id,
            is_enabled: true,
            intensity: 1.0,
        }
    }

    pub fn with_intensity(mut self, intensity: f64) -> Self {
        self.intensity = intensity.clamp(0.0, 2.0);
        self
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_creation() {
        let light = Light::new(1);
        assert_eq!(light.light_id, 1);
        assert!(light.is_enabled);
        assert_eq!(light.intensity, 1.0);
    }

    #[test]
    fn light_builder() {
        let light = Light::new(2).with_intensity(1.5);
        assert_eq!(light.intensity, 1.5);
    }

    #[test]
    fn light_enable_disable() {
        let mut light = Light::new(1);
        light.disable();
        assert!(!light.is_enabled());
        light.enable();
        assert!(light.is_enabled());
    }
}
