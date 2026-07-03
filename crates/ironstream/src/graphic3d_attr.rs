// FILE: graphic3d_attr.rs
// occt: Graphic3d_Aspects, Graphic3d_MaterialAspect, Graphic3d_TextureParams

use std::collections::HashMap;

/// Type of polygon fill.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolygonOffset {
    None,
    Fill,
    Line,
    Point,
}

impl Default for PolygonOffset {
    fn default() -> Self { Self::None }
}

/// Interpolation type for a texture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureInterp {
    Nearest,
    Linear,
    MipMapNearest,
    MipMapLinear,
}

impl Default for TextureInterp {
    fn default() -> Self { Self::Linear }
}

/// How UV is wrapped for a texture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureWrapMode {
    Repeat,
    Clamp,
    Mirror,
    ClampToBorder,
}

impl Default for TextureWrapMode {
    fn default() -> Self { Self::Repeat }
}

// occt: Graphic3d_TextureParams — texture mapping parameters
#[derive(Clone, Debug, Default)]
pub struct Graphic3dTextureParams {
    pub is_modulate: bool,
    pub repeat: [f64; 2],
    pub scale: [f64; 2],
    pub translation: [f64; 2],
    pub rotation_angle: f64,
    pub filter: TextureInterp,
    pub wrap_u: TextureWrapMode,
    pub wrap_v: TextureWrapMode,
    pub sampler_anisotropy: f32,
}

impl Graphic3dTextureParams {
    pub fn new() -> Self {
        Self {
            is_modulate: true,
            repeat: [1.0, 1.0],
            scale: [1.0, 1.0],
            sampler_anisotropy: 1.0,
            ..Default::default()
        }
    }

    pub fn set_repeat(&mut self, s: f64, t: f64) { self.repeat = [s, t]; }
    pub fn set_scale(&mut self, s: f64, t: f64) { self.scale = [s, t]; }
    pub fn set_translation(&mut self, u: f64, v: f64) { self.translation = [u, v]; }
    pub fn set_rotation(&mut self, angle: f64) { self.rotation_angle = angle; }
    pub fn set_filter(&mut self, f: TextureInterp) { self.filter = f; }
    pub fn set_wrap(&mut self, wu: TextureWrapMode, wv: TextureWrapMode) { self.wrap_u = wu; self.wrap_v = wv; }

    pub fn repeat(&self) -> [f64; 2] { self.repeat }
    pub fn scale(&self) -> [f64; 2] { self.scale }
}

// occt: Graphic3d_MaterialAspect — surface material for rendering
#[derive(Clone, Debug)]
pub struct Graphic3dMaterialAspect {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub emissive_color: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    pub refraction_index: f32,
    pub pbr_albedo: [f32; 3],
    pub pbr_metallic: f32,
    pub pbr_roughness: f32,
    pub pbr_emissive: [f32; 3],
    pub is_env_reflective: bool,
}

impl Default for Graphic3dMaterialAspect {
    fn default() -> Self {
        Self {
            name: "DefaultMaterial".to_owned(),
            ambient_color: [0.1, 0.1, 0.1],
            diffuse_color: [0.7, 0.7, 0.7],
            specular_color: [0.2, 0.2, 0.2],
            emissive_color: [0.0; 3],
            shininess: 20.0,
            transparency: 0.0,
            refraction_index: 1.5,
            pbr_albedo: [0.7, 0.7, 0.7],
            pbr_metallic: 0.0,
            pbr_roughness: 0.5,
            pbr_emissive: [0.0; 3],
            is_env_reflective: false,
        }
    }
}

impl Graphic3dMaterialAspect {
    pub fn new(name: &str) -> Self { Self { name: name.to_owned(), ..Default::default() } }

    pub fn set_diffuse_color(&mut self, r: f32, g: f32, b: f32) { self.diffuse_color = [r, g, b]; }
    pub fn set_specular_color(&mut self, r: f32, g: f32, b: f32) { self.specular_color = [r, g, b]; }
    pub fn set_shininess(&mut self, s: f32) { self.shininess = s.clamp(0.0, 128.0); }
    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_pbr_metallic(&mut self, v: f32) { self.pbr_metallic = v.clamp(0.0, 1.0); }
    pub fn set_pbr_roughness(&mut self, v: f32) { self.pbr_roughness = v.clamp(0.0, 1.0); }

