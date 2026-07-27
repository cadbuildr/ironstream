// FILE: prs3d_text_prop.rs
// occt: Graphic3d_AspectText3d, Font_FontMgr
// occt-ref: Prs3d_TextAspect

/// Font weight.
/// occt: Font_FontAspect
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontAspect {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl Default for FontAspect {
    fn default() -> Self { Self::Regular }
}

/// Text horizontal alignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextHAlign {
    Left,
    Center,
    Right,
}

impl Default for TextHAlign {
    fn default() -> Self { Self::Center }
}

/// Text vertical alignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextVAlign {
    Top,
    Center,
    Bottom,
    TopFirstLine,
}

impl Default for TextVAlign {
    fn default() -> Self { Self::Center }
}

/// Text rendering style.
/// occt: Graphic3d_TextPath // (direction the text is rendered along)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextPath {
    Right,
    Left,
    Up,
    Down,
}

impl Default for TextPath {
    fn default() -> Self { Self::Right }
}

/// Text display type (background/overlay mode).
// occt-ref: Aspect_TypeOfDisplayText
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfDisplayText {
    Normal,
    Subtitle,
    Decal,
    Dimension,
}

impl Default for TypeOfDisplayText {
    fn default() -> Self { Self::Normal }
}

/// 3D text aspect: font, size, color, alignment.
// occt-ref: Prs3d_TextAspect // / Graphic3d_AspectText3d
#[derive(Clone, Debug)]
pub struct Prs3dTextAspect {
    pub font_name: String,
    pub font_aspect: FontAspect,
    pub height: f64,
    pub color: [f64; 3],
    pub color_subtitle: [f64; 3],
    pub h_align: TextHAlign,
    pub v_align: TextVAlign,
    pub display_type: TypeOfDisplayText,
    pub is_3d: bool,
    pub angle: f64,   // text rotation angle (degrees)
}

impl Default for Prs3dTextAspect {
    fn default() -> Self {
        Self {
            font_name: String::from("Courier"),
            font_aspect: FontAspect::Regular,
            height: 16.0,
            color: [1.0, 1.0, 1.0],
            color_subtitle: [0.0, 0.0, 0.0],
            h_align: TextHAlign::Center,
            v_align: TextVAlign::Center,
            display_type: TypeOfDisplayText::Normal,
            is_3d: false,
            angle: 0.0,
        }
    }
}

impl Prs3dTextAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_font(&mut self, name: &str) { self.font_name = name.to_string(); }
    pub fn set_font_aspect(&mut self, a: FontAspect) { self.font_aspect = a; }
    pub fn set_height(&mut self, h: f64) { self.height = h.max(1.0); }
    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_subtitle_color(&mut self, c: [f64; 3]) { self.color_subtitle = c; }
    pub fn set_h_align(&mut self, a: TextHAlign) { self.h_align = a; }
    pub fn set_v_align(&mut self, a: TextVAlign) { self.v_align = a; }
    pub fn set_display_type(&mut self, t: TypeOfDisplayText) { self.display_type = t; }
    pub fn set_3d(&mut self, v: bool) { self.is_3d = v; }
    pub fn set_angle(&mut self, deg: f64) { self.angle = deg; }

    pub fn font_name(&self) -> &str { &self.font_name }
    pub fn height(&self) -> f64 { self.height }
    pub fn color(&self) -> [f64; 3] { self.color }
    pub fn h_align(&self) -> TextHAlign { self.h_align }
    pub fn v_align(&self) -> TextVAlign { self.v_align }
}

/// Font descriptor for the font manager.
/// occt: Font_SystemFont
#[derive(Clone, Debug)]
pub struct FontDescriptor {
    pub family_name: String,
    pub font_path: String,
    pub aspect: FontAspect,
    pub size_points: f64,
    pub is_system_font: bool,
}

impl FontDescriptor {
    pub fn new(family: &str, path: &str, aspect: FontAspect) -> Self {
        Self {
            family_name: family.to_string(),
            font_path: path.to_string(),
            aspect,
            size_points: 12.0,
            is_system_font: true,
        }
    }
    pub fn family_name(&self) -> &str { &self.family_name }
    pub fn aspect(&self) -> FontAspect { self.aspect }
}

/// Font manager registry.
/// occt: Font_FontMgr
#[derive(Clone, Debug, Default)]
pub struct FontFontMgr {
    pub fonts: Vec<FontDescriptor>,
}

impl FontFontMgr {
    pub fn new() -> Self { Self::default() }

    pub fn register_font(&mut self, f: FontDescriptor) { self.fonts.push(f); }

    pub fn find_font(&self, family: &str, aspect: FontAspect) -> Option<&FontDescriptor> {
        self.fonts.iter().find(|f| f.family_name == family && f.aspect == aspect)
    }

    pub fn nb_fonts(&self) -> usize { self.fonts.len() }

    pub fn available_families(&self) -> Vec<String> {
        let mut names: Vec<String> = self.fonts.iter().map(|f| f.family_name.clone()).collect();
        names.sort();
        names.dedup();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_aspect_defaults() {
        let t = Prs3dTextAspect::new();
        assert_eq!(t.font_name(), "Courier");
        assert!((t.height() - 16.0).abs() < 1e-10);
        assert_eq!(t.h_align(), TextHAlign::Center);
    }

    #[test]
    fn text_aspect_setters() {
        let mut t = Prs3dTextAspect::new();
        t.set_font("Arial");
        t.set_height(24.0);
        t.set_h_align(TextHAlign::Left);
        assert_eq!(t.font_name(), "Arial");
        assert!((t.height() - 24.0).abs() < 1e-10);
        assert_eq!(t.h_align(), TextHAlign::Left);
    }

    #[test]
    fn text_aspect_height_clamped() {
        let mut t = Prs3dTextAspect::new();
        t.set_height(-5.0);
        assert!(t.height() >= 1.0);
    }

    #[test]
    fn font_mgr_register_find() {
        let mut mgr = FontFontMgr::new();
        mgr.register_font(FontDescriptor::new("Arial", "/fonts/arial.ttf", FontAspect::Regular));
        mgr.register_font(FontDescriptor::new("Arial", "/fonts/arialb.ttf", FontAspect::Bold));
        assert_eq!(mgr.nb_fonts(), 2);
        let f = mgr.find_font("Arial", FontAspect::Bold);
        assert!(f.is_some());
        assert_eq!(f.unwrap().aspect(), FontAspect::Bold);
    }

    #[test]
    fn font_mgr_available_families() {
        let mut mgr = FontFontMgr::new();
        mgr.register_font(FontDescriptor::new("Arial", "", FontAspect::Regular));
        mgr.register_font(FontDescriptor::new("Arial", "", FontAspect::Bold));
        mgr.register_font(FontDescriptor::new("Courier", "", FontAspect::Regular));
        let families = mgr.available_families();
        assert_eq!(families.len(), 2);
    }
}
