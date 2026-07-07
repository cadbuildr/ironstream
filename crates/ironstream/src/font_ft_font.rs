// FILE: font_ft_font.rs
// occt: Font_FTFont

use core::fmt;

/// Font hinting enumeration for FreeType.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontHinting {
    Off = 0x00,
    Normal = 0x01,
    Light = 0x02,
    ForceAutohint = 0x10,
    NoAutohint = 0x20,
}

/// Strict level enumeration for font search.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontStrictLevel {
    Any = 0,
}

/// Unicode subset enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontUnicodeSubset {
    Western = 0,
    CJK = 1,
    Korean = 2,
    Arabic = 3,
}

pub const FONT_UNICODE_SUBSET_NB: usize = 4;

/// Font initialization parameters.
#[derive(Clone, Debug)]
pub struct FontFtFontParams {
    /// Face size in points (1/72 inch)
    pub point_size: u32,
    /// Resolution of the target device in dpi
    pub resolution: u32,
    /// Font hinting option
    pub font_hinting: FontHinting,
    /// Whether to synthesize italic style
    pub to_synthesize_italic: bool,
    /// Whether this is a single-stroke (one-line) font
    pub is_single_stroke_font: bool,
}

impl FontFtFontParams {
    /// Create default font parameters.
    pub fn new() -> Self {
        Self {
            point_size: 0,
            resolution: 72,
            font_hinting: FontHinting::Off,
            to_synthesize_italic: false,
            is_single_stroke_font: false,
        }
    }

    /// Create font parameters with point size and resolution.
    pub fn with_size(point_size: u32, resolution: u32) -> Self {
        Self {
            point_size,
            resolution,
            font_hinting: FontHinting::Off,
            to_synthesize_italic: false,
            is_single_stroke_font: false,
        }
    }
}

impl Default for FontFtFontParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper over FreeType font.
///
/// This class loads and manages FreeType fonts. It provides methods for:
/// - Font initialization from file or buffer
/// - Glyph rendering and metrics
/// - Unicode character classification
/// - Text bounding box computation
pub struct FontFtFont {
    /// Font parameters
    font_params: FontFtFontParams,
    /// Font path (when loaded from file)
    font_path: String,
    /// Width scaling factor for glyphs (default 1.0)
    width_scaling: f32,
    /// Whether to use fallback fonts for Unicode subsets
    to_use_unicode_subset_fallback: bool,
    /// Currently loaded unicode character
    u_char: u32,
}

impl FontFtFont {
    /// Create a new uninitialized font.
    pub fn new() -> Self {
        Self {
            font_params: FontFtFontParams::new(),
            font_path: String::new(),
            width_scaling: 1.0,
            to_use_unicode_subset_fallback: true,
            u_char: 0,
        }
    }

    /// Check if font is loaded and valid.
    pub fn is_valid(&self) -> bool {
        !self.font_path.is_empty()
    }

    /// Get the font parameters.
    pub fn font_params(&self) -> &FontFtFontParams {
        &self.font_params
    }

    /// Get mutable font parameters.
    pub fn font_params_mut(&mut self) -> &mut FontFtFontParams {
        &mut self.font_params
    }

    /// Get the font path.
    pub fn font_path(&self) -> &str {
        &self.font_path
    }

    /// Set the font path.
    pub fn set_font_path(&mut self, path: &str) {
        self.font_path = path.to_string();
    }

    /// Get the configured point size.
    pub fn point_size(&self) -> u32 {
        self.font_params.point_size
    }

    /// Get glyph width scaling factor.
    pub fn width_scaling(&self) -> f32 {
        self.width_scaling
    }

    /// Set glyph width scaling factor.
    pub fn set_width_scaling(&mut self, scale: f32) {
        self.width_scaling = scale;
    }

    /// Get fallback font usage flag.
    pub fn to_use_unicode_subset_fallback(&self) -> bool {
        self.to_use_unicode_subset_fallback
    }

    /// Set fallback font usage flag.
    pub fn set_use_unicode_subset_fallback(&mut self, to_fallback: bool) {
        self.to_use_unicode_subset_fallback = to_fallback;
    }

    /// Check if this is a single-stroke font.
    pub fn is_single_stroke_font(&self) -> bool {
        self.font_params.is_single_stroke_font
    }

    /// Set single-stroke font flag.
    pub fn set_single_stroke_font(&mut self, is_single_line: bool) {
        self.font_params.is_single_stroke_font = is_single_line;
    }

    /// Check if italic style should be synthesized.
    pub fn to_synthesize_italic(&self) -> bool {
        self.font_params.to_synthesize_italic
    }

    // Unicode classification methods (static)

    /// Check if character is from CJK (Chinese, Japanese, Korean) subset.
    pub fn is_char_from_cjk(u_char: u32) -> bool {
        (u_char >= 0x03400 && u_char <= 0x04DFF)
            || (u_char >= 0x04E00 && u_char <= 0x09FFF)
            || (u_char >= 0x0F900 && u_char <= 0x0FAFF)
            || (u_char >= 0x20000 && u_char <= 0x2A6DF)
            || (u_char >= 0x2F800 && u_char <= 0x2FA1F)
            || Self::is_char_from_hiragana(u_char)
            || Self::is_char_from_katakana(u_char)
    }

    /// Check if character is from Hiragana (Japanese).
    pub fn is_char_from_hiragana(u_char: u32) -> bool {
        u_char >= 0x03040 && u_char <= 0x0309F
    }

