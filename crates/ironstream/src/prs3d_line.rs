// FILE: prs3d_line.rs
// occt: Prs3d_LineAspect, Prs3d_PointAspect, Prs3d_ShadingAspect,
//       Prs3d_IsoAspect, Prs3d_PlaneAspect

/// Line type (stipple pattern).
/// occt: Aspect_TypeOfLine
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfLine {
    Solid,
    Dotted,
    Dashed,
    DotDash,
    UserDefined,
}

impl Default for TypeOfLine {
    fn default() -> Self { Self::Solid }
}

/// Marker type for points.
/// occt: Aspect_TypeOfMarker
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfMarker {
    Point,
    Plus,
    Star,
    X,
    Ring1,
    Ring2,
    Ball,
    UserDefined,
}

impl Default for TypeOfMarker {
    fn default() -> Self { Self::Point }
}

/// Interpol type for shading.
/// occt: Graphic3d_TypeOfShadingModel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadingModel {
    Flat,
    Gouraud,
    Phong,
    Pbr,
    Unlit,
}

impl Default for ShadingModel {
    fn default() -> Self { Self::Phong }
}

/// Line aspect (color + type + width).
/// occt: Prs3d_LineAspect
#[derive(Clone, Debug)]
pub struct Prs3dLineAspect {
    pub color: [f64; 3],
    pub line_type: TypeOfLine,
    pub width: f64,
}

impl Prs3dLineAspect {
    pub fn new(color: [f64; 3], line_type: TypeOfLine, width: f64) -> Self {
        Self { color, line_type, width }
    }

    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_type_of_line(&mut self, t: TypeOfLine) { self.line_type = t; }
    pub fn set_width(&mut self, w: f64) { self.width = w; }

    pub fn color(&self) -> [f64; 3] { self.color }
    pub fn type_of_line(&self) -> TypeOfLine { self.line_type }
    pub fn width(&self) -> f64 { self.width }
}

impl Default for Prs3dLineAspect {
    fn default() -> Self {
        Self { color: [1.0, 1.0, 1.0], line_type: TypeOfLine::Solid, width: 1.0 }
    }
}

/// Point (marker) aspect.
/// occt: Prs3d_PointAspect
#[derive(Clone, Debug)]
pub struct Prs3dPointAspect {
    pub color: [f64; 3],
    pub marker: TypeOfMarker,
    pub scale: f64,
}

impl Prs3dPointAspect {
    pub fn new(color: [f64; 3], marker: TypeOfMarker, scale: f64) -> Self {
        Self { color, marker, scale }
    }

    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_type_of_marker(&mut self, m: TypeOfMarker) { self.marker = m; }
    pub fn set_scale(&mut self, s: f64) { self.scale = s; }

    pub fn color(&self) -> [f64; 3] { self.color }
    pub fn type_of_marker(&self) -> TypeOfMarker { self.marker }
    pub fn scale(&self) -> f64 { self.scale }
}

impl Default for Prs3dPointAspect {
    fn default() -> Self {
        Self { color: [1.0, 1.0, 1.0], marker: TypeOfMarker::Point, scale: 1.0 }
    }
}

/// Shading (surface fill) aspect.
/// occt: Prs3d_ShadingAspect
#[derive(Clone, Debug)]
pub struct Prs3dShadingAspect {
    pub color_front: [f64; 3],
    pub color_back: [f64; 3],
    pub material_shininess: f64,
    pub transparency: f64,
    pub shading_model: ShadingModel,
    pub is_backface_cull: bool,
}

impl Prs3dShadingAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_color(&mut self, c: [f64; 3]) { self.color_front = c; }
    pub fn set_color_on_side(&mut self, front: bool, c: [f64; 3]) {
        if front { self.color_front = c; } else { self.color_back = c; }
    }
    pub fn set_transparency(&mut self, t: f64) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_shininess(&mut self, s: f64) { self.material_shininess = s; }
    pub fn set_shading_model(&mut self, m: ShadingModel) { self.shading_model = m; }
    pub fn set_backface_cull(&mut self, v: bool) { self.is_backface_cull = v; }

    pub fn color(&self) -> [f64; 3] { self.color_front }
    pub fn transparency(&self) -> f64 { self.transparency }
    pub fn shininess(&self) -> f64 { self.material_shininess }
    pub fn shading_model(&self) -> ShadingModel { self.shading_model }
    pub fn is_backface_cull(&self) -> bool { self.is_backface_cull }
}

impl Default for Prs3dShadingAspect {
    fn default() -> Self {
        Self {
            color_front: [0.7, 0.7, 0.7],
            color_back: [0.5, 0.5, 0.5],
            material_shininess: 64.0,
            transparency: 0.0,
            shading_model: ShadingModel::Phong,
            is_backface_cull: false,
        }
    }
}

