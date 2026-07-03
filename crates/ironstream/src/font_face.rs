// FILE: font_face.rs
// occt: Font_BRepFont, Font_TextFormatter, Font_Rect

/// Axis for text layout.
/// occt: Font_TextFormatter
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontHAlign {
    Left,
    Center,
    Right,
}

impl Default for FontHAlign {
    fn default() -> Self { Self::Left }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontVAlign {
    Top,
    Center,
    Bottom,
}

impl Default for FontVAlign {
    fn default() -> Self { Self::Top }
}

/// A 2D rectangle (pixel-space).
/// occt: Font_Rect
#[derive(Clone, Copy, Debug, Default)]
pub struct FontRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl FontRect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn width(&self) -> f32 { (self.right - self.left).abs() }
    pub fn height(&self) -> f32 { (self.bottom - self.top).abs() }
    pub fn is_empty(&self) -> bool { self.width() < 1e-6 && self.height() < 1e-6 }
}

/// Text formatting result for a single string.
/// occt: Font_TextFormatter
#[derive(Clone, Debug)]
pub struct FontTextFormatter {
    pub text: String,
    pub font_size: f32,
    pub h_align: FontHAlign,
    pub v_align: FontVAlign,
    pub line_spacing: f32,
    pub tab_size: f32,
}

impl Default for FontTextFormatter {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 12.0,
            h_align: FontHAlign::Left,
            v_align: FontVAlign::Top,
            line_spacing: 1.2,
            tab_size: 4.0,
        }
    }
}

impl FontTextFormatter {
    pub fn new() -> Self { Self::default() }

    pub fn set_text(&mut self, t: &str) { self.text = t.to_string(); }
    pub fn set_font_size(&mut self, s: f32) { self.font_size = s.max(1.0); }
    pub fn set_h_align(&mut self, a: FontHAlign) { self.h_align = a; }
    pub fn set_v_align(&mut self, a: FontVAlign) { self.v_align = a; }
    pub fn set_line_spacing(&mut self, s: f32) { self.line_spacing = s.max(0.1); }
    pub fn set_tab_size(&mut self, s: f32) { self.tab_size = s.max(1.0); }

    /// Estimate bounding rect for the formatted text.
    pub fn bounding_box(&self) -> FontRect {
        if self.text.is_empty() { return FontRect::default(); }
        let lines: Vec<&str> = self.text.split('\n').collect();
        let nb_lines = lines.len() as f32;
        let max_chars = lines.iter().map(|l| l.len()).max().unwrap_or(0) as f32;
        // Stub: 0.6 * font_size per char width, line_spacing * font_size per line
        let w = max_chars * self.font_size * 0.6;
        let h = nb_lines * self.font_size * self.line_spacing;
        FontRect::new(0.0, 0.0, w, h)
    }

    pub fn nb_lines(&self) -> usize { self.text.split('\n').count() }

    pub fn line_at(&self, i: usize) -> Option<&str> {
        self.text.split('\n').nth(i)
    }
}

/// BRep font: a font that generates BRep shapes for glyphs (stub).
/// occt: Font_BRepFont
#[derive(Clone, Debug)]
pub struct FontBRepFont {
    pub font_name: String,
    pub font_size: f32,
    pub is_loaded: bool,
    pub nb_glyphs_cached: usize,
}

impl Default for FontBRepFont {
    fn default() -> Self {
        Self { font_name: String::from("Courier"), font_size: 10.0, is_loaded: false, nb_glyphs_cached: 0 }
    }
}

impl FontBRepFont {
    pub fn new() -> Self { Self::default() }

    pub fn load(&mut self, font_name: &str, size: f32) -> bool {
        self.font_name = font_name.to_string();
        self.font_size = size.max(1.0);
        self.is_loaded = !font_name.is_empty();
        self.is_loaded
    }

    pub fn is_loaded(&self) -> bool { self.is_loaded }
    pub fn font_size(&self) -> f32 { self.font_size }

    /// Stub: returns a shape id for the glyph character.
    pub fn render_glyph(&mut self, ch: char) -> u32 {
        if !self.is_loaded { return 0; }
        self.nb_glyphs_cached += 1;
        (ch as u32).wrapping_mul(31).wrapping_add(self.font_size as u32)
    }

    /// Advance width for a character (stub: proportional to font_size).
    pub fn glyph_advance(&self, ch: char) -> f32 {
        if !self.is_loaded { return 0.0; }
        let base = if ch == ' ' { 0.3 } else if ch.is_ascii() { 0.6 } else { 0.8 };
        base * self.font_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_rect_dimensions() {
        let r = FontRect::new(0.0, 0.0, 100.0, 50.0);
        assert!((r.width() - 100.0).abs() < 1e-6);
        assert!((r.height() - 50.0).abs() < 1e-6);
        assert!(!r.is_empty());
    }

    #[test]
    fn formatter_bounding_box() {
        let mut f = FontTextFormatter::new();
        f.set_text("Hello\nWorld");
        f.set_font_size(12.0);
        let bb = f.bounding_box();
        assert!(bb.width() > 0.0);
        assert!(bb.height() > 0.0);
        assert_eq!(f.nb_lines(), 2);
    }

    #[test]
    fn formatter_line_at() {
        let mut f = FontTextFormatter::new();
        f.set_text("line0\nline1\nline2");
        assert_eq!(f.line_at(0), Some("line0"));
        assert_eq!(f.line_at(2), Some("line2"));
        assert!(f.line_at(5).is_none());
    }

    #[test]
    fn brep_font_load_and_render() {
        let mut bf = FontBRepFont::new();
        assert!(!bf.is_loaded());
        assert!(bf.load("Courier", 14.0));
        assert!(bf.is_loaded());
        let id = bf.render_glyph('A');
        assert!(id > 0);
        assert!(bf.glyph_advance('A') > 0.0);
        assert_eq!(bf.glyph_advance(' '), 14.0 * 0.3);
    }

    #[test]
    fn brep_font_empty_name() {
        let mut bf = FontBRepFont::new();
        assert!(!bf.load("", 12.0));
        assert!(!bf.is_loaded());
    }
}
