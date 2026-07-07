// FILE: vrml_converter_shading_aspect.rs
// occt: VrmlConverter_ShadingAspect

#[derive(Clone, Debug)]
pub struct VrmlConverterShadingAspect {
    shininess: f32,
}

impl VrmlConverterShadingAspect {
    pub fn new(shininess: f32) -> Self {
        VrmlConverterShadingAspect { shininess }
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let aspect = VrmlConverterShadingAspect::new(0.8);
        assert_eq!(aspect.shininess(), 0.8);
    }
}
