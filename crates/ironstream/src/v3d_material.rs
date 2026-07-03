// FILE: v3d_material.rs
// occt: Graphic3d_MaterialAspect, Graphic3d_PBRMaterial

/// Type of predefined material.
/// occt: Graphic3d_NameOfMaterial
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MaterialName {
    #[default]
    UserDefined,
    Brass,
    Bronze,
    Copper,
    Gold,
    Pewter,
    Plaster,
    Plastic,
    Silver,
    Steel,
    Stone,
    ShinyPlastic,
    Satin,
    Metalized,
    Neon,
    Aluminium,
    Chrome,
    Jade,
    Obsidian,
    Pearl,
    Ruby,
    Turquoise,
    Water,
    Glass,
    Diamond,
    Charcoal,
    Default,
}

/// Classic Phong material properties.
/// occt: Graphic3d_MaterialAspect
#[derive(Clone, Debug)]
pub struct Graphic3dMaterialAspect {
    pub name: MaterialName,
    pub ambient_color: [f64; 3],
    pub diffuse_color: [f64; 3],
    pub specular_color: [f64; 3],
    pub emissive_color: [f64; 3],
    pub shininess: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub is_reflective: bool,
    pub ambient_factor: f64,
    pub diffuse_factor: f64,
    pub specular_factor: f64,
    pub emissive_factor: f64,
}

impl Default for Graphic3dMaterialAspect {
    fn default() -> Self {
        Self {
            name: MaterialName::Plastic,
            ambient_color: [0.1, 0.1, 0.1],
            diffuse_color: [0.6, 0.6, 0.6],
            specular_color: [1.0, 1.0, 1.0],
            emissive_color: [0.0; 3],
            shininess: 0.5,
            transparency: 0.0,
            refractive_index: 1.0,
            is_reflective: false,
            ambient_factor: 0.3,
            diffuse_factor: 0.65,
            specular_factor: 0.0,
            emissive_factor: 0.0,
        }
    }
}

impl Graphic3dMaterialAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_name(&mut self, name: MaterialName) {
        self.name = name;
        // Stub: apply some preset values for known materials
        match name {
            MaterialName::Gold => {
                self.ambient_color = [0.247, 0.2, 0.075];
                self.diffuse_color = [0.752, 0.606, 0.226];
                self.specular_color = [0.628, 0.556, 0.366];
                self.shininess = 0.4;
            }
            MaterialName::Silver => {
                self.ambient_color = [0.192, 0.192, 0.192];
                self.diffuse_color = [0.508, 0.508, 0.508];
                self.specular_color = [0.508, 0.508, 0.508];
                self.shininess = 0.4;
            }
            MaterialName::Glass => {
                self.transparency = 0.7;
                self.refractive_index = 1.5;
            }
            _ => {}
        }
    }

    pub fn set_shininess(&mut self, s: f64) { self.shininess = s.clamp(0.0, 1.0); }
    pub fn set_transparency(&mut self, t: f64) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_ambient(&mut self, c: [f64; 3]) { self.ambient_color = c; }
    pub fn set_diffuse(&mut self, c: [f64; 3]) { self.diffuse_color = c; }
    pub fn set_specular(&mut self, c: [f64; 3]) { self.specular_color = c; }
    pub fn set_emissive(&mut self, c: [f64; 3]) { self.emissive_color = c; }
    pub fn set_reflective(&mut self, v: bool) { self.is_reflective = v; }
    pub fn set_refractive_index(&mut self, r: f64) { self.refractive_index = r.max(1.0); }

    pub fn shininess(&self) -> f64 { self.shininess }
    pub fn transparency(&self) -> f64 { self.transparency }
    pub fn is_transparent(&self) -> bool { self.transparency > 1e-3 }
}

/// PBR (physically based rendering) material.
/// occt: Graphic3d_PBRMaterial
#[derive(Clone, Debug)]
pub struct Graphic3dPbrMaterial {
    pub base_color: [f64; 4],   // RGBA (a = alpha)
    pub metallic: f64,
    pub roughness: f64,
    pub emissive: [f64; 3],
    pub ior: f64,               // index of refraction
    pub emissive_factor: f64,
}

impl Default for Graphic3dPbrMaterial {
    fn default() -> Self {
        Self {
            base_color: [0.8, 0.8, 0.8, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            emissive: [0.0; 3],
            ior: 1.5,
            emissive_factor: 0.0,
        }
    }
}

impl Graphic3dPbrMaterial {
    pub fn new() -> Self { Self::default() }

    pub fn set_base_color(&mut self, c: [f64; 4]) { self.base_color = c; }
    pub fn set_metallic(&mut self, m: f64) { self.metallic = m.clamp(0.0, 1.0); }
    pub fn set_roughness(&mut self, r: f64) { self.roughness = r.clamp(0.0, 1.0); }
    pub fn set_emissive(&mut self, e: [f64; 3]) { self.emissive = e; }
    pub fn set_ior(&mut self, ior: f64) { self.ior = ior.max(1.0); }

    pub fn is_metallic(&self) -> bool { self.metallic > 0.5 }
    pub fn alpha(&self) -> f64 { self.base_color[3] }
    pub fn is_transparent(&self) -> bool { self.alpha() < 1.0 - 1e-6 }

    /// Fresnel reflectance at normal incidence (Schlick approximation base value).
    pub fn f0(&self) -> [f64; 3] {
        let r = ((1.0 - self.ior) / (1.0 + self.ior)).powi(2);
        let metal_f0 = [self.base_color[0], self.base_color[1], self.base_color[2]];
        let dielectric_f0 = [r, r, r];
        // Interpolate: metallic * metal + (1-metallic) * dielectric
        let m = self.metallic;
        [metal_f0[0]*m + dielectric_f0[0]*(1.0-m),
         metal_f0[1]*m + dielectric_f0[1]*(1.0-m),
         metal_f0[2]*m + dielectric_f0[2]*(1.0-m)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_defaults() {
        let m = Graphic3dMaterialAspect::new();
        assert!(!m.is_transparent());
        assert!((m.shininess() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn material_preset_gold() {
        let mut m = Graphic3dMaterialAspect::new();
        m.set_name(MaterialName::Gold);
        assert!(m.ambient_color[0] > 0.0);
        assert!(m.diffuse_color[0] > 0.5);
    }

    #[test]
    fn material_glass_transparent() {
        let mut m = Graphic3dMaterialAspect::new();
        m.set_name(MaterialName::Glass);
        assert!(m.is_transparent());
        assert!((m.refractive_index - 1.5).abs() < 1e-10);
    }

    #[test]
    fn pbr_metallic_roughness() {
        let mut p = Graphic3dPbrMaterial::new();
        p.set_metallic(0.8);
        p.set_roughness(0.2);
        assert!(p.is_metallic());
        assert!(!p.is_transparent());
    }

    #[test]
    fn pbr_f0_dielectric() {
        let mut p = Graphic3dPbrMaterial::new();
        p.set_metallic(0.0);
        p.set_ior(1.5);
        let f = p.f0();
        // Fresnel for glass ~0.04
        assert!(f[0] > 0.0 && f[0] < 0.1);
    }

    #[test]
    fn pbr_transparent() {
        let mut p = Graphic3dPbrMaterial::new();
        p.set_base_color([0.8, 0.8, 0.8, 0.3]);
        assert!(p.is_transparent());
    }
}
