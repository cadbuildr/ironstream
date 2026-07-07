// FILE: rw_gltf_material_metallic_roughness.rs
// occt: RWGltf_MaterialMetallicRoughness

//! Metallic-roughness PBR material for glTF.

#[derive(Debug, Clone)]
pub struct MaterialMetallicRoughness {
    base_color: (f32, f32, f32, f32),
    metallic: f32,
    roughness: f32,
}

impl MaterialMetallicRoughness {
    pub fn new() -> Self {
        Self {
            base_color: (1.0, 1.0, 1.0, 1.0),
            metallic: 0.0,
            roughness: 1.0,
        }
    }

    pub fn base_color(&self) -> (f32, f32, f32, f32) {
        self.base_color
    }

    pub fn set_base_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.base_color = (r, g, b, a);
    }

    pub fn metallic(&self) -> f32 {
        self.metallic
    }

    pub fn set_metallic(&mut self, m: f32) {
        self.metallic = m.max(0.0).min(1.0);
    }

    pub fn roughness(&self) -> f32 {
        self.roughness
    }

    pub fn set_roughness(&mut self, r: f32) {
        self.roughness = r.max(0.0).min(1.0);
    }
}

impl Default for MaterialMetallicRoughness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mat = MaterialMetallicRoughness::new();
        assert_eq!(mat.metallic(), 0.0);
    }
}