    pub fn is_transparent(&self) -> bool { self.transparency > 1e-3 }
    pub fn shininess(&self) -> f32 { self.shininess }
    pub fn transparency(&self) -> f32 { self.transparency }
    pub fn diffuse(&self) -> [f32; 3] { self.diffuse_color }
    pub fn specular(&self) -> [f32; 3] { self.specular_color }
}

// occt: Graphic3d_Aspects — combined rendering attributes for a group
#[derive(Clone, Debug)]
pub struct Graphic3dAspects {
    pub color: [f32; 3],
    pub interior_color: [f32; 3],
    pub edge_color: [f32; 3],
    pub line_width: f32,
    pub transparency: f32,
    pub polygon_offset: PolygonOffset,
    pub polygon_offset_units: f32,
    pub polygon_offset_factor: f32,
    pub material_front: Graphic3dMaterialAspect,
    pub material_back: Graphic3dMaterialAspect,
    pub texture_params: Vec<Graphic3dTextureParams>,
    pub draw_edges: bool,
    pub suppress_facets: bool,
    pub marker_type: u32,
    pub marker_scale: f32,
}

impl Default for Graphic3dAspects {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            interior_color: [0.7, 0.7, 0.7],
            edge_color: [0.0, 0.0, 0.0],
            line_width: 1.0,
            transparency: 0.0,
            polygon_offset: PolygonOffset::None,
            polygon_offset_units: 0.0,
            polygon_offset_factor: 1.0,
            material_front: Graphic3dMaterialAspect::default(),
            material_back: Graphic3dMaterialAspect::default(),
            texture_params: Vec::new(),
            draw_edges: false,
            suppress_facets: false,
            marker_type: 0,
            marker_scale: 1.0,
        }
    }
}

impl Graphic3dAspects {
    pub fn new() -> Self { Self::default() }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; }
    pub fn set_interior_color(&mut self, r: f32, g: f32, b: f32) { self.interior_color = [r, g, b]; }
    pub fn set_edge_color(&mut self, r: f32, g: f32, b: f32) { self.edge_color = [r, g, b]; }
    pub fn set_line_width(&mut self, w: f32) { self.line_width = w; }
    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_draw_edges(&mut self, v: bool) { self.draw_edges = v; }
    pub fn set_material(&mut self, mat: Graphic3dMaterialAspect) {
        self.material_front = mat.clone();
        self.material_back = mat;
    }

    pub fn add_texture(&mut self, params: Graphic3dTextureParams) { self.texture_params.push(params); }
    pub fn nb_textures(&self) -> usize { self.texture_params.len() }
    pub fn is_transparent(&self) -> bool { self.transparency > 1e-3 }
    pub fn color(&self) -> [f32; 3] { self.color }
    pub fn line_width(&self) -> f32 { self.line_width }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_params_repeat() {
        let mut tp = Graphic3dTextureParams::new();
        tp.set_repeat(2.0, 3.0);
        assert_eq!(tp.repeat(), [2.0, 3.0]);
        tp.set_filter(TextureInterp::MipMapLinear);
        assert_eq!(tp.filter, TextureInterp::MipMapLinear);
    }

    #[test]
    fn material_aspect() {
        let mut mat = Graphic3dMaterialAspect::new("Gold");
        mat.set_diffuse_color(1.0, 0.8, 0.0);
        mat.set_shininess(80.0);
        mat.set_transparency(0.0);
        assert!(!mat.is_transparent());
        assert!((mat.shininess() - 80.0).abs() < 1e-6);
    }

    #[test]
    fn material_pbr() {
        let mut mat = Graphic3dMaterialAspect::new("Metal");
        mat.set_pbr_metallic(1.0);
        mat.set_pbr_roughness(0.1);
        assert!((mat.pbr_metallic - 1.0).abs() < 1e-6);
    }

    #[test]
    fn aspects_color_and_transparency() {
        let mut a = Graphic3dAspects::new();
        a.set_color(0.5, 0.5, 0.5);
        a.set_transparency(0.3);
        assert!(a.is_transparent());
        assert_eq!(a.color(), [0.5, 0.5, 0.5]);
    }

    #[test]
    fn aspects_textures() {
        let mut a = Graphic3dAspects::new();
        let tp = Graphic3dTextureParams::new();
        a.add_texture(tp);
        assert_eq!(a.nb_textures(), 1);
    }

    #[test]
    fn aspects_draw_edges() {
        let mut a = Graphic3dAspects::new();
        assert!(!a.draw_edges);
        a.set_draw_edges(true);
        assert!(a.draw_edges);
    }
}
