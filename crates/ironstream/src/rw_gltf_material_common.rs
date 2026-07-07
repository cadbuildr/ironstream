// FILE: rw_gltf_material_common.rs
// occt: RWGltf_MaterialCommon

//! Common material properties for glTF.

#[derive(Debug, Clone)]
pub struct MaterialCommon {
    ambient: (f32, f32, f32),
    diffuse: (f32, f32, f32),
    specular: (f32, f32, f32),
    shininess: f32,
}

impl MaterialCommon {
    pub fn new() -> Self {
        Self {
            ambient: (0.2, 0.2, 0.2),
            diffuse: (0.8, 0.8, 0.8),
            specular: (1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }

    pub fn diffuse(&self) -> (f32, f32, f32) {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, r: f32, g: f32, b: f32) {
        self.diffuse = (r, g, b);
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    pub fn set_shininess(&mut self, s: f32) {
        self.shininess = s;
    }
}

impl Default for MaterialCommon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mat = MaterialCommon::new();
        assert_eq!(mat.shininess(), 32.0);
    }
}
