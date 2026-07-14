// FILE: rust/ironstream/crates/ironstream/src/font_mgr.rs

// occt-ref: Font_FontAspect
/// Style variant of a typeface, mirroring `Font_FontAspect` in OCCT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontAspect {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

// occt-ref: Font_FontMgr // (per-font metadata entry)
/// Metadata record for a single registered font file.
#[derive(Clone, Debug)]
pub struct FontInfo {
    pub name: String,
    pub path: String,
    pub aspect: FontAspect,
}

impl FontInfo {
    /// Create a new `FontInfo` with `Regular` aspect.
    pub fn new(name: &str, path: &str) -> Self {
        FontInfo {
            name: name.to_string(),
            path: path.to_string(),
            aspect: FontAspect::Regular,
        }
    }
}

// occt-ref: Font_FontMgr
/// Registry of available fonts, analogous to `Font_FontMgr` in OCCT.
pub struct FontMgr {
    pub fonts: Vec<FontInfo>,
}

impl FontMgr {
    /// Create an empty font manager.
    pub fn new() -> Self {
        FontMgr { fonts: Vec::new() }
    }

    /// Register a font entry.
    pub fn register_font(&mut self, info: FontInfo) {
        self.fonts.push(info);
    }

    /// Find the first font matching `name` and `aspect`.
    ///
    /// Falls back to the first font with a matching name if an exact aspect
    /// match is not found, mirroring OCCT's fallback behaviour.
    pub fn find_font(&self, name: &str, aspect: FontAspect) -> Option<&FontInfo> {
        // Exact match first.
        if let Some(f) = self
            .fonts
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(name) && f.aspect == aspect)
        {
            return Some(f);
        }
        // Name-only fallback.
        self.fonts
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(name))
    }

    /// Return the deduplicated list of font family names that are registered.
    pub fn available_fonts(&self) -> Vec<&str> {
        let mut seen: Vec<&str> = Vec::new();
        for f in &self.fonts {
            if !seen.iter().any(|s| s.eq_ignore_ascii_case(f.name.as_str())) {
                seen.push(f.name.as_str());
            }
        }
        seen
    }

    /// Return `true` if at least one variant of `name` has been registered.
    pub fn has_font(&self, name: &str) -> bool {
        self.fonts
            .iter()
            .any(|f| f.name.eq_ignore_ascii_case(name))
    }
}

impl Default for FontMgr {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: Font_BRepFont
/// Stub for a font-driven B-Rep glyph renderer, analogous to `Font_BRepFont`
/// in OCCT.  The `render_char` method returns contour loops as sequences of
/// 2-D points; this stub always returns an empty set of contours.
pub struct BRepFont {
    pub font: String,
    pub size: f64,
}

impl BRepFont {
    /// Create a new `BRepFont` bound to `font` at point size `size`.
    pub fn new(font: &str, size: f64) -> Self {
        BRepFont {
            font: font.to_string(),
            size,
        }
    }

    /// Render `c` as a list of closed contours, each contour being an ordered
    /// sequence of 2-D points `[x, y]`.
    ///
    /// This stub always returns an empty contour list; a real implementation
    /// would parse the glyph outlines from the font file and scale them by
    /// `self.size`.
    pub fn render_char(&self, _c: char) -> Vec<Vec<[f64; 2]>> {
        Vec::new()
    }
}
