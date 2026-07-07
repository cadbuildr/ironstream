// FILE: step_visual_surface_style_reflectance_ambient_diffuse_specular.rs
// occt: StepVisual_SurfaceStyleReflectanceAmbientDiffuseSpecular

use std::sync::Arc;

pub struct Colour;

pub struct SurfaceStyleReflectanceAmbientDiffuseSpecular {
    ambient_reflectance: f64,
    diffuse_reflectance: f64,
    specular_reflectance: f64,
    specular_exponent: f64,
    specular_colour: Option<Arc<Colour>>,
}

impl SurfaceStyleReflectanceAmbientDiffuseSpecular {
    pub fn new() -> Self {
        SurfaceStyleReflectanceAmbientDiffuseSpecular {
            ambient_reflectance: 0.0,
            diffuse_reflectance: 0.0,
            specular_reflectance: 0.0,
            specular_exponent: 0.0,
            specular_colour: None,
        }
    }

    pub fn init(
        &mut self,
        ambient_reflectance: f64,
        diffuse_reflectance: f64,
        specular_reflectance: f64,
        specular_exponent: f64,
        specular_colour: Option<Arc<Colour>>,
    ) {
        self.ambient_reflectance = ambient_reflectance;
        self.diffuse_reflectance = diffuse_reflectance;
        self.specular_reflectance = specular_reflectance;
        self.specular_exponent = specular_exponent;
        self.specular_colour = specular_colour;
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

    pub fn specular_reflectance(&self) -> f64 {
        self.specular_reflectance
    }

    pub fn set_specular_reflectance(&mut self, value: f64) {
        self.specular_reflectance = value;
    }

    pub fn specular_exponent(&self) -> f64 {
        self.specular_exponent
    }

    pub fn set_specular_exponent(&mut self, value: f64) {
        self.specular_exponent = value;
    }

    pub fn specular_colour(&self) -> Option<&Arc<Colour>> {
        self.specular_colour.as_ref()
    }

    pub fn set_specular_colour(&mut self, colour: Option<Arc<Colour>>) {
        self.specular_colour = colour;
    }
}

impl Default for SurfaceStyleReflectanceAmbientDiffuseSpecular {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssrads = SurfaceStyleReflectanceAmbientDiffuseSpecular::new();
        assert_eq!(ssrads.ambient_reflectance(), 0.0);
        assert_eq!(ssrads.diffuse_reflectance(), 0.0);
        assert_eq!(ssrads.specular_reflectance(), 0.0);
        assert_eq!(ssrads.specular_exponent(), 0.0);
        assert!(ssrads.specular_colour().is_none());
    }

    #[test]
    fn test_init() {
        let mut ssrads = SurfaceStyleReflectanceAmbientDiffuseSpecular::new();
        let colour = Arc::new(Colour);
        ssrads.init(0.2, 0.5, 0.8, 128.0, Some(colour));
        assert_eq!(ssrads.ambient_reflectance(), 0.2);
        assert_eq!(ssrads.diffuse_reflectance(), 0.5);
        assert_eq!(ssrads.specular_reflectance(), 0.8);
        assert_eq!(ssrads.specular_exponent(), 128.0);
        assert!(ssrads.specular_colour().is_some());
    }

    #[test]
    fn test_setters() {
        let mut ssrads = SurfaceStyleReflectanceAmbientDiffuseSpecular::new();
        ssrads.set_ambient_reflectance(0.1);
        ssrads.set_diffuse_reflectance(0.4);
        ssrads.set_specular_reflectance(0.9);
        ssrads.set_specular_exponent(64.0);
        assert_eq!(ssrads.ambient_reflectance(), 0.1);
        assert_eq!(ssrads.diffuse_reflectance(), 0.4);
        assert_eq!(ssrads.specular_reflectance(), 0.9);
        assert_eq!(ssrads.specular_exponent(), 64.0);
    }
}
