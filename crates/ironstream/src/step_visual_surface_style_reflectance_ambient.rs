// FILE: step_visual_surface_style_reflectance_ambient.rs
// occt: StepVisual_SurfaceStyleReflectanceAmbient

pub struct SurfaceStyleReflectanceAmbient {
    ambient_reflectance: f64,
}

impl SurfaceStyleReflectanceAmbient {
    pub fn new() -> Self {
        SurfaceStyleReflectanceAmbient {
            ambient_reflectance: 0.0,
        }
    }

    pub fn init(&mut self, ambient_reflectance: f64) {
        self.ambient_reflectance = ambient_reflectance;
    }

    pub fn ambient_reflectance(&self) -> f64 {
        self.ambient_reflectance
    }

    pub fn set_ambient_reflectance(&mut self, value: f64) {
        self.ambient_reflectance = value;
    }
}

impl Default for SurfaceStyleReflectanceAmbient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssra = SurfaceStyleReflectanceAmbient::new();
        assert_eq!(ssra.ambient_reflectance(), 0.0);
    }

    #[test]
    fn test_set_and_get_ambient_reflectance() {
        let mut ssra = SurfaceStyleReflectanceAmbient::new();
        ssra.set_ambient_reflectance(0.5);
        assert_eq!(ssra.ambient_reflectance(), 0.5);
    }

    #[test]
    fn test_init() {
        let mut ssra = SurfaceStyleReflectanceAmbient::new();
        ssra.init(0.75);
        assert_eq!(ssra.ambient_reflectance(), 0.75);
    }

    #[test]
    fn test_reflectance_range() {
        let mut ssra = SurfaceStyleReflectanceAmbient::new();
        ssra.set_ambient_reflectance(1.0);
        assert_eq!(ssra.ambient_reflectance(), 1.0);

        ssra.set_ambient_reflectance(0.0);
        assert_eq!(ssra.ambient_reflectance(), 0.0);
    }
}
