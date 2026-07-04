// FILE: step_visual_surface_style_reflectance_ambient_diffuse.rs
// occt: StepVisual_SurfaceStyleReflectanceAmbientDiffuse

pub struct SurfaceStyleReflectanceAmbientDiffuse {
    ambient_reflectance: f64,
    diffuse_reflectance: f64,
}

impl SurfaceStyleReflectanceAmbientDiffuse {
    pub fn new() -> Self {
        SurfaceStyleReflectanceAmbientDiffuse {
            ambient_reflectance: 0.0,
            diffuse_reflectance: 0.0,
        }
    }

    pub fn init(&mut self, ambient_reflectance: f64, diffuse_reflectance: f64) {
        self.ambient_reflectance = ambient_reflectance;
        self.diffuse_reflectance = diffuse_reflectance;
    }

    pub fn ambient_reflectance(&self) -> f64 {
        self.ambient_reflectance
    }

    pub fn set_ambient_reflectance(&mut self, value: f64) {
        self.ambient_reflectance = value;
    }

    pub fn diffuse_reflectance(&self) -> f64 {
        self.diffuse_reflectance
    }

    pub fn set_diffuse_reflectance(&mut self, value: f64) {
        self.diffuse_reflectance = value;
    }
}

impl Default for SurfaceStyleReflectanceAmbientDiffuse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssrad = SurfaceStyleReflectanceAmbientDiffuse::new();
        assert_eq!(ssrad.ambient_reflectance(), 0.0);
        assert_eq!(ssrad.diffuse_reflectance(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut ssrad = SurfaceStyleReflectanceAmbientDiffuse::new();
        ssrad.init(0.5, 0.75);
        assert_eq!(ssrad.ambient_reflectance(), 0.5);
        assert_eq!(ssrad.diffuse_reflectance(), 0.75);
    }

    #[test]
    fn test_set_and_get_diffuse_reflectance() {
        let mut ssrad = SurfaceStyleReflectanceAmbientDiffuse::new();
        ssrad.set_diffuse_reflectance(0.6);
        assert_eq!(ssrad.diffuse_reflectance(), 0.6);
    }

    #[test]
    fn test_both_reflectances() {
        let mut ssrad = SurfaceStyleReflectanceAmbientDiffuse::new();
        ssrad.set_ambient_reflectance(0.3);
        ssrad.set_diffuse_reflectance(0.7);
        assert_eq!(ssrad.ambient_reflectance(), 0.3);
        assert_eq!(ssrad.diffuse_reflectance(), 0.7);
    }
}
