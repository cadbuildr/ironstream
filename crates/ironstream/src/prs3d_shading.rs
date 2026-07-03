// FILE: prs3d_shading.rs
// occt: Prs3d_ShadingAspect, Prs3d_PlaneAspect, Prs3d_IsoAspect

use std::collections::HashMap;

/// Type of shading model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadingModel {
    Unlit,
    Flat,
    Phong,
    Pbr,
}

impl Default for ShadingModel {
    fn default() -> Self { Self::Phong }
}

/// Side of a surface to shade.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceSide {
    Aspect_TOFM_BACK_SIDE,
    Aspect_TOFM_FRONT_SIDE,
    Aspect_TOFM_BOTH_SIDE,
}

impl Default for FaceSide {
    fn default() -> Self { Self::Aspect_TOFM_BOTH_SIDE }
}

/// Material properties for shading.
#[derive(Clone, Debug)]
pub struct ShadingMaterial {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub emissive: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    pub metallic: f32,
    pub roughness: f32,
}

impl Default for ShadingMaterial {
    fn default() -> Self {
        Self {
            name: "Default".to_owned(),
            ambient: [0.1, 0.1, 0.1],
            diffuse: [0.7, 0.7, 0.7],
            specular: [0.2, 0.2, 0.2],
            emissive: [0.0, 0.0, 0.0],
            shininess: 20.0,
            transparency: 0.0,
            metallic: 0.0,
            roughness: 0.5,
        }
    }
}

impl ShadingMaterial {
    pub fn matte() -> Self {
        Self { name: "Matte".to_owned(), shininess: 1.0, specular: [0.0; 3], ..Default::default() }
    }

    pub fn metallic() -> Self {
        Self {
            name: "Metal".to_owned(),
            diffuse: [0.5, 0.5, 0.5],
            specular: [0.8, 0.8, 0.8],
            shininess: 100.0,
            metallic: 1.0,
            roughness: 0.1,
            ..Default::default()
        }
    }

    pub fn glass(transparency: f32) -> Self {
        Self {
            name: "Glass".to_owned(),
            diffuse: [0.9, 0.9, 0.9],
            specular: [0.9, 0.9, 0.9],
            shininess: 128.0,
            transparency: transparency.clamp(0.0, 1.0),
            roughness: 0.0,
            ..Default::default()
        }
    }

    pub fn is_transparent(&self) -> bool { self.transparency > 1e-3 }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.diffuse = [r, g, b];
    }
}

// occt: Prs3d_ShadingAspect — shading attributes for faces in a presentation
#[derive(Clone, Debug, Default)]
pub struct Prs3dShadingAspect {
    pub material_front: ShadingMaterial,
    pub material_back: ShadingMaterial,
    pub face_side: FaceSide,
    pub shading_model: ShadingModel,
    pub is_back_face_drawing: bool,
    pub polygon_offset_mode: bool,
    pub polygon_offset: f32,
    pub line_width: f32,
}

impl Prs3dShadingAspect {
    pub fn new() -> Self { Self { line_width: 1.0, ..Default::default() } }

    pub fn set_material(&mut self, mat: ShadingMaterial) { self.material_front = mat.clone(); self.material_back = mat; }
    pub fn set_material_front(&mut self, mat: ShadingMaterial) { self.material_front = mat; }
    pub fn set_material_back(&mut self, mat: ShadingMaterial) { self.material_back = mat; }
    pub fn set_face_side(&mut self, side: FaceSide) { self.face_side = side; }
    pub fn set_shading_model(&mut self, model: ShadingModel) { self.shading_model = model; }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.material_front.set_color(r, g, b);
        self.material_back.set_color(r, g, b);
    }

    pub fn transparency(&self) -> f32 { self.material_front.transparency }
    pub fn set_transparency(&mut self, t: f32) {
        let t = t.clamp(0.0, 1.0);
        self.material_front.transparency = t;
        self.material_back.transparency = t;
    }

    pub fn is_transparent(&self) -> bool { self.material_front.is_transparent() }
}

// occt: Prs3d_PlaneAspect — visual attributes for a displayed infinite plane
#[derive(Clone, Debug)]
pub struct Prs3dPlaneAspect {
    pub edges_in_axes_length: f64,
    pub planes_length: f64,
    pub arrow_length: f64,
    pub draw_arrows: bool,
    pub draw_center: bool,
    pub is_xy_plane: bool,
    pub is_xz_plane: bool,
    pub is_yz_plane: bool,
    pub color: [f32; 3],
    pub line_type: u8,
}

