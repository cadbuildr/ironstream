// FILE: rust/ironstream/crates/ironstream/src/prs3d_text_aspect.rs

// occt-ref: Graphic3d_AspectText3d
/// Font slant style, mirroring the slant variants exposed by Graphic3d_AspectText3d.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontSlant {
    Regular,
    Italic,
    Oblique,
}

// occt-ref: Graphic3d_AspectText3d
/// Font weight style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Bold,
}

// occt: Prs3d_TextAspect
/// High-level text display attributes used by the Prs3d presentation layer.
/// Corresponds to Prs3d_TextAspect in OCCT.
#[derive(Debug, Clone, PartialEq)]
pub struct TextAspect {
    /// RGB colour in [0.0, 1.0] per channel.
    pub color: [f64; 3],
    /// Font family name (e.g. "Courier").
    pub font: String,
    /// Character height in model units.
    pub height: f64,
    /// Text orientation angle in radians, measured counter-clockwise from the X axis.
    pub angle: f64,
    /// Horizontal alignment: 0 = left, 1 = center, 2 = right.
    pub h_align: u8,
    /// Vertical alignment: 0 = bottom, 1 = center, 2 = top.
    pub v_align: u8,
}

impl TextAspect {
    /// Create a new `TextAspect` with OCCT-compatible defaults:
    /// black colour, "Courier" font, 12 pt height, no rotation, left/bottom alignment.
    pub fn new() -> Self {
        Self {
            color: [0.0, 0.0, 0.0],
            font: String::from("Courier"),
            height: 12.0,
            angle: 0.0,
            h_align: 0,
            v_align: 0,
        }
    }

    /// Return a default `TextAspect` (delegates to [`new`]).
    pub fn default() -> Self {
        Self::new()
    }

    /// Set the RGB display colour.  Each component should be in [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    /// Set the font family name.
    pub fn set_font(&mut self, f: &str) {
        self.font = f.to_string();
    }

    /// Set the character height in model units.
    pub fn set_height(&mut self, h: f64) {
        self.height = h;
    }

    /// Set the text orientation angle in radians.
    pub fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }
}

impl std::default::Default for TextAspect {
    fn default() -> Self {
        TextAspect::new()
    }
}

// occt-ref: Graphic3d_AspectText3d
/// Low-level 3-D text rendering attributes, mirroring Graphic3d_AspectText3d in OCCT.
#[derive(Debug, Clone, PartialEq)]
pub struct AspectText3d {
    /// RGB colour in [0.0, 1.0] per channel.
    pub color: [f64; 3],
    /// Font family name.
    pub font_name: String,
    /// Character height in model units.
    pub height: f64,
    /// Slant (style) of the font.
    pub slant: FontSlant,
    /// Weight of the font.
    pub weight: FontWeight,
}

impl AspectText3d {
    /// Create a new `AspectText3d` with OCCT-compatible defaults:
    /// black colour, "Courier" font, 12 pt, regular slant, normal weight.
    pub fn new() -> Self {
        Self {
            color: [0.0, 0.0, 0.0],
            font_name: String::from("Courier"),
            height: 12.0,
            slant: FontSlant::Regular,
            weight: FontWeight::Normal,
        }
    }

    /// Set the RGB display colour.  Each component should be in [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    /// Set the font family name and character height simultaneously.
    pub fn set_font(&mut self, f: &str, h: f64) {
        self.font_name = f.to_string();
        self.height = h;
    }
}

impl std::default::Default for AspectText3d {
    fn default() -> Self {
        AspectText3d::new()
    }
}