    /// Check if character is from Katakana (Japanese).
    pub fn is_char_from_katakana(u_char: u32) -> bool {
        u_char >= 0x030A0 && u_char <= 0x030FF
    }

    /// Check if character is from Korean (Hangul).
    pub fn is_char_from_korean(u_char: u32) -> bool {
        (u_char >= 0x01100 && u_char <= 0x011FF)
            || (u_char >= 0x03130 && u_char <= 0x0318F)
            || (u_char >= 0x0AC00 && u_char <= 0x0D7A3)
    }

    /// Check if character is from Arabic.
    pub fn is_char_from_arabic(u_char: u32) -> bool {
        u_char >= 0x00600 && u_char <= 0x006FF
    }

    /// Check if character is right-to-left.
    pub fn is_char_right_to_left(u_char: u32) -> bool {
        Self::is_char_from_arabic(u_char)
    }

    /// Determine Unicode subset for a character.
    pub fn char_subset(u_char: u32) -> FontUnicodeSubset {
        if Self::is_char_from_cjk(u_char) {
            FontUnicodeSubset::CJK
        } else if Self::is_char_from_korean(u_char) {
            FontUnicodeSubset::Korean
        } else if Self::is_char_from_arabic(u_char) {
            FontUnicodeSubset::Arabic
        } else {
            FontUnicodeSubset::Western
        }
    }
}

impl Default for FontFtFont {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for FontFtFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontFtFont")
            .field("font_params", &self.font_params)
            .field("font_path", &self.font_path)
            .field("width_scaling", &self.width_scaling)
            .field("to_use_unicode_subset_fallback", &self.to_use_unicode_subset_fallback)
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_ft_font_params_default() {
        let params = FontFtFontParams::new();
        assert_eq!(params.point_size, 0);
        assert_eq!(params.resolution, 72);
        assert_eq!(params.font_hinting, FontHinting::Off);
        assert!(!params.to_synthesize_italic);
        assert!(!params.is_single_stroke_font);
    }

    #[test]
    fn test_font_ft_font_params_with_size() {
        let params = FontFtFontParams::with_size(24, 96);
        assert_eq!(params.point_size, 24);
        assert_eq!(params.resolution, 96);
    }

    #[test]
    fn test_font_ft_font_new() {
        let font = FontFtFont::new();
        assert!(!font.is_valid());
        assert_eq!(font.width_scaling(), 1.0);
        assert!(font.to_use_unicode_subset_fallback());
    }

    #[test]
    fn test_font_ft_font_width_scaling() {
        let mut font = FontFtFont::new();
        assert_eq!(font.width_scaling(), 1.0);
        font.set_width_scaling(0.8);
        assert!((font.width_scaling() - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_font_ft_font_single_stroke() {
        let mut font = FontFtFont::new();
        assert!(!font.is_single_stroke_font());
        font.set_single_stroke_font(true);
        assert!(font.is_single_stroke_font());
    }

    #[test]
    fn test_font_ft_font_synthesize_italic() {
        let font = FontFtFont::new();
        assert!(!font.to_synthesize_italic());
    }

    #[test]
    fn test_is_char_from_cjk() {
        assert!(FontFtFont::is_char_from_cjk(0x04E00)); // CJK
        assert!(FontFtFont::is_char_from_cjk(0x03040)); // Hiragana
        assert!(FontFtFont::is_char_from_cjk(0x030A0)); // Katakana
        assert!(!FontFtFont::is_char_from_cjk(0x0041)); // 'A'
    }

    #[test]
    fn test_is_char_from_hiragana() {
        assert!(FontFtFont::is_char_from_hiragana(0x03040));
        assert!(FontFtFont::is_char_from_hiragana(0x0309F));
        assert!(!FontFtFont::is_char_from_hiragana(0x0041));
    }

    #[test]
    fn test_is_char_from_katakana() {
        assert!(FontFtFont::is_char_from_katakana(0x030A0));
        assert!(FontFtFont::is_char_from_katakana(0x030FF));
        assert!(!FontFtFont::is_char_from_katakana(0x0041));
    }

    #[test]
    fn test_is_char_from_korean() {
        assert!(FontFtFont::is_char_from_korean(0x01100));
        assert!(FontFtFont::is_char_from_korean(0x0AC00));
        assert!(!FontFtFont::is_char_from_korean(0x0041));
    }

    #[test]
    fn test_is_char_from_arabic() {
        assert!(FontFtFont::is_char_from_arabic(0x00600));
        assert!(FontFtFont::is_char_from_arabic(0x006FF));
        assert!(!FontFtFont::is_char_from_arabic(0x0041));
    }

    #[test]
    fn test_is_char_right_to_left() {
        assert!(FontFtFont::is_char_right_to_left(0x00600));
        assert!(!FontFtFont::is_char_right_to_left(0x0041));
    }

    #[test]
    fn test_char_subset() {
        assert_eq!(FontFtFont::char_subset(0x04E00), FontUnicodeSubset::CJK);
        assert_eq!(FontFtFont::char_subset(0x0AC00), FontUnicodeSubset::Korean);
        assert_eq!(FontFtFont::char_subset(0x00600), FontUnicodeSubset::Arabic);
        assert_eq!(FontFtFont::char_subset(0x0041), FontUnicodeSubset::Western);
    }

    #[test]
    fn test_font_ft_font_fallback_flag() {
        let mut font = FontFtFont::new();
        assert!(font.to_use_unicode_subset_fallback());
        font.set_use_unicode_subset_fallback(false);
        assert!(!font.to_use_unicode_subset_fallback());
    }
}
