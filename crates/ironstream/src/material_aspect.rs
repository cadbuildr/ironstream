// FILE: material_aspect.rs
// occt-ref: Graphic3d_Material, Graphic3d_AspectFillArea, Graphic3d_Surface
//       Graphic3d_ReflectionModel

/// Material reflection model.
// occt-ref: Graphic3d_ReflectionModel
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum MaterialReflectionModel {
    #[default]
    PhongReflection,
    MetallicReflection,
    PlasticReflection,
}

/// Surface material properties.
// occt-ref: Graphic3d_Material
#[derive(Clone, Debug)]
pub struct Material {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub emissive: [f32; 3],
    pub shininess: f32,
    pub reflection_model: MaterialReflectionModel,
    pub refractive_index: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "default".into(),
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.8, 0.8, 0.8],
            specular: [1.0, 1.0, 1.0],
            emissive: [0.0, 0.0, 0.0],
            shininess: 0.2,
            reflection_model: MaterialReflectionModel::PhongReflection,
            refractive_index: 1.0,
        }
    }
}

impl Material {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Self::default() }
    }

    pub fn plastic() -> Self {
        Self {
            name: "plastic".into(),
            diffuse: [0.7, 0.7, 0.7],
            reflection_model: MaterialReflectionModel::PlasticReflection,
            ..Self::default()
        }
    }

    pub fn metal() -> Self {
        Self {
            name: "metal".into(),
            diffuse: [0.5, 0.5, 0.5],
            specular: [0.9, 0.9, 0.9],
            shininess: 0.8,
            reflection_model: MaterialReflectionModel::MetallicReflection,
            ..Self::default()
        }
    }

    pub fn set_shininess(&mut self, s: f32) { self.shininess = s.clamp(0.0, 1.0); }
    pub fn set_refractive_index(&mut self, n: f32) { self.refractive_index = n.max(1.0); }

    pub fn is_transparent(&self) -> bool { self.refractive_index > 1.0 }
}

/// Fill area aspect (surface appearance).
// occt-ref: Graphic3d_AspectFillArea
#[derive(Clone, Debug)]
pub struct AspectFillArea {
    pub material: Material,
    pub line_width: f32,
    pub interior_style: u8,  // 0=solid 1=hatch 2=hollow 3=stipple
    pub back_material: Option<Material>,
    pub cull_face: bool,
}

impl Default for AspectFillArea {
    fn default() -> Self {
        Self {
            material: Material::default(),
            line_width: 1.0,
            interior_style: 0,
            back_material: None,
            cull_face: false,
        }
    }
}

impl AspectFillArea {
    pub fn new() -> Self { Self::default() }

    pub fn set_material(&mut self, m: Material) { self.material = m; }
    pub fn set_back_material(&mut self, m: Material) { self.back_material = Some(m); }
    pub fn set_interior_style(&mut self, s: u8) { self.interior_style = s.min(3); }
    pub fn set_line_width(&mut self, w: f32) { self.line_width = w.max(0.1); }

    pub fn set_cull_back_faces(&mut self, v: bool) { self.cull_face = v; }
    pub fn is_two_sided(&self) -> bool { self.back_material.is_some() }
}

/// Surface finish properties.
/// occt-note: Graphic3d_Surface (material params)
#[derive(Clone, Debug)]
pub struct Surface {
    pub material_id: u32,
    pub gloss_factor: f32,
    pub roughness: f32,
    pub metallic: f32,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            material_id: 0,
            gloss_factor: 0.5,
            roughness: 0.5,
            metallic: 0.0,
        }
    }
}

impl Surface {
    pub fn new(mat_id: u32) -> Self {
        Self { material_id: mat_id, ..Self::default() }
    }

    pub fn is_glossy(&self) -> bool { self.gloss_factor > 0.7 }
    pub fn is_rough(&self) -> bool { self.roughness > 0.7 }
    pub fn is_metallic(&self) -> bool { self.metallic > 0.5 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_defaults() {
        let m = Material::default();
        assert_eq!(m.reflection_model, MaterialReflectionModel::PhongReflection);
        assert!(!m.is_transparent());
    }

    #[test]
    fn material_presets() {
        let p = Material::plastic();
        assert_eq!(p.name, "plastic");
        let me = Material::metal();
        assert_eq!(me.name, "metal");
        assert!(me.shininess > p.shininess);
    }

    #[test]
    fn aspect_fill_area() {
        let mut a = AspectFillArea::new();
        assert!(!a.is_two_sided());
        a.set_back_material(Material::plastic());
        assert!(a.is_two_sided());
        a.set_line_width(2.5);
        assert_eq!(a.line_width, 2.5);
    }

    #[test]
    fn surface_properties() {
        let mut s = Surface::new(1);
        s.gloss_factor = 0.9;
        s.metallic = 0.8;
        assert!(s.is_glossy());
        assert!(s.is_metallic());
    }

    #[test]
    fn material_refractive() {
        let mut m = Material::default();
        assert!(!m.is_transparent());
        m.set_refractive_index(1.5);
        assert!(m.is_transparent());
    }
}
