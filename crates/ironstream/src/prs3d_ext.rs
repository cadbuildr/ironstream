// FILE: prs3d_ext.rs
// occt: Prs3d_Drawer, Prs3d_BndBox, Prs3d_ShadingAspect, Prs3d_LineAspect

/// Display mode for Prs3d drawing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs3dDisplayMode {
    Wireframe,
    Shading,
    ShadingWithEdges,
    QuickHidden,
}

impl Default for Prs3dDisplayMode {
    fn default() -> Self { Self::Shading }
}

/// Type of edge to draw for highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs3dTypeOfHLR {
    NotSet,
    PolyAlgo,
    Algo,
}

impl Default for Prs3dTypeOfHLR {
    fn default() -> Self { Self::NotSet }
}

// occt: Prs3d_LineAspect — line rendering attributes
#[derive(Clone, Debug)]
pub struct Prs3dLineAspect {
    pub color: [f32; 3],
    pub line_type: u8,
    pub width: f32,
}

impl Default for Prs3dLineAspect {
    fn default() -> Self {
        Self { color: [1.0, 1.0, 1.0], line_type: 0, width: 1.0 }
    }
}

impl Prs3dLineAspect {
    pub fn new(r: f32, g: f32, b: f32, width: f32) -> Self {
        Self { color: [r, g, b], width, line_type: 0 }
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; }
    pub fn set_width(&mut self, w: f32) { self.width = w; }
    pub fn color(&self) -> [f32; 3] { self.color }
    pub fn width(&self) -> f32 { self.width }
}

// occt: Prs3d_ShadingAspect — surface shading rendering attributes
#[derive(Clone, Debug)]
pub struct Prs3dShadingAspect {
    pub color: [f32; 3],
    pub transparency: f32,
    pub shininess: f32,
    pub material_name: String,
}

impl Default for Prs3dShadingAspect {
    fn default() -> Self {
        Self {
            color: [0.7, 0.7, 0.7],
            transparency: 0.0,
            shininess: 20.0,
            material_name: "Plastique".to_owned(),
        }
    }
}

impl Prs3dShadingAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; }
    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_material(&mut self, name: &str) { self.material_name = name.to_owned(); }
    pub fn color(&self) -> [f32; 3] { self.color }
    pub fn transparency(&self) -> f32 { self.transparency }
    pub fn is_transparent(&self) -> bool { self.transparency > 1e-3 }
}

// occt: Prs3d_BndBox — bounding box presentation attributes
#[derive(Clone, Debug)]
pub struct Prs3dBndBox {
    pub is_oriented: bool,
    pub nb_facets_in_triangle: u32,
    pub draw_named_texts: bool,
    pub corner_labels: bool,
}

impl Default for Prs3dBndBox {
    fn default() -> Self {
        Self {
            is_oriented: false,
            nb_facets_in_triangle: 0,
            draw_named_texts: false,
            corner_labels: false,
        }
    }
}

impl Prs3dBndBox {
    pub fn new() -> Self { Self::default() }
    pub fn set_oriented(&mut self, v: bool) { self.is_oriented = v; }
    pub fn set_draw_named_texts(&mut self, v: bool) { self.draw_named_texts = v; }
}

// occt: Prs3d_Drawer — aggregated presentation attributes
#[derive(Clone, Debug)]
pub struct Prs3dDrawer {
    pub wire_aspect: Prs3dLineAspect,
    pub line_aspect: Prs3dLineAspect,
    pub shading_aspect: Prs3dShadingAspect,
    pub bnd_box: Prs3dBndBox,
    pub type_of_hlr: Prs3dTypeOfHLR,
    pub display_mode: Prs3dDisplayMode,
    pub deviation_coefficient: f64,
    pub deviation_angle: f64,
    pub is_autotrianglulation: bool,
    pub nb_points: i32,
}

impl Default for Prs3dDrawer {
    fn default() -> Self {
        Self {
            wire_aspect: Prs3dLineAspect::new(1.0, 1.0, 0.0, 1.0),
            line_aspect: Prs3dLineAspect::default(),
            shading_aspect: Prs3dShadingAspect::default(),
            bnd_box: Prs3dBndBox::default(),
            type_of_hlr: Prs3dTypeOfHLR::NotSet,
            display_mode: Prs3dDisplayMode::Shading,
            deviation_coefficient: 0.001,
            deviation_angle: 0.04,
            is_autotrianglulation: true,
            nb_points: 30,
        }
    }
}

impl Prs3dDrawer {
    pub fn new() -> Self { Self::default() }

    pub fn set_deviation_coefficient(&mut self, v: f64) { self.deviation_coefficient = v; }
    pub fn set_deviation_angle(&mut self, v: f64) { self.deviation_angle = v; }
    pub fn set_type_of_hlr(&mut self, t: Prs3dTypeOfHLR) { self.type_of_hlr = t; }
    pub fn set_display_mode(&mut self, m: Prs3dDisplayMode) { self.display_mode = m; }

    pub fn deviation_coefficient(&self) -> f64 { self.deviation_coefficient }
    pub fn deviation_angle(&self) -> f64 { self.deviation_angle }
    pub fn nb_points(&self) -> i32 { self.nb_points }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_aspect_color() {
        let mut la = Prs3dLineAspect::new(1.0, 0.0, 0.0, 2.0);
        assert_eq!(la.color(), [1.0, 0.0, 0.0]);
        la.set_width(3.0);
        assert!((la.width() - 3.0).abs() < 1e-6);
    }

    #[test]
    fn shading_aspect_transparency() {
        let mut sa = Prs3dShadingAspect::new();
        sa.set_transparency(0.5);
        assert!(sa.is_transparent());
        assert!((sa.transparency() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn shading_aspect_color() {
        let mut sa = Prs3dShadingAspect::new();
        sa.set_color(0.2, 0.3, 0.4);
        assert_eq!(sa.color(), [0.2, 0.3, 0.4]);
    }

    #[test]
    fn bnd_box_defaults() {
        let b = Prs3dBndBox::new();
        assert!(!b.is_oriented);
        assert!(!b.draw_named_texts);
    }

    #[test]
    fn drawer_defaults() {
        let d = Prs3dDrawer::new();
        assert!((d.deviation_coefficient() - 0.001).abs() < 1e-15);
        assert_eq!(d.nb_points(), 30);
        assert!(d.is_autotrianglulation);
    }

    #[test]
    fn drawer_set_hlr() {
        let mut d = Prs3dDrawer::new();
        d.set_type_of_hlr(Prs3dTypeOfHLR::Algo);
        assert_eq!(d.type_of_hlr, Prs3dTypeOfHLR::Algo);
    }
}
