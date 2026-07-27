// FILE: graphic3d.rs

// occt: Graphic3d_TypeOfReflection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dTypeOfReflection {
    Ambient,
    Diffuse,
    Specular,
    Emissive,
}

// occt-ref: Graphic3d_TypeOfShadingModel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dTypeOfShadingModel {
    Unlit,
    Gouraud,
    Phong,
    Pbr,
    PbrFacet,
}

// occt-ref: Graphic3d_MaterialAspect
pub struct Graphic3dMaterialAspect {
    pub name: String,
    pub transparency: f32,
    pub shininess: f32,
    pub refraction_index: f32,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub emissive_color: [f32; 3],
}

impl Graphic3dMaterialAspect {
    pub fn new(name: &str) -> Self {
        Graphic3dMaterialAspect {
            name: name.to_string(),
            transparency: 0.0,
            shininess: 0.3,
            refraction_index: 1.0,
            ambient_color: [0.5, 0.5, 0.5],
            diffuse_color: [0.5, 0.5, 0.5],
            specular_color: [0.5, 0.5, 0.5],
            emissive_color: [0.5, 0.5, 0.5],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_transparency(&mut self, t: f32) {
        self.transparency = t;
    }

    pub fn transparency(&self) -> f32 {
        self.transparency
    }

    pub fn is_transparent(&self) -> bool {
        self.transparency > 0.01
    }

    pub fn set_shininess(&mut self, s: f32) {
        self.shininess = s;
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }
}

// occt: Graphic3d_Vertex
#[derive(Clone, Copy, Debug)]
pub struct Graphic3dVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Graphic3dVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Graphic3dVertex { x, y, z }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// occt-ref: Graphic3d_Group
pub struct Graphic3dGroup {
    vertices: Vec<Graphic3dVertex>,
    material: Option<Graphic3dMaterialAspect>,
    is_empty: bool,
}

impl Graphic3dGroup {
    pub fn new() -> Self {
        Graphic3dGroup {
            vertices: Vec::new(),
            material: None,
            is_empty: true,
        }
    }

    pub fn add_vertex(&mut self, v: Graphic3dVertex) {
        self.vertices.push(v);
        self.is_empty = false;
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn set_material(&mut self, m: Graphic3dMaterialAspect) {
        self.material = Some(m);
    }

    pub fn material(&self) -> Option<&Graphic3dMaterialAspect> {
        self.material.as_ref()
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.material = None;
        self.is_empty = true;
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl Default for Graphic3dGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflection_variants_are_distinct() {
        assert_ne!(Graphic3dTypeOfReflection::Ambient, Graphic3dTypeOfReflection::Diffuse);
        assert_ne!(Graphic3dTypeOfReflection::Specular, Graphic3dTypeOfReflection::Emissive);
    }

    #[test]
    fn shading_model_variants_are_distinct() {
        assert_ne!(Graphic3dTypeOfShadingModel::Unlit, Graphic3dTypeOfShadingModel::Phong);
        assert_ne!(Graphic3dTypeOfShadingModel::Pbr, Graphic3dTypeOfShadingModel::PbrFacet);
    }

    #[test]
    fn material_new_defaults() {
        let m = Graphic3dMaterialAspect::new("Gold");
        assert_eq!(m.name(), "Gold");
        assert_eq!(m.transparency(), 0.0);
        assert!(!m.is_transparent());
        assert_eq!(m.shininess(), 0.3);
    }

    #[test]
    fn material_set_transparency_above_threshold() {
        let mut m = Graphic3dMaterialAspect::new("Glass");
        m.set_transparency(0.5);
        assert_eq!(m.transparency(), 0.5);
        assert!(m.is_transparent());
    }

    #[test]
    fn material_transparency_below_threshold_is_opaque() {
        let mut m = Graphic3dMaterialAspect::new("Plastic");
        m.set_transparency(0.005);
        assert!(!m.is_transparent());
    }

    #[test]
    fn material_set_shininess() {
        let mut m = Graphic3dMaterialAspect::new("Steel");
        m.set_shininess(0.9);
        assert_eq!(m.shininess(), 0.9);
    }

    #[test]
    fn vertex_new_and_fields() {
        let v = Graphic3dVertex::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn vertex_distance_self_is_zero() {
        let v = Graphic3dVertex::new(1.0, 2.0, 3.0);
        assert_eq!(v.distance(&v), 0.0);
    }

    #[test]
    fn vertex_distance_axis_aligned() {
        let a = Graphic3dVertex::new(0.0, 0.0, 0.0);
        let b = Graphic3dVertex::new(3.0, 4.0, 0.0);
        assert!((b.distance(&a) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn group_new_is_empty() {
        let g = Graphic3dGroup::new();
        assert!(g.is_empty());
        assert_eq!(g.nb_vertices(), 0);
        assert!(g.material().is_none());
    }

    #[test]
    fn group_add_vertex_updates_count_and_not_empty() {
        let mut g = Graphic3dGroup::new();
        g.add_vertex(Graphic3dVertex::new(0.0, 0.0, 0.0));
        assert!(!g.is_empty());
        assert_eq!(g.nb_vertices(), 1);
        g.add_vertex(Graphic3dVertex::new(1.0, 0.0, 0.0));
        assert_eq!(g.nb_vertices(), 2);
    }

    #[test]
    fn group_set_and_get_material() {
        let mut g = Graphic3dGroup::new();
        let m = Graphic3dMaterialAspect::new("Copper");
        g.set_material(m);
        let mat = g.material().unwrap();
        assert_eq!(mat.name(), "Copper");
    }

    #[test]
    fn group_clear_resets_state() {
        let mut g = Graphic3dGroup::new();
        g.add_vertex(Graphic3dVertex::new(1.0, 2.0, 3.0));
        g.set_material(Graphic3dMaterialAspect::new("Iron"));
        g.clear();
        assert!(g.is_empty());
        assert_eq!(g.nb_vertices(), 0);
        assert!(g.material().is_none());
    }
}