/// Iso-lines aspect (u/v isolines on surfaces).
/// occt: Prs3d_IsoAspect
#[derive(Clone, Debug)]
pub struct Prs3dIsoAspect {
    pub line_aspect: Prs3dLineAspect,
    pub nb_iso: usize,
}

impl Prs3dIsoAspect {
    pub fn new(color: [f64; 3], line_type: TypeOfLine, width: f64, nb_iso: usize) -> Self {
        Self { line_aspect: Prs3dLineAspect::new(color, line_type, width), nb_iso }
    }

    pub fn set_number(&mut self, n: usize) { self.nb_iso = n; }
    pub fn number(&self) -> usize { self.nb_iso }
    pub fn line_aspect(&self) -> &Prs3dLineAspect { &self.line_aspect }
    pub fn line_aspect_mut(&mut self) -> &mut Prs3dLineAspect { &mut self.line_aspect }
}

impl Default for Prs3dIsoAspect {
    fn default() -> Self {
        Self {
            line_aspect: Prs3dLineAspect::new([0.5, 0.5, 0.5], TypeOfLine::Solid, 1.0),
            nb_iso: 10,
        }
    }
}

/// Plane (infinite/finite) aspect.
/// occt: Prs3d_PlaneAspect
#[derive(Clone, Debug)]
pub struct Prs3dPlaneAspect {
    pub edges_aspect: Prs3dLineAspect,
    pub iso_aspect: Prs3dIsoAspect,
    pub arrow_aspect: Prs3dLineAspect,
    pub display_edges: bool,
    pub display_iso: bool,
    pub display_arrows: bool,
    pub plane_x_length: f64,
    pub plane_y_length: f64,
    pub arrow_length: f64,
}

impl Prs3dPlaneAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_plane_length(&mut self, x: f64, y: f64) {
        self.plane_x_length = x;
        self.plane_y_length = y;
    }

    pub fn set_display_edges(&mut self, v: bool) { self.display_edges = v; }
    pub fn set_display_iso(&mut self, v: bool) { self.display_iso = v; }
    pub fn set_display_arrows(&mut self, v: bool) { self.display_arrows = v; }
    pub fn set_arrow_length(&mut self, l: f64) { self.arrow_length = l; }

    pub fn plane_x_length(&self) -> f64 { self.plane_x_length }
    pub fn plane_y_length(&self) -> f64 { self.plane_y_length }
    pub fn display_edges(&self) -> bool { self.display_edges }
    pub fn display_iso(&self) -> bool { self.display_iso }
}

impl Default for Prs3dPlaneAspect {
    fn default() -> Self {
        Self {
            edges_aspect: Prs3dLineAspect::default(),
            iso_aspect: Prs3dIsoAspect::default(),
            arrow_aspect: Prs3dLineAspect::default(),
            display_edges: true,
            display_iso: true,
            display_arrows: true,
            plane_x_length: 100.0,
            plane_y_length: 100.0,
            arrow_length: 5.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_aspect_basic() {
        let mut a = Prs3dLineAspect::new([1.0,0.0,0.0], TypeOfLine::Dashed, 2.0);
        assert_eq!(a.type_of_line(), TypeOfLine::Dashed);
        assert!((a.width() - 2.0).abs() < 1e-12);
        a.set_width(3.0);
        assert!((a.width() - 3.0).abs() < 1e-12);
        a.set_color([0.0,1.0,0.0]);
        assert_eq!(a.color(), [0.0,1.0,0.0]);
    }

    #[test]
    fn point_aspect_basic() {
        let mut p = Prs3dPointAspect::new([1.0,1.0,0.0], TypeOfMarker::Star, 2.0);
        assert_eq!(p.type_of_marker(), TypeOfMarker::Star);
        p.set_scale(3.0);
        assert!((p.scale() - 3.0).abs() < 1e-12);
    }

    #[test]
    fn shading_aspect_transparency() {
        let mut s = Prs3dShadingAspect::new();
        s.set_transparency(0.5);
        assert!((s.transparency() - 0.5).abs() < 1e-12);
        s.set_transparency(2.0); // clamp to 1.0
        assert!((s.transparency() - 1.0).abs() < 1e-12);
        s.set_color([0.2, 0.3, 0.4]);
        assert_eq!(s.color(), [0.2, 0.3, 0.4]);
    }

    #[test]
    fn iso_aspect_number() {
        let mut iso = Prs3dIsoAspect::default();
        assert_eq!(iso.number(), 10);
        iso.set_number(5);
        assert_eq!(iso.number(), 5);
    }

    #[test]
    fn plane_aspect_length() {
        let mut p = Prs3dPlaneAspect::new();
        p.set_plane_length(200.0, 150.0);
        assert!((p.plane_x_length() - 200.0).abs() < 1e-10);
        assert!((p.plane_y_length() - 150.0).abs() < 1e-10);
    }
}
