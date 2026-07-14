// FILE: font_brep.rs
// occt: Font_BRepFont, Font_BRepTextBuilder
// occt-ref: Font_FontMgr, Font_SystemFont

/// Font rendering mode for BRep text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontAspect {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

/// Unit system for font metrics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontUnit {
    Point,
    Pixel,
    Mm,
    Inch,
}

impl FontUnit {
    pub fn to_mm_factor(&self) -> f64 {
        match self {
            FontUnit::Mm => 1.0,
            FontUnit::Inch => 25.4,
            FontUnit::Point => 0.352778,
            FontUnit::Pixel => 0.264583,
        }
    }
}

// occt-ref: Font_SystemFont
#[derive(Clone, Debug)]
pub struct SystemFont {
    pub family: String,
    pub aspect: FontAspect,
    pub file_path: String,
    pub nb_faces: usize,
}

impl SystemFont {
    pub fn new(family: impl Into<String>, aspect: FontAspect, path: impl Into<String>) -> Self {
        Self { family: family.into(), aspect, file_path: path.into(), nb_faces: 1 }
    }

    pub fn is_bold(&self) -> bool {
        matches!(self.aspect, FontAspect::Bold | FontAspect::BoldItalic)
    }

    pub fn is_italic(&self) -> bool {
        matches!(self.aspect, FontAspect::Italic | FontAspect::BoldItalic)
    }
}

// occt-ref: Font_FontMgr
#[derive(Clone, Debug, Default)]
pub struct FontManager {
    pub fonts: Vec<SystemFont>,
}

impl FontManager {
    pub fn new() -> Self { Self::default() }

    pub fn register_font(&mut self, font: SystemFont) {
        self.fonts.push(font);
    }

    pub fn find(&self, family: &str, aspect: FontAspect) -> Option<&SystemFont> {
        self.fonts.iter().find(|f| f.family == family && f.aspect == aspect)
    }

    pub fn find_family(&self, family: &str) -> Vec<&SystemFont> {
        self.fonts.iter().filter(|f| f.family == family).collect()
    }

    pub fn available_families(&self) -> Vec<&str> {
        let mut fams: Vec<&str> = self.fonts.iter().map(|f| f.family.as_str()).collect();
        fams.sort();
        fams.dedup();
        fams
    }

    pub fn nb_fonts(&self) -> usize { self.fonts.len() }
}

/// A glyph outline represented as 2D contours.
#[derive(Clone, Debug, Default)]
pub struct GlyphContour {
    pub points: Vec<[f64; 2]>,
    pub is_closed: bool,
}

impl GlyphContour {
    pub fn new(points: Vec<[f64; 2]>, closed: bool) -> Self {
        Self { points, is_closed: closed }
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
}

// occt: Font_BRepFont
#[derive(Clone, Debug)]
pub struct BRepFont {
    pub family: String,
    pub aspect: FontAspect,
    pub size: f64,
    pub unit: FontUnit,
    pub fixed_advance: Option<f64>,
}

impl BRepFont {
    pub fn new(family: impl Into<String>, aspect: FontAspect, size: f64) -> Self {
        Self { family: family.into(), aspect, size, unit: FontUnit::Point, fixed_advance: None }
    }

    pub fn with_unit(mut self, u: FontUnit) -> Self { self.unit = u; self }
    pub fn with_fixed_advance(mut self, adv: f64) -> Self { self.fixed_advance = Some(adv); self }

    pub fn size_in_mm(&self) -> f64 { self.size * self.unit.to_mm_factor() }

    pub fn glyph_advance(&self, _char: char) -> f64 {
        self.fixed_advance.unwrap_or(self.size_in_mm() * 0.6)
    }

    pub fn string_width(&self, text: &str) -> f64 {
        text.chars().map(|c| self.glyph_advance(c)).sum()
    }

    pub fn string_height(&self) -> f64 { self.size_in_mm() }

    pub fn ascender(&self) -> f64 { self.size_in_mm() * 0.8 }
    pub fn descender(&self) -> f64 { self.size_in_mm() * 0.2 }
}

/// Text layout result for BRep text builder.
#[derive(Clone, Debug, Default)]
pub struct BRepTextShape {
    pub text: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    pub up: [f64; 3],
    pub width: f64,
    pub height: f64,
    pub nb_glyphs: usize,
}

// occt: Font_BRepTextBuilder
#[derive(Clone, Debug)]
pub struct BRepTextBuilder {
    pub font: BRepFont,
    pub halign: TextHAlign,
    pub valign: TextVAlign,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextHAlign { Left, Center, Right }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextVAlign { Bottom, Center, Top }

impl BRepTextBuilder {
    pub fn new(font: BRepFont) -> Self {
        Self { font, halign: TextHAlign::Left, valign: TextVAlign::Bottom }
    }

    pub fn with_halign(mut self, h: TextHAlign) -> Self { self.halign = h; self }
    pub fn with_valign(mut self, v: TextVAlign) -> Self { self.valign = v; self }

    pub fn build(&self, text: &str, pos: [f64; 3], dir: [f64; 3], up: [f64; 3]) -> BRepTextShape {
        let w = self.font.string_width(text);
        let h = self.font.string_height();
        let offset_x = match self.halign {
            TextHAlign::Left => 0.0,
            TextHAlign::Center => -w * 0.5,
            TextHAlign::Right => -w,
        };
        let position = [pos[0] + dir[0] * offset_x, pos[1] + dir[1] * offset_x, pos[2] + dir[2] * offset_x];
        BRepTextShape {
            text: text.to_string(),
            position,
            direction: dir,
            up,
            width: w,
            height: h,
            nb_glyphs: text.chars().count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_manager_register_find() {
        let mut mgr = FontManager::new();
        mgr.register_font(SystemFont::new("Arial", FontAspect::Regular, "/fonts/arial.ttf"));
        mgr.register_font(SystemFont::new("Arial", FontAspect::Bold, "/fonts/arialbd.ttf"));
        assert_eq!(mgr.nb_fonts(), 2);
        let f = mgr.find("Arial", FontAspect::Bold).unwrap();
        assert!(f.is_bold());
        assert!(!f.is_italic());
    }

    #[test]
    fn brep_font_metrics() {
        let f = BRepFont::new("Arial", FontAspect::Regular, 12.0).with_unit(FontUnit::Mm);
        assert!((f.size_in_mm() - 12.0).abs() < 1e-10);
        assert!(f.string_width("Hello") > 0.0);
    }

    #[test]
    fn brep_text_builder() {
        let font = BRepFont::new("Mono", FontAspect::Regular, 10.0).with_fixed_advance(6.0);
        let builder = BRepTextBuilder::new(font);
        let shape = builder.build("Hi", [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(shape.nb_glyphs, 2);
        assert!((shape.width - 12.0).abs() < 1e-10);
    }

    #[test]
    fn font_unit_factors() {
        assert!((FontUnit::Inch.to_mm_factor() - 25.4).abs() < 1e-10);
    }
}