impl Default for Prs3dPlaneAspect {
    fn default() -> Self {
        Self {
            edges_in_axes_length: 1.0,
            planes_length: 10.0,
            arrow_length: 0.5,
            draw_arrows: true,
            draw_center: true,
            is_xy_plane: false,
            is_xz_plane: false,
            is_yz_plane: false,
            color: [0.5, 0.5, 0.5],
            line_type: 0,
        }
    }
}

impl Prs3dPlaneAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_planes_length(&mut self, l: f64) { self.planes_length = l; }
    pub fn set_arrows_length(&mut self, l: f64) { self.arrow_length = l; }
    pub fn display_arrows(&mut self, show: bool) { self.draw_arrows = show; }
    pub fn display_center(&mut self, show: bool) { self.draw_center = show; }
    pub fn set_display_xy_plane(&mut self, b: bool) { self.is_xy_plane = b; }
    pub fn set_display_xz_plane(&mut self, b: bool) { self.is_xz_plane = b; }
    pub fn set_display_yz_plane(&mut self, b: bool) { self.is_yz_plane = b; }

    pub fn planes_length(&self) -> f64 { self.planes_length }
    pub fn arrows_length(&self) -> f64 { self.arrow_length }
    pub fn is_display_xy_plane(&self) -> bool { self.is_xy_plane }
}

// occt: Prs3d_IsoAspect — attributes for drawing isoparametric curves on surfaces
#[derive(Clone, Debug)]
pub struct Prs3dIsoAspect {
    pub number: usize,
    pub color: [f32; 3],
    pub line_width: f32,
    pub line_type: u8,
}

impl Default for Prs3dIsoAspect {
    fn default() -> Self {
        Self { number: 1, color: [0.7, 0.7, 0.7], line_width: 0.5, line_type: 0 }
    }
}

impl Prs3dIsoAspect {
    pub fn new(number: usize) -> Self { Self { number, ..Default::default() } }
    pub fn set_number(&mut self, n: usize) { self.number = n; }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; }
    pub fn set_width(&mut self, w: f32) { self.line_width = w; }
    pub fn number(&self) -> usize { self.number }
}

// occt: Prs3d_Drawer wrapper that collects shading + plane + iso aspects
#[derive(Clone, Debug, Default)]
pub struct DrawerExt {
    pub shading_aspect: Prs3dShadingAspect,
    pub plane_aspect: Prs3dPlaneAspect,
    pub u_iso_aspect: Prs3dIsoAspect,
    pub v_iso_aspect: Prs3dIsoAspect,
    pub extra_aspects: HashMap<String, String>,
}

impl DrawerExt {
    pub fn new() -> Self { Self::default() }

    pub fn set_shading_color(&mut self, r: f32, g: f32, b: f32) {
        self.shading_aspect.set_color(r, g, b);
    }

    pub fn set_nb_isos(&mut self, nu: usize, nv: usize) {
        self.u_iso_aspect.set_number(nu);
        self.v_iso_aspect.set_number(nv);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shading_material_glass() {
        let g = ShadingMaterial::glass(0.5);
        assert!(g.is_transparent());
        assert!((g.transparency - 0.5).abs() < 1e-6);
    }

    #[test]
    fn shading_material_metallic() {
        let m = ShadingMaterial::metallic();
        assert!((m.metallic - 1.0).abs() < 1e-6);
        assert!(!m.is_transparent());
    }

    #[test]
    fn shading_aspect_color() {
        let mut a = Prs3dShadingAspect::new();
        a.set_color(1.0, 0.0, 0.0);
        assert_eq!(a.material_front.diffuse, [1.0, 0.0, 0.0]);
        a.set_transparency(0.6);
        assert!(a.is_transparent());
    }

    #[test]
    fn plane_aspect() {
        let mut p = Prs3dPlaneAspect::new();
        p.set_planes_length(20.0);
        p.set_display_xy_plane(true);
        assert!((p.planes_length() - 20.0).abs() < 1e-10);
        assert!(p.is_display_xy_plane());
    }

    #[test]
    fn iso_aspect() {
        let mut iso = Prs3dIsoAspect::new(4);
        iso.set_color(0.5, 0.5, 1.0);
        assert_eq!(iso.number(), 4);
        assert_eq!(iso.color, [0.5, 0.5, 1.0]);
    }

    #[test]
    fn drawer_ext() {
        let mut d = DrawerExt::new();
        d.set_shading_color(0.8, 0.8, 0.8);
        d.set_nb_isos(3, 3);
        assert_eq!(d.u_iso_aspect.number(), 3);
        assert_eq!(d.v_iso_aspect.number(), 3);
    }
}
